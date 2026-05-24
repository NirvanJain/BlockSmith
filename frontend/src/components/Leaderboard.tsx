import type { LeaderboardEntry } from "../types";

interface LeaderboardProps {
  entries: LeaderboardEntry[];
  loading: boolean;
}

export default function Leaderboard({ entries, loading }: LeaderboardProps) {
  return (
    <div>
      <div style={{ marginBottom: "24px" }}>
        <div className="content-title">Contributor Leaderboard</div>
        <div className="content-subtitle">
          Ranked by verified reputation score. Updated on each new block appended to chain.
        </div>
      </div>

      {loading ? (
        <div className="empty-state">
          <div className="empty-state-msg cursor-blink">Fetching rankings</div>
        </div>
      ) : entries.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon">◻</div>
          <div className="empty-state-msg">No contributors yet — submit a contribution to appear here</div>
        </div>
      ) : (
        <div className="table-wrapper">
          <table className="bs-table" id="leaderboard-table">
            <thead>
              <tr>
                <th style={{ width: 40 }}>#</th>
                <th>Username</th>
                <th>Reputation</th>
                <th>Verified Contributions</th>
                <th>Total</th>
                <th>Score Bar</th>
              </tr>
            </thead>
            <tbody>
              {entries.map((entry) => {
                const maxScore = entries[0]?.reputation_score || 1;
                const pct = Math.round((entry.reputation_score / maxScore) * 100);
                return (
                  <tr key={entry.github_username} id={`leaderboard-row-${entry.rank}`}>
                    <td className={`rank-cell ${entry.rank <= 3 ? "top" : ""}`}>
                      {entry.rank <= 3 ? "★" : ""}{entry.rank}
                    </td>
                    <td>
                      <span style={{ fontWeight: 600, color: "var(--text-primary)" }}>
                        {entry.github_username}
                      </span>
                    </td>
                    <td>
                      <span className="score-badge">{entry.reputation_score} pts</span>
                    </td>
                    <td>
                      <span className="verified-badge">{entry.verified_contributions}</span>
                    </td>
                    <td style={{ color: "var(--text-dim)", fontSize: "10px" }}>
                      {entry.total_contributions}
                    </td>
                    <td style={{ width: 120 }}>
                      <div className="progress-bar">
                        <div className="progress-fill" style={{ width: `${pct}%` }} />
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
