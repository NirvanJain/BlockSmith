import type { View } from "../types";
import { useAuth } from "../hooks/useAuth";

interface SidebarProps {
  activeView: View;
  onNavigate: (view: View) => void;
  chainValid: boolean | null;
  blockCount: number;
  onSignOut: () => void;
}

const sections: {
  label: string;
  items: { id: View; label: string; arrow?: boolean }[];
}[] = [
  {
    label: "Chain",
    items: [
      { id: "dashboard", label: "Dashboard" },
      { id: "chain-explorer", label: "Block Explorer", arrow: true },
      { id: "submit-contribution", label: "Submit Contribution" },
      { id: "verify", label: "Verify Chain" },
    ],
  },
  {
    label: "Contributors",
    items: [
      { id: "leaderboard", label: "Leaderboard" },
      { id: "repositories", label: "Repositories", arrow: true },
    ],
  },
  {
    label: "System",
    items: [
      { id: "audit-log", label: "Audit Log" },
      { id: "api-keys", label: "API Keys" },
    ],
  },
];

export default function Sidebar({
  activeView,
  onNavigate,
  chainValid,
  blockCount,
  onSignOut,
}: SidebarProps) {
  const { user } = useAuth();
  return (
    <aside className="sidebar">
      {/* Brand */}
      <div className="sidebar-brand">
        <div className="sidebar-brand-title">BlockSmith</div>
        <div className="sidebar-brand-sub">Contribution Chain v0.1.0</div>
      </div>

      {/* Nav sections */}
      {sections.map((section) => (
        <div key={section.label} className="sidebar-section">
          <div className="sidebar-section-label">{section.label}</div>
          {section.items.map((item) => (
            <button
              key={item.id}
              id={`nav-${item.id}`}
              className={`sidebar-item ${activeView === item.id ? "active" : ""}`}
              onClick={() => onNavigate(item.id)}
            >
              <span>{item.label}</span>
              {item.arrow && (
                <span className="sidebar-item-arrow">›</span>
              )}
            </button>
          ))}
        </div>
      ))}

      {/* Chain status */}
      <div className="sidebar-status">
        <div className="chain-status-badge">
          <div className={`status-dot ${chainValid === false ? "invalid" : ""}`} />
          <span style={{ color: "var(--text-dim)", fontSize: "9px" }}>
            {chainValid === null
              ? "CHAIN: CHECKING..."
              : chainValid
              ? "CHAIN: VALID"
              : "CHAIN: INVALID"}
          </span>
        </div>
        <div
          style={{
            fontSize: "9px",
            color: "var(--text-muted)",
            marginTop: "6px",
            textTransform: "uppercase",
            letterSpacing: "0.08em",
          }}
        >
          {blockCount} block{blockCount !== 1 ? "s" : ""} indexed
        </div>
      </div>

      {/* User profile */}
      {user && (
        <div className="sidebar-user">
          <div className="sidebar-user-avatar">
            <img
              src={user.avatar_url}
              alt={user.login}
              onError={(e) => {
                (e.target as HTMLImageElement).src =
                  `https://api.dicebear.com/7.x/initials/svg?seed=${user.login}`;
              }}
            />
          </div>
          <div className="sidebar-user-info">
            <div className="sidebar-user-name">{user.name || user.login}</div>
            <div className="sidebar-user-handle">@{user.login}</div>
          </div>
          <button
            className="sidebar-signout-btn"
            onClick={onSignOut}
            title="Sign out"
            id="sidebar-signout"
          >
            ⏻
          </button>
        </div>
      )}
    </aside>
  );
}
