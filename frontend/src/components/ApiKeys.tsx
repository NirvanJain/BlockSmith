import { useState } from "react";

interface ApiKey {
  id: number;
  label: string;
  key: string;
  created_at: string;
  last_used: string | null;
}

const DEMO_KEYS: ApiKey[] = [
  {
    id: 1,
    label: "Webhook Listener",
    key: "bsk_live_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6",
    created_at: "2026-05-10 14:22:00",
    last_used: "2026-05-24 20:11:44",
  },
];

export default function ApiKeys() {
  const [keys, setKeys] = useState<ApiKey[]>(DEMO_KEYS);
  const [label, setLabel] = useState("");
  const [revealed, setRevealed] = useState<Set<number>>(new Set());

  const generateKey = () => {
    if (!label.trim()) return;
    const chars = "abcdefghijklmnopqrstuvwxyz0123456789";
    const rand = Array.from({ length: 32 }, () =>
      chars[Math.floor(Math.random() * chars.length)]
    ).join("");
    const newKey: ApiKey = {
      id: Date.now(),
      label: label.trim(),
      key: `bsk_live_${rand}`,
      created_at: new Date().toISOString().replace("T", " ").slice(0, 19),
      last_used: null,
    };
    setKeys((k) => [...k, newKey]);
    setLabel("");
  };

  const revealKey = (id: number) => {
    setRevealed((r) => {
      const next = new Set(r);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const revokeKey = (id: number) => {
    setKeys((k) => k.filter((key) => key.id !== id));
  };

  const maskKey = (key: string) =>
    key.slice(0, 12) + "•".repeat(key.length - 16) + key.slice(-4);

  return (
    <div>
      <div style={{ marginBottom: "24px" }}>
        <div className="content-title">API Keys</div>
        <div className="content-subtitle">
          Manage API keys for webhook integrations, external tooling, and CI/CD pipelines.
        </div>
      </div>

      {/* Generate form */}
      <div className="form-panel" style={{ marginBottom: "28px" }}>
        <div
          style={{
            fontSize: "9px",
            textTransform: "uppercase",
            letterSpacing: "0.14em",
            color: "var(--text-dim)",
            marginBottom: "14px",
          }}
        >
          Generate New API Key
        </div>
        <div className="form-row">
          <div className="form-group">
            <label className="form-label" htmlFor="api-key-label">Key Label</label>
            <input
              id="api-key-label"
              className="form-input"
              placeholder="e.g. CI/CD Pipeline, Webhook Listener"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && generateKey()}
            />
          </div>
          <div className="form-group" style={{ justifyContent: "flex-end" }}>
            <button
              id="btn-generate-api-key"
              className="btn btn-primary"
              onClick={generateKey}
              style={{ alignSelf: "flex-end" }}
            >
              Generate Key
            </button>
          </div>
        </div>
        <div
          style={{
            fontSize: "9px",
            color: "var(--text-muted)",
            textTransform: "uppercase",
            letterSpacing: "0.1em",
            marginTop: "8px",
          }}
        >
          ▲ Keys are shown once. Store them securely — they cannot be recovered.
        </div>
      </div>

      {/* Keys list */}
      <div className="section-title">Active Keys ({keys.length})</div>
      {keys.length === 0 ? (
        <div
          style={{
            border: "1px solid var(--border)",
            padding: "48px",
            textAlign: "center",
            fontSize: "10px",
            color: "var(--text-muted)",
            textTransform: "uppercase",
            letterSpacing: "0.12em",
          }}
        >
          No active API keys
        </div>
      ) : (
        <div className="repo-list">
          {keys.map((k) => (
            <div key={k.id} style={{ padding: "16px 20px", borderBottom: "1px solid var(--border)" }}>
              <div
                style={{
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "flex-start",
                  marginBottom: "10px",
                }}
              >
                <div>
                  <div style={{ fontSize: "12px", fontWeight: 600, marginBottom: "4px" }}>
                    {k.label}
                  </div>
                  <div
                    style={{
                      fontSize: "10px",
                      fontFamily: "var(--font-mono)",
                      color: "var(--accent-green)",
                      background: "var(--bg-input)",
                      padding: "6px 10px",
                      border: "1px solid var(--border)",
                      letterSpacing: "0.04em",
                    }}
                  >
                    {revealed.has(k.id) ? k.key : maskKey(k.key)}
                  </div>
                </div>
                <div style={{ display: "flex", gap: "8px", flexShrink: 0, marginLeft: "16px" }}>
                  <button
                    className="btn btn-ghost"
                    style={{ padding: "4px 12px", fontSize: "9px" }}
                    onClick={() => revealKey(k.id)}
                  >
                    {revealed.has(k.id) ? "Hide" : "Reveal"}
                  </button>
                  <button
                    className="btn btn-danger"
                    style={{ padding: "4px 12px", fontSize: "9px" }}
                    onClick={() => revokeKey(k.id)}
                  >
                    Revoke
                  </button>
                </div>
              </div>
              <div
                style={{
                  display: "flex",
                  gap: "24px",
                  fontSize: "9px",
                  color: "var(--text-dim)",
                  textTransform: "uppercase",
                  letterSpacing: "0.08em",
                }}
              >
                <span>Created: {k.created_at}</span>
                <span>Last used: {k.last_used || "Never"}</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
