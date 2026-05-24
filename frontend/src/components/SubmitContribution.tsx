import { useState } from "react";
import type { Block } from "../types";

interface SubmitContributionProps {
  onSubmit: (block: Omit<Block, "index" | "hash" | "previous_hash" | "timestamp">) => Promise<void>;
  loading: boolean;
}

const CONTRIBUTION_TYPES = [
  { value: "", label: "-- SELECT TYPE --" },
  { value: "pull_request", label: "PULL_REQUEST" },
  { value: "commit", label: "COMMIT" },
  { value: "issue", label: "ISSUE" },
];

export default function SubmitContribution({
  onSubmit,
  loading,
}: SubmitContributionProps) {
  const [form, setForm] = useState({
    contributor: "",
    repository: "",
    contribution_type: "",
    contribution_link: "",
  });
  const [errors, setErrors] = useState<Partial<typeof form>>({});

  const validate = () => {
    const e: Partial<typeof form> = {};
    if (!form.contributor.trim()) e.contributor = "Required";
    if (!form.repository.trim()) e.repository = "Required";
    if (!form.contribution_type) e.contribution_type = "Required";
    if (!form.contribution_link.trim()) e.contribution_link = "Required";
    setErrors(e);
    return Object.keys(e).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;
    await onSubmit(form);
    setForm({ contributor: "", repository: "", contribution_type: "", contribution_link: "" });
    setErrors({});
  };

  const field = (key: keyof typeof form, val: string) =>
    setForm((f) => ({ ...f, [key]: val }));

  return (
    <div>
      <div className="content-header" style={{ background: "transparent", borderBottom: "none", padding: 0, marginBottom: "28px" }}>
        <div className="content-title">Submit Contribution</div>
        <div className="content-subtitle">
          Record a verified GitHub contribution to the chain. <br />
          Each submission becomes an immutable block in the contribution history.
        </div>
      </div>

      <form onSubmit={handleSubmit} id="submit-contribution-form">
        <div className="form-panel">
          <div className="form-row">
            <div className="form-group">
              <label className="form-label" htmlFor="input-contributor">
                GitHub Username
              </label>
              <input
                id="input-contributor"
                className="form-input"
                type="text"
                placeholder="e.g. nirvanjain"
                value={form.contributor}
                onChange={(e) => field("contributor", e.target.value)}
                autoComplete="off"
              />
              {errors.contributor && (
                <span style={{ fontSize: "9px", color: "var(--accent-red)", textTransform: "uppercase", letterSpacing: "0.08em" }}>
                  ▲ {errors.contributor}
                </span>
              )}
            </div>

            <div className="form-group">
              <label className="form-label" htmlFor="input-repository">
                Repository
              </label>
              <input
                id="input-repository"
                className="form-input"
                type="text"
                placeholder="e.g. owner/repo-name"
                value={form.repository}
                onChange={(e) => field("repository", e.target.value)}
                autoComplete="off"
              />
              {errors.repository && (
                <span style={{ fontSize: "9px", color: "var(--accent-red)", textTransform: "uppercase", letterSpacing: "0.08em" }}>
                  ▲ {errors.repository}
                </span>
              )}
            </div>
          </div>

          <div className="form-row">
            <div className="form-group">
              <label className="form-label" htmlFor="input-type">
                Contribution Type
              </label>
              <select
                id="input-type"
                className="form-select"
                value={form.contribution_type}
                onChange={(e) => field("contribution_type", e.target.value)}
              >
                {CONTRIBUTION_TYPES.map((t) => (
                  <option key={t.value} value={t.value}>
                    {t.label}
                  </option>
                ))}
              </select>
              {errors.contribution_type && (
                <span style={{ fontSize: "9px", color: "var(--accent-red)", textTransform: "uppercase", letterSpacing: "0.08em" }}>
                  ▲ {errors.contribution_type}
                </span>
              )}
            </div>

            <div className="form-group">
              <label className="form-label" htmlFor="input-link">
                Contribution URL
              </label>
              <input
                id="input-link"
                className="form-input"
                type="url"
                placeholder="e.g. https://github.com/owner/repo/pull/42"
                value={form.contribution_link}
                onChange={(e) => field("contribution_link", e.target.value)}
                autoComplete="off"
              />
              {errors.contribution_link && (
                <span style={{ fontSize: "9px", color: "var(--accent-red)", textTransform: "uppercase", letterSpacing: "0.08em" }}>
                  ▲ {errors.contribution_link}
                </span>
              )}
            </div>
          </div>

          {/* Reputation preview */}
          {form.contribution_type && (
            <div
              style={{
                padding: "12px 16px",
                border: "1px solid var(--border)",
                background: "var(--bg-input)",
                marginBottom: "20px",
                display: "flex",
                gap: "32px",
                fontSize: "10px",
                textTransform: "uppercase",
                letterSpacing: "0.1em",
              }}
            >
              <span style={{ color: "var(--text-dim)" }}>Reputation Reward</span>
              <span style={{ color: "var(--accent-green)", fontWeight: 700 }}>
                +{form.contribution_type === "pull_request" ? 10 : form.contribution_type === "issue" ? 5 : form.contribution_type === "commit" ? 3 : 1} pts
              </span>
              <span style={{ color: "var(--text-dim)" }}>Block Type</span>
              <span style={{ color: "var(--accent-blue)" }}>
                {form.contribution_type.toUpperCase()}
              </span>
            </div>
          )}

          <div style={{ display: "flex", gap: "12px" }}>
            <button
              id="btn-submit-contribution"
              type="submit"
              className="btn btn-primary"
              disabled={loading}
            >
              {loading ? (
                <>
                  <span className="spinner" />
                  Hashing Block...
                </>
              ) : (
                "Append to Chain"
              )}
            </button>
            <button
              type="button"
              className="btn btn-ghost"
              onClick={() => {
                setForm({ contributor: "", repository: "", contribution_type: "", contribution_link: "" });
                setErrors({});
              }}
            >
              Clear
            </button>
          </div>
        </div>
      </form>

      {/* Info panel */}
      <div
        style={{
          padding: "16px 20px",
          border: "1px solid var(--border)",
          background: "var(--bg-card)",
          fontSize: "10px",
          color: "var(--text-dim)",
          textTransform: "uppercase",
          letterSpacing: "0.1em",
          lineHeight: "2",
        }}
      >
        <div style={{ color: "var(--text-primary)", marginBottom: "8px", fontSize: "9px" }}>
          How block hashing works
        </div>
        SHA256( index | timestamp | contributor | repository | type | link | prev_hash ) → new block hash
      </div>
    </div>
  );
}
