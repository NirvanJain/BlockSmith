import type { Block, LeaderboardEntry, LogEntry } from "../types";

interface DashboardProps {
  blocks: Block[];
  leaderboard: LeaderboardEntry[];
  chainValid: boolean | null;
  logs: LogEntry[];
  onNavigate: (view: string) => void;
}

const fmt = (ts: string) => {
  try {
    return new Date(ts).toISOString().replace("T", " ").slice(0, 16);
  } catch {
    return ts;
  }
};

export default function Dashboard({
  blocks,
  leaderboard,
  chainValid,
  logs,
  onNavigate,
}: DashboardProps) {
  // Compute stats
  const totalPts = leaderboard.reduce((s, e) => s + e.reputation_score, 0);
  const verifiedCount = leaderboard.reduce((s, e) => s + e.verified_contributions, 0);
  const prCount = blocks.filter((b) => b.contribution_type === "pull_request").length;

  const recentBlocks = [...blocks].reverse().slice(0, 5);

  return (
    <div>
      <div style={{ marginBottom: "28px" }}>
        <div className="content-title cursor-blink">System Dashboard</div>
        <div className="content-subtitle">
          Live overview of contribution chain state, reputation scores, and activity log.
        </div>
      </div>

      {/* Stats */}
      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-label">Blocks Indexed</div>
          <div className="stat-value">{blocks.length}</div>
          <div className="stat-delta">↑ Chain Growing</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Verified Contributions</div>
          <div className="stat-value">{verifiedCount}</div>
          <div className="stat-delta">↑ On-Chain Verified</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Total Reputation</div>
          <div className="stat-value">{totalPts}</div>
          <div className="stat-delta">Across {leaderboard.length} contributors</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Chain Integrity</div>
          <div
            className="stat-value"
            style={{
              fontSize: "16px",
              color:
                chainValid === null
                  ? "var(--text-dim)"
                  : chainValid
                  ? "var(--accent-green)"
                  : "var(--accent-red)",
              display: "flex",
              alignItems: "center",
              gap: "8px",
            }}
          >
            {chainValid === null ? "CHECKING..." : chainValid ? "VALID" : "TAMPERED"}
          </div>
          <div className="stat-delta" style={{ color: prCount > 0 ? "var(--accent-green)" : "var(--text-dim)" }}>
            {prCount} pull request{prCount !== 1 ? "s" : ""} merged
          </div>
        </div>
      </div>

      {/* Two column: recent blocks + leaderboard preview */}
      <div className="two-col" style={{ marginBottom: "28px" }}>
        {/* Recent blocks */}
        <div>
          <div className="section-title">Recent Blocks</div>
          {recentBlocks.length === 0 ? (
            <div
              style={{
                border: "1px solid var(--border)",
                padding: "32px",
                textAlign: "center",
                fontSize: "10px",
                color: "var(--text-muted)",
                textTransform: "uppercase",
                letterSpacing: "0.12em",
              }}
            >
              No blocks yet
            </div>
          ) : (
            <div style={{ border: "1px solid var(--border)" }}>
              {recentBlocks.map((block, i) => (
                <div
                  key={block.hash + i}
                  style={{
                    padding: "12px 16px",
                    borderBottom: i < recentBlocks.length - 1 ? "1px solid var(--border)" : "none",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "space-between",
                    cursor: "pointer",
                    transition: "background 0.1s",
                  }}
                  onClick={() => onNavigate("chain-explorer")}
                  onMouseEnter={(e) =>
                    ((e.currentTarget as HTMLElement).style.background =
                      "rgba(255,255,255,0.015)")
                  }
                  onMouseLeave={(e) =>
                    ((e.currentTarget as HTMLElement).style.background = "transparent")
                  }
                >
                  <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <span
                      style={{
                        fontSize: "9px",
                        color: "var(--accent-green)",
                        fontWeight: 700,
                      }}
                    >
                      #{String(block.index).padStart(4, "0")}
                    </span>
                    <span style={{ fontSize: "11px" }}>{block.contributor || "—"}</span>
                    <span
                      className={`block-type-tag ${
                        block.contribution_type === "pull_request"
                          ? "tag-pr"
                          : block.contribution_type === "commit"
                          ? "tag-commit"
                          : block.contribution_type === "issue"
                          ? "tag-issue"
                          : "tag-genesis"
                      }`}
                      style={{ fontSize: "8px" }}
                    >
                      {block.contribution_type.toUpperCase()}
                    </span>
                  </div>
                  <span style={{ fontSize: "9px", color: "var(--text-dim)" }}>
                    {fmt(block.timestamp)}
                  </span>
                </div>
              ))}
            </div>
          )}
          <button
            id="btn-view-all-blocks"
            className="btn btn-ghost"
            style={{ marginTop: "10px", width: "100%" }}
            onClick={() => onNavigate("chain-explorer")}
          >
            View Full Chain →
          </button>
        </div>

        {/* Top contributors */}
        <div>
          <div className="section-title">Top Contributors</div>
          {leaderboard.length === 0 ? (
            <div
              style={{
                border: "1px solid var(--border)",
                padding: "32px",
                textAlign: "center",
                fontSize: "10px",
                color: "var(--text-muted)",
                textTransform: "uppercase",
                letterSpacing: "0.12em",
              }}
            >
              No contributors yet
            </div>
          ) : (
            <div style={{ border: "1px solid var(--border)" }}>
              {leaderboard.slice(0, 5).map((entry, i) => {
                const maxScore = leaderboard[0]?.reputation_score || 1;
                const pct = Math.round((entry.reputation_score / maxScore) * 100);
                return (
                  <div
                    key={entry.github_username}
                    style={{
                      padding: "12px 16px",
                      borderBottom: i < Math.min(leaderboard.length, 5) - 1 ? "1px solid var(--border)" : "none",
                    }}
                  >
                    <div
                      style={{
                        display: "flex",
                        justifyContent: "space-between",
                        alignItems: "center",
                        marginBottom: "6px",
                      }}
                    >
                      <div style={{ display: "flex", gap: "10px", alignItems: "center" }}>
                        <span
                          style={{
                            fontSize: "9px",
                            color: i < 3 ? "var(--accent-amber)" : "var(--text-dim)",
                            fontWeight: i < 3 ? 700 : 400,
                          }}
                        >
                          {i < 3 ? "★" : "·"} {i + 1}
                        </span>
                        <span style={{ fontSize: "11px" }}>{entry.github_username}</span>
                      </div>
                      <span className="score-badge">{entry.reputation_score} pts</span>
                    </div>
                    <div className="progress-bar">
                      <div className="progress-fill" style={{ width: `${pct}%` }} />
                    </div>
                  </div>
                );
              })}
            </div>
          )}
          <button
            id="btn-view-leaderboard"
            className="btn btn-ghost"
            style={{ marginTop: "10px", width: "100%" }}
            onClick={() => onNavigate("leaderboard")}
          >
            Full Leaderboard →
          </button>
        </div>
      </div>

      {/* Activity log */}
      <div className="section-title" style={{ marginBottom: "14px" }}>Activity Log</div>
      <div className="terminal-log">
        <div className="terminal-log-header">
          <span className="status-dot" />
          stdout — blocksmith-node-01
        </div>
        <div className="terminal-log-body">
          {logs.length === 0 ? (
            <div className="log-entry">
              <span className="log-time">--:--:--</span>
              <span className="log-prefix info">[INFO]</span>
              <span className="log-msg">Waiting for events...</span>
            </div>
          ) : (
            logs.map((log) => (
              <div key={log.id} className="log-entry">
                <span className="log-time">{log.time}</span>
                <span className={`log-prefix ${log.level}`}>
                  [{log.level.toUpperCase()}]
                </span>
                <span className="log-msg">{log.message}</span>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
