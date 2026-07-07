//! Citizen registry — usernames, identity roots, rolling UID seeds, and the
//! creation time of each citizen's current login password.
//!
//! Backed by Postgres when `DATABASE_URL` is reachable; otherwise an in-memory
//! map so the API still runs locally without a database.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

#[derive(Clone)]
pub struct Citizen {
    pub id: u16,
    pub username: String,
    pub root: String, // identity_root fingerprint (hex)
    #[allow(dead_code)]
    pub phone_masked: String,
    pub pwd_created: i64,     // unix seconds of the active password
    pub login_count: i64,     // length of the seed chain
    pub current_seed: String, // current Citizen UID (hex)
}

pub(crate) struct MemState {
    next_id: u16,
    by_name: HashMap<String, Citizen>,
}

pub enum Registry {
    Postgres(Db),
    Memory(Mutex<MemState>),
}

const CREATE_TABLE: &str = "\
CREATE TABLE IF NOT EXISTS citizens (\
    id            BIGSERIAL,\
    username      TEXT PRIMARY KEY,\
    root          TEXT NOT NULL,\
    phone_masked  TEXT NOT NULL DEFAULT '',\
    pwd_created   BIGINT NOT NULL DEFAULT 0,\
    login_count   BIGINT NOT NULL DEFAULT 0,\
    current_seed  TEXT NOT NULL DEFAULT '',\
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()\
)";

type Row = (i64, String, String, String, i64, i64, String);
fn to_citizen(r: Row) -> Citizen {
    Citizen {
        id: r.0 as u16,
        username: r.1,
        root: r.2,
        phone_masked: r.3,
        pwd_created: r.4,
        login_count: r.5,
        current_seed: r.6,
    }
}
const SELECT: &str =
    "SELECT id, username, root, phone_masked, pwd_created, login_count, current_seed FROM citizens";

impl Registry {
    pub async fn init(database_url: Option<String>) -> Arc<Registry> {
        let Some(url) = database_url.filter(|u| !u.is_empty()) else {
            println!("janmat-api: no DATABASE_URL — using in-memory citizen registry");
            return Arc::new(Registry::mem());
        };
        for attempt in 1..=10u32 {
            match PgPoolOptions::new()
                .max_connections(5)
                .acquire_timeout(Duration::from_secs(3))
                .connect(&url)
                .await
            {
                Ok(pool) => match sqlx::query(CREATE_TABLE).execute(&pool).await {
                    Ok(_) => {
                        println!("janmat-api: connected to Postgres citizen registry");
                        return Arc::new(Registry::Postgres(pool));
                    }
                    Err(e) => eprintln!("janmat-api: migrate failed: {e}"),
                },
                Err(e) => eprintln!("janmat-api: db connect attempt {attempt}/10 failed: {e}"),
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        eprintln!("janmat-api: database unreachable — using in-memory registry");
        Arc::new(Registry::mem())
    }

    fn mem() -> Registry {
        Registry::Memory(Mutex::new(MemState {
            next_id: 1,
            by_name: HashMap::new(),
        }))
    }

    /// Insert a new citizen. Returns Some(id) if created, None if username taken.
    pub async fn register(
        &self,
        username: &str,
        root: &str,
        phone_masked: &str,
        pwd_created: i64,
        seed0: &str,
    ) -> Result<Option<u16>, sqlx::Error> {
        match self {
            Registry::Postgres(pool) => {
                let row: Option<(i64,)> = sqlx::query_as(
                    "INSERT INTO citizens (username, root, phone_masked, pwd_created, login_count, current_seed) \
                     VALUES ($1,$2,$3,$4,0,$5) ON CONFLICT (username) DO NOTHING RETURNING id",
                )
                .bind(username)
                .bind(root)
                .bind(phone_masked)
                .bind(pwd_created)
                .bind(seed0)
                .fetch_optional(pool)
                .await?;
                Ok(row.map(|(id,)| id as u16))
            }
            Registry::Memory(m) => {
                let mut g = m.lock().unwrap();
                if g.by_name.contains_key(username) {
                    return Ok(None);
                }
                let id = g.next_id;
                g.next_id = g.next_id.wrapping_add(1);
                g.by_name.insert(
                    username.to_string(),
                    Citizen {
                        id,
                        username: username.to_string(),
                        root: root.to_string(),
                        phone_masked: phone_masked.to_string(),
                        pwd_created,
                        login_count: 0,
                        current_seed: seed0.to_string(),
                    },
                );
                Ok(Some(id))
            }
        }
    }

    pub async fn by_username(&self, username: &str) -> Result<Option<Citizen>, sqlx::Error> {
        match self {
            Registry::Postgres(pool) => {
                let row: Option<Row> = sqlx::query_as(&format!("{SELECT} WHERE username = $1"))
                    .bind(username)
                    .fetch_optional(pool)
                    .await?;
                Ok(row.map(to_citizen))
            }
            Registry::Memory(m) => Ok(m.lock().unwrap().by_name.get(username).cloned()),
        }
    }

    pub async fn by_id(&self, id: u16) -> Result<Option<Citizen>, sqlx::Error> {
        match self {
            Registry::Postgres(pool) => {
                let row: Option<Row> = sqlx::query_as(&format!("{SELECT} WHERE id = $1"))
                    .bind(id as i64)
                    .fetch_optional(pool)
                    .await?;
                Ok(row.map(to_citizen))
            }
            Registry::Memory(m) => Ok(m
                .lock()
                .unwrap()
                .by_name
                .values()
                .find(|c| c.id == id)
                .cloned()),
        }
    }

    pub async fn is_taken(&self, username: &str) -> Result<bool, sqlx::Error> {
        Ok(self.by_username(username).await?.is_some())
    }

    /// Rotate the seed chain on a successful login.
    pub async fn rotate_seed(
        &self,
        username: &str,
        login_count: i64,
        current_seed: &str,
    ) -> Result<(), sqlx::Error> {
        match self {
            Registry::Postgres(pool) => {
                sqlx::query(
                    "UPDATE citizens SET login_count = $2, current_seed = $3 WHERE username = $1",
                )
                .bind(username)
                .bind(login_count)
                .bind(current_seed)
                .execute(pool)
                .await?;
                Ok(())
            }
            Registry::Memory(m) => {
                if let Some(c) = m.lock().unwrap().by_name.get_mut(username) {
                    c.login_count = login_count;
                    c.current_seed = current_seed.to_string();
                }
                Ok(())
            }
        }
    }

    /// Set a new active password creation time (regeneration after expiry).
    pub async fn set_pwd_created(
        &self,
        username: &str,
        pwd_created: i64,
    ) -> Result<(), sqlx::Error> {
        match self {
            Registry::Postgres(pool) => {
                sqlx::query("UPDATE citizens SET pwd_created = $2 WHERE username = $1")
                    .bind(username)
                    .bind(pwd_created)
                    .execute(pool)
                    .await?;
                Ok(())
            }
            Registry::Memory(m) => {
                if let Some(c) = m.lock().unwrap().by_name.get_mut(username) {
                    c.pwd_created = pwd_created;
                }
                Ok(())
            }
        }
    }
}
