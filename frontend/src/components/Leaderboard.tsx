import React from 'react';
import type { LeaderboardEntry } from '../types';

type Props = {
  entries: LeaderboardEntry[];
};

const RANK_MEDALS = ['🥇', '🥈', '🥉'];

const Leaderboard: React.FC<Props> = ({ entries }) => {
  if (entries.length === 0) {
    return <p className="empty-state">No leaderboard data yet.</p>;
  }

  return (
    <table className="leaderboard-table">
      <thead>
        <tr>
          <th>#</th>
          <th>User</th>
          <th>Level</th>
          <th>XP</th>
          <th>Reputation</th>
        </tr>
      </thead>
      <tbody>
        {entries.map((e) => (
          <tr key={e.id} className="leaderboard-row">
            <td className="rank-cell">
              {e.rank <= 3 ? RANK_MEDALS[e.rank - 1] : e.rank}
            </td>
            <td className="user-cell">
              <img src={e.avatar_url} alt={e.name} className="avatar-sm" />
              <div>
                <span className="user-name">{e.name}</span>
                <span className="user-handle">@{e.github_username}</span>
              </div>
            </td>
            <td>Lvl {e.level}</td>
            <td>{e.xp.toLocaleString()}</td>
            <td>{e.reputation_score.toLocaleString()}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default Leaderboard;
