import { useEffect, useState } from "react";
import { api, type UsernameAvailability } from "../api/client";
import { Avatar, Button, Icon, VerifiedBadge } from "../components/ui";
import { fmtRemaining, useNow } from "../lib/time";
import { useCivic } from "../store/store";

type Mode = "login" | "register" | "forgot";

export function Identity() {
  const { state, register, login, regeneratePassword, logout } = useCivic();
  const { identity } = state;
  const now = useNow(1000);

  const [mode, setMode] = useState<Mode>("login");
  const [copied, setCopied] = useState(false);

  // login
  const [pw, setPw] = useState("");
  const [loginBusy, setLoginBusy] = useState(false);
  const [loginErr, setLoginErr] = useState<string | null>(null);

  // register / forgot shared fields
  const [username, setUsername] = useState("");
  const [pan, setPan] = useState("");
  const [aadhaar, setAadhaar] = useState("");
  const [dob, setDob] = useState("");
  const [phone, setPhone] = useState("");
  const [avail, setAvail] = useState<UsernameAvailability | null>(null);
  const [checking, setChecking] = useState(false);
  const [busy, setBusy] = useState(false);
  const [fieldErr, setFieldErr] = useState<{ field?: string; msg?: string }>({});

  useEffect(() => {
    setAvail(null);
    if (mode !== "register" || username.trim().length < 3) return;
    setChecking(true);
    const t = setTimeout(async () => {
      try {
        setAvail(await api.usernameAvailable(username.trim()));
      } catch {
        setAvail(null);
      } finally {
        setChecking(false);
      }
    }, 400);
    return () => clearTimeout(t);
  }, [username, mode]);

  const copy = (text: string) => {
    void navigator.clipboard?.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  // ── Logged in: profile ──
  if (identity) {
    const remaining = identity.expiresAt - now;
    return (
      <div className="id-view">
        <h1>Your identity</h1>
        <div className="card id-card">
          <div className="id-top">
            <Avatar seed={identity.handle} size={56} />
            <div>
              <div className="id-name">
                @{identity.username} <VerifiedBadge verified={identity.verified} />
              </div>
              <div className="muted small">
                {identity.country} · phone {identity.phoneMasked ?? "—"} · logins{" "}
                {identity.loginCount}
              </div>
            </div>
          </div>

          <div className="pw-panel">
            <div className="rail-label">Your login password</div>
            <div className="pw-row">
              <code className="pw-value">{identity.password}</code>
              <button className="iconbtn" title="Copy" onClick={() => copy(identity.password)}>
                <Icon name="copy" size={16} /> {copied ? "copied" : "copy"}
              </button>
            </div>
            <div className={`pw-expiry ${remaining < 3600000 ? "low" : ""}`}>
              <Icon name="clock" size={14} /> valid for {fmtRemaining(remaining)} — then
              auto sign-out
            </div>
            <p className="muted small">
              Save this. It's the only way to log back in until it expires. You
              can log in unlimited times while it's valid.
            </p>
          </div>

          <div className="uid-box">
            <span className="rail-label">Citizen UID (rotates every login)</span>
            <code className="uid-value">{identity.uid}</code>
          </div>

          <Button variant="ghost" onClick={logout}>
            Sign out
          </Button>
        </div>
      </div>
    );
  }

  // ── Logged out ──
  const canRegister =
    !!username && !!pan && !!aadhaar && !!dob && !!phone && avail?.available !== false;

  async function doLogin() {
    if (pw.trim().length < 8) return;
    setLoginBusy(true);
    setLoginErr(null);
    const r = await login(pw.trim());
    setLoginBusy(false);
    if (!r.ok) setLoginErr(r.error ?? "Login failed");
  }
  async function doRegister() {
    setBusy(true);
    setFieldErr({});
    const r = await register({ username, pan, aadhaar, dob, phone });
    setBusy(false);
    if (!r.ok) setFieldErr({ field: r.field, msg: r.error });
  }
  async function doForgot() {
    setBusy(true);
    setFieldErr({});
    const r = await regeneratePassword({ username, pan, aadhaar, dob });
    setBusy(false);
    if (!r.ok) setFieldErr({ field: r.field, msg: r.error });
  }
  const errFor = (f: string) =>
    fieldErr.field === f ? <div className="inline-err">{fieldErr.msg}</div> : null;

  return (
    <div className="id-view">
      <h1>Join JanMat</h1>

      <div className="mode-tabs">
        {(["login", "register", "forgot"] as Mode[]).map((m) => (
          <button
            key={m}
            className={`tab ${mode === m ? "on" : ""}`}
            onClick={() => setMode(m)}
          >
            {m === "login" ? "Log in" : m === "register" ? "Register" : "Forgot password"}
          </button>
        ))}
      </div>

      {mode === "login" && (
        <div className="card onboard-card">
          <h3>Log in with your password</h3>
          <p className="muted small">
            Enter your 16-character password. It expires 24 hours after it was
            created — after that, use “Forgot password” to make a new one.
          </p>
          <label className="field">
            <span>Password</span>
            <input
              value={pw}
              placeholder="16-character password"
              maxLength={16}
              autoComplete="off"
              onChange={(e) => setPw(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && void doLogin()}
              style={{ fontFamily: "var(--mono)" }}
            />
          </label>
          {loginErr && <div className="inline-err">{loginErr}</div>}
          <Button loading={loginBusy} disabled={pw.trim().length < 8} onClick={doLogin}>
            Log in
          </Button>
        </div>
      )}

      {mode === "register" && (
        <div className="card onboard-card">
          <h3>Create your citizen ID</h3>
          <p className="muted small">
            No verification for this POC — your PAN, Aadhaar, DOB and phone are
            hashed into a unique ID and a 24-hour password. Pick a username.
          </p>
          <label className="field">
            <span>Username</span>
            <input value={username} placeholder="e.g. rohan_k" onChange={(e) => setUsername(e.target.value)} />
            {username.trim().length >= 3 && (
              <small className={checking ? "muted" : avail?.available ? "ok-text" : "inline-err"}>
                {checking ? "checking…" : avail?.available ? "✓ available" : `✗ ${avail?.reason ?? "unavailable"}`}
              </small>
            )}
            {errFor("username")}
          </label>
          <label className="field">
            <span>PAN</span>
            <input value={pan} placeholder="ABCDE1234F" maxLength={10} onChange={(e) => setPan(e.target.value.toUpperCase())} />
            {errFor("pan")}
          </label>
          <label className="field">
            <span>Aadhaar number</span>
            <input value={aadhaar} placeholder="12 digits" inputMode="numeric" onChange={(e) => setAadhaar(e.target.value)} />
            {errFor("aadhaar")}
          </label>
          <div className="field-row">
            <label className="field">
              <span>Date of birth</span>
              <input type="date" value={dob} onChange={(e) => setDob(e.target.value)} />
              {errFor("dob")}
            </label>
            <label className="field">
              <span>Phone</span>
              <input value={phone} placeholder="10-digit mobile" inputMode="tel" onChange={(e) => setPhone(e.target.value)} />
              {errFor("phone")}
            </label>
          </div>
          <Button loading={busy} disabled={!canRegister} onClick={doRegister}>
            Create my citizen ID
          </Button>
        </div>
      )}

      {mode === "forgot" && (
        <div className="card onboard-card">
          <h3>Generate a new password</h3>
          <p className="muted small">
            Re-enter your exact details to mint a fresh 24-hour password. This
            only works once your current password has expired (no early reset).
          </p>
          <label className="field">
            <span>Username</span>
            <input value={username} placeholder="your username" onChange={(e) => setUsername(e.target.value)} />
            {errFor("username")}
          </label>
          <label className="field">
            <span>PAN</span>
            <input value={pan} placeholder="ABCDE1234F" maxLength={10} onChange={(e) => setPan(e.target.value.toUpperCase())} />
            {errFor("pan")}
          </label>
          <label className="field">
            <span>Aadhaar number</span>
            <input value={aadhaar} placeholder="12 digits" inputMode="numeric" onChange={(e) => setAadhaar(e.target.value)} />
            {errFor("aadhaar")}
          </label>
          <label className="field">
            <span>Date of birth</span>
            <input type="date" value={dob} onChange={(e) => setDob(e.target.value)} />
            {errFor("dob")}
          </label>
          {errFor("active")}
          {errFor("info")}
          <Button loading={busy} disabled={!username || !pan || !aadhaar || !dob} onClick={doForgot}>
            Generate new password
          </Button>
        </div>
      )}
    </div>
  );
}
