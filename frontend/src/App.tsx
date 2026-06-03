import React, { useEffect, useState } from 'react';
import './styles.css';
import ActivityFeed from './components/ActivityFeed';
import ChatBox from './components/ChatBox';
import ThreeCube from './components/ThreeCube';
import Leaderboard from './components/Leaderboard';
import Discovery from './components/Discovery';
import { fetchMe, fetchFeed, fetchLeaderboard, fetchDiscovery } from './api';
import type { User, Activity, LeaderboardEntry, DiscoveryIssue } from './types';

type View = 'dashboard' | 'leaderboard' | 'discover' | 'chat' | 'profile';

const App: React.FC = () => {
  const [token, setToken] = useState<string | null>(localStorage.getItem('blocksmith_jwt'));
  const [tokenInput, setTokenInput] = useState('');
  const [view, setView] = useState<View>('dashboard');

  const [user, setUser] = useState<User | null>(null);
  const [feed, setFeed] = useState<Activity[]>([]);
  const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>([]);
  const [discovery, setDiscovery] = useState<DiscoveryIssue[]>([]);

  // Load public data (no auth needed)
  useEffect(() => {
    fetchFeed().then(setFeed).catch(console.error);
    fetchLeaderboard().then(setLeaderboard).catch(console.error);
    fetchDiscovery().then(setDiscovery).catch(console.error);
  }, []);

  // Load user when token changes
  useEffect(() => {
    if (!token) { setUser(null); return; }
    fetchMe(token).then(setUser).catch(() => setUser(null));
  }, [token]);

  const handleLogin = () => {
    const t = tokenInput.trim();
    if (!t) return;
    localStorage.setItem('blocksmith_jwt', t);
    setToken(t);
    setTokenInput('');
  };

  const handleLogout = () => {
    localStorage.removeItem('blocksmith_jwt');
    setToken(null);
    setUser(null);
  };

  const navItems: { id: View; label: string }[] = [
    { id: 'dashboard', label: 'Dashboard' },
    { id: 'leaderboard', label: 'Leaderboard' },
    { id: 'discover', label: 'Discover' },
    { id: 'chat', label: 'Chat' },
    { id: 'profile', label: 'Profile' },
  ];

  return (
    <>
      <header className="header">
        <h1 className="logo">BlockSmith</h1>
        <nav className="nav">
          {navItems.map((item) => (
            <button
              key={item.id}
              className={`nav-link ${view === item.id ? 'active' : ''}`}
              onClick={() => setView(item.id)}
            >
              {item.label}
            </button>
          ))}
          {user ? (
            <button onClick={handleLogout} className="btn primary" style={{ marginLeft: '1rem' }}>
              Logout
            </button>
          ) : null}
        </nav>
      </header>

      {!token && (
        <div className="token-banner">
          <span>Paste your Clerk JWT to enable chat and profile:</span>
          <input
            type="text"
            placeholder="eyJhbGci..."
            value={tokenInput}
            onChange={(e) => setTokenInput(e.target.value)}
            onKeyDown={(e) => { if (e.key === 'Enter') handleLogin(); }}
            style={{ width: '320px' }}
          />
          <button className="btn primary" onClick={handleLogin}>Sign In</button>
          <button
            className="btn"
            style={{ background: '#333' }}
            onClick={() => {
              const demo = 'mock_demo_token';
              localStorage.setItem('blocksmith_jwt', demo);
              setToken(demo);
            }}
          >
            Use Demo Token
          </button>
        </div>
      )}

      <main className="main">
        {view === 'dashboard' && (
          <>
            <section className="card welcome-card">
              {user ? (
                <div className="user-hero">
                  <img src={user.avatar_url} alt={user.name} className="avatar-lg" />
                  <div>
                    <h2>Welcome, {user.name}!</h2>
                    {user.github_username && (
                      <p className="user-handle">@{user.github_username}</p>
                    )}
                    <div className="user-stats">
                      <span className="stat-chip">Lvl {user.level}</span>
                      <span className="stat-chip">{user.xp} XP</span>
                      <span className="stat-chip">{user.reputation_score} Rep</span>
                      <span className="stat-chip">{user.total_contributions} Contributions</span>
                    </div>
                  </div>
                </div>
              ) : (
                <div>
                  <h2>Welcome to BlockSmith!</h2>
                  <p>Track open-source contributions, earn XP, and discover issues tailored to you.</p>
                </div>
              )}
            </section>

            <section className="card activity-card">
              <h2>Live Activity Feed</h2>
              <ActivityFeed items={feed} />
            </section>

            <section className="card three-card">
              <h2>3D Explorer</h2>
              <ThreeCube />
            </section>
          </>
        )}

        {view === 'leaderboard' && (
          <section className="card full-width">
            <h2>Leaderboard</h2>
            <Leaderboard entries={leaderboard} />
          </section>
        )}

        {view === 'discover' && (
          <section className="card full-width">
            <h2>Discover Issues</h2>
            <Discovery issues={discovery} />
          </section>
        )}

        {view === 'chat' && (
          <section className="card full-width">
            <h2>Messages</h2>
            <ChatBox token={token} currentUsername={user?.github_username} />
          </section>
        )}

        {view === 'profile' && (
          <section className="card full-width">
            {user ? (
              <div className="profile-view">
                <div className="profile-header">
                  <img src={user.avatar_url} alt={user.name} className="avatar-xl" />
                  <div>
                    <h2>{user.name}</h2>
                    {user.github_username && (
                      <a
                        href={`https://github.com/${user.github_username}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="user-handle"
                      >
                        @{user.github_username}
                      </a>
                    )}
                    {user.bio && <p style={{ marginTop: '0.5rem' }}>{user.bio}</p>}
                    <div className="user-stats" style={{ marginTop: '0.75rem' }}>
                      <span className="stat-chip">Level {user.level}</span>
                      <span className="stat-chip">{user.xp} XP</span>
                      <span className="stat-chip">{user.reputation_score} Reputation</span>
                      <span className="stat-chip">{user.total_contributions} PRs merged</span>
                    </div>
                  </div>
                </div>
                {user.location && <p><strong>Location:</strong> {user.location}</p>}
                {user.company && <p><strong>Company:</strong> {user.company}</p>}
                {user.website && (
                  <p>
                    <strong>Website:</strong>{' '}
                    <a href={user.website} target="_blank" rel="noopener noreferrer">{user.website}</a>
                  </p>
                )}
                {user.skills && user.skills.length > 0 && (
                  <div style={{ marginTop: '1rem' }}>
                    <strong>Skills</strong>
                    <div className="discovery-labels" style={{ marginTop: '0.4rem' }}>
                      {user.skills.map((s) => <span key={s} className="label-chip">{s}</span>)}
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <div>
                <h2>Profile</h2>
                <p className="empty-state">Sign in to view your profile.</p>
              </div>
            )}
          </section>
        )}
      </main>

      <footer className="footer">
        <p>&copy; 2026 BlockSmith – Empowering Open-Source Talent</p>
      </footer>
    </>
  );
};

export default App;
