import React from 'react';
import type { DiscoveryIssue } from '../types';

type Props = {
  issues: DiscoveryIssue[];
};

const Discovery: React.FC<Props> = ({ issues }) => {
  if (issues.length === 0) {
    return <p className="empty-state">No issues to discover right now.</p>;
  }

  return (
    <ul className="discovery-list">
      {issues.map((issue) => (
        <li key={issue.id} className="discovery-item">
          <div className="discovery-header">
            <span className="discovery-repo">
              {issue.repo.owner}/{issue.repo.name} #{issue.number}
            </span>
            <span className="match-score" title="AI match score">
              {issue.ai_match_score}% match
            </span>
          </div>
          <a
            href={`https://github.com/${issue.repo.owner}/${issue.repo.name}/issues/${issue.number}`}
            target="_blank"
            rel="noopener noreferrer"
            className="discovery-title"
          >
            {issue.title}
          </a>
          <div className="discovery-labels">
            {issue.labels.map((label) => (
              <span key={label} className="label-chip">{label}</span>
            ))}
          </div>
          <p className="discovery-analysis">{issue.ai_analysis}</p>
          <div className="discovery-footer">
            <span className="complexity-badge">
              Complexity: {issue.ai_complexity_score}/10
            </span>
            <span className="creator">by @{issue.creator_username}</span>
          </div>
        </li>
      ))}
    </ul>
  );
};

export default Discovery;
