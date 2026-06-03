import React from 'react';
import type { Activity } from '../types';

const ACTIVITY_LABELS: Record<string, string> = {
  pr_merged: 'PR Merged',
  badge_awarded: 'Badge',
  issue_opened: 'Issue',
  commit_pushed: 'Commit',
};

type Props = {
  items: Activity[];
};

const ActivityFeed: React.FC<Props> = ({ items }) => {
  if (items.length === 0) {
    return <p className="empty-state">No activity yet. Contributions will appear here.</p>;
  }

  return (
    <ul className="activity-list">
      {items.map((item) => (
        <li key={item.id} className="activity-item">
          <div className="activity-actor">
            {item.actor?.avatar_url && (
              <img src={item.actor.avatar_url} alt={item.actor.name} className="avatar-sm" />
            )}
            <span className="actor-name">{item.actor?.name ?? 'Unknown'}</span>
          </div>
          <div className="activity-body">
            <div className="activity-meta">
              <span className="activity-badge">{ACTIVITY_LABELS[item.activity_type] ?? item.activity_type}</span>
              {item.link ? (
                <a href={item.link} target="_blank" rel="noopener noreferrer" className="activity-title">
                  {item.title}
                </a>
              ) : (
                <span className="activity-title">{item.title}</span>
              )}
            </div>
            {item.description && <p className="activity-desc">{item.description}</p>}
          </div>
          <div className="activity-xp">+{item.xp_earned} XP</div>
        </li>
      ))}
    </ul>
  );
};

export default ActivityFeed;
