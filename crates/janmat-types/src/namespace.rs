use serde::{Deserialize, Serialize};
use ulid::Ulid;

macro_rules! typed_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $name(pub Ulid);

        impl $name {
            pub fn new() -> Self {
                Self(Ulid::new())
            }

            pub fn as_ulid(&self) -> &Ulid {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}/{}", $prefix, self.0)
            }
        }
    };
}

typed_id!(TopicId, "t");
typed_id!(SubtopicId, "st");
typed_id!(OpinionId, "o");
typed_id!(VoteId, "v");
