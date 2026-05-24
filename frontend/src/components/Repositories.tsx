import { useState } from "react";
import type { RepoEntry } from "../types";

interface RepositoriesProps {
  repos: RepoEntry[];
  onAdd: (owner: string, repo: string) => void;
}

const STATUS_COLORS: Record<RepoEntry["status"], string> = {
  active: "var(--accent-green)",
  syncing: "var(--accent-amber)",
  paused: "var(--text-muted)",
};

export default function Repositories({ repos, onAdd }: RepositoriesProps) {
  const [owner, setOwner] = useState("");
  const [repoName, setRepoName] = useState("");

  const handleAdd = () => {
    if (!owner.trim() || !repoName.trim()) return;
    onAdd(owner.trim(), repoName.trim());
    setOwner("");
    setRepoName("");
  };

  return (
    <div>
      <div style={{ marginBottom: "24px" }}>
        <div className="content-title">Tracked Repositories</div>
        <div className="content-subtitle">
          Connect GitHub repositories to receive webhook events and auto-verify contributions.
        </div>
      </div>

      {/* Connect form */}
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
          Connect Repository
        </div>
        <div className="form-row">
          <div className="form-group">
            <label className="form-label" htmlFor="repo-owner">Owner</label>
            <input
              id="repo-owner"
              className="form-input"
              placeholder="e.g. nirvanjain"
              value={owner}
              onChange={(e) => setOwner(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label className="form-label" htmlFor="repo-name">Repository Name</label>
            <input
              id="repo-name"
              className="form-input"
              placeholder="e.g. BlockSmith"
              value={repoName}
              onChange={(e) => setRepoName(e.target.value)}
            />
          </div>
        </div>
        <button id="btn-connect-repo" className="btn btn-primary" onClick={handleAdd}>
          Connect Repo
        </button>
      </div>

      {/* Repo list */}
      <div className="section-title">Active Repositories</div>
      {repos.length === 0 ? (
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
          No repositories connected
        </div>
      ) : (
        <div className="repo-list">
          {repos.map((r) => (
            <div key={`${r.owner}/${r.name}`} className="repo-item">
              <div className="repo-name">
                <div
                  className="repo-dot"
                  style={{ background: STATUS_COLORS[r.status] }}
                />
                <span style={{ color: "var(--text-dim)" }}>{r.owner}/</span>
                <span>{r.name}</span>
              </div>
              <div className="repo-meta">
                <span
                  style={{
                    color: STATUS_COLORS[r.status],
                  }}
                >
                  {r.status.toUpperCase()}
                </span>
                <span>{r.contributions} contributions</span>
                <span>Since {r.tracked_since}</span>
                <button className="btn btn-ghost" style={{ padding: "4px 12px", fontSize: "9px" }}>
                  Sync
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
