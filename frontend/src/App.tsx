import { useState, useEffect, useCallback } from "react";
import type { View, Block, LeaderboardEntry, RepoEntry, LogEntry } from "./types";
import { AuthProvider, useAuth } from "./context/AuthContext";
import Sidebar from "./components/Sidebar";
import Dashboard from "./components/Dashboard";
import ChainExplorer from "./components/ChainExplorer";
import SubmitContribution from "./components/SubmitContribution";
import Leaderboard from "./components/Leaderboard";
import Repositories from "./components/Repositories";
import VerifyChain from "./components/VerifyChain";
import AuditLog from "./components/AuditLog";
import ApiKeys from "./components/ApiKeys";
import AuthPage from "./pages/AuthPage";

// ─── Demo seed data ────────────────────────────────────────
const SEED_BLOCKS: Block[] = [
  {
    index: 0,
    timestamp: "2026-05-10T08:00:00Z",
    contributor: "BlockSmith",
    repository: "Genesis",
    contribution_type: "genesis",
    contribution_link: "None",
    previous_hash: "0000000000000000",
    hash: "genesis_hash_0000000000000000000000000000000000000000000000",
  },
  {
    index: 1,
    timestamp: "2026-05-12T10:33:00Z",
    contributor: "nirvanjain",
    repository: "BlockSmith/BlockSmith",
    contribution_type: "pull_request",
    contribution_link: "https://github.com/BlockSmith/BlockSmith/pull/1",
    previous_hash: "genesis_hash_0000000000000000000000000000000000000000000000",
    hash: "a1b2c3d4e5f6789012345678abcdef0123456789abcdef0123456789abcd",
  },
  {
    index: 2,
    timestamp: "2026-05-14T15:20:00Z",
    contributor: "devraj42",
    repository: "BlockSmith/BlockSmith",
    contribution_type: "commit",
    contribution_link: "https://github.com/BlockSmith/BlockSmith/commit/abc123",
    previous_hash: "a1b2c3d4e5f6789012345678abcdef0123456789abcdef0123456789abcd",
    hash: "b9f1e2d3c4a5067891234567defabc9876543210fedcba0987654321fedc",
  },
  {
    index: 3,
    timestamp: "2026-05-16T09:45:00Z",
    contributor: "priya_dev",
    repository: "BlockSmith/frontend",
    contribution_type: "issue",
    contribution_link: "https://github.com/BlockSmith/frontend/issues/14",
    previous_hash: "b9f1e2d3c4a5067891234567defabc9876543210fedcba0987654321fedc",
    hash: "cc3d5e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b",
  },
  {
    index: 4,
    timestamp: "2026-05-18T14:10:00Z",
    contributor: "nirvanjain",
    repository: "BlockSmith/BlockSmith",
    contribution_type: "pull_request",
    contribution_link: "https://github.com/BlockSmith/BlockSmith/pull/7",
    previous_hash: "cc3d5e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b",
    hash: "dd4e6f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c",
  },
];

const SEED_LEADERBOARD: LeaderboardEntry[] = [
  { rank: 1, github_username: "nirvanjain", reputation_score: 20, verified_contributions: 2, total_contributions: 2 },
  { rank: 2, github_username: "devraj42", reputation_score: 3, verified_contributions: 1, total_contributions: 1 },
  { rank: 3, github_username: "priya_dev", reputation_score: 5, verified_contributions: 1, total_contributions: 1 },
];

const SEED_REPOS: RepoEntry[] = [
  { name: "BlockSmith", owner: "nirvanjain", tracked_since: "2026-05-10", contributions: 4, status: "active" },
  { name: "frontend", owner: "nirvanjain", tracked_since: "2026-05-14", contributions: 1, status: "syncing" },
];

const SEED_LOGS: LogEntry[] = [
  { id: 1, time: "08:00:00", level: "ok", message: "Genesis block created — chain initialized" },
  { id: 2, time: "10:33:12", level: "ok", message: "Block #0001 appended — contributor: nirvanjain, type: pull_request, +10 pts" },
  { id: 3, time: "15:20:44", level: "ok", message: "Block #0002 appended — contributor: devraj42, type: commit, +3 pts" },
  { id: 4, time: "15:21:01", level: "info", message: "Chain validation passed — 3 blocks verified" },
  { id: 5, time: "09:45:07", level: "ok", message: "Block #0003 appended — contributor: priya_dev, type: issue, +5 pts" },
  { id: 6, time: "14:10:33", level: "ok", message: "Block #0004 appended — contributor: nirvanjain, type: pull_request, +10 pts" },
  { id: 7, time: "14:10:34", level: "info", message: "Leaderboard updated — nirvanjain leads at 20 pts" },
  { id: 8, time: "20:00:00", level: "info", message: "WebSocket connection established — client: dashboard" },
  { id: 9, time: "20:00:01", level: "info", message: "Listening for GitHub webhook events on /api/v1/webhook" },
];

const nowTime = () =>
  new Date().toTimeString().slice(0, 8);

// ─── Reputation calculator (mirrors backend) ───────────────
const reputationFor = (type: string): number => {
  if (type === "pull_request") return 10;
  if (type === "issue") return 5;
  if (type === "commit") return 3;
  return 1;
};

// ─── Page type ─────────────────────────────────────────────
type Page = "auth" | "app";

// ─── Dashboard shell (requires auth) ───────────────────────
function DashboardShell({ onSignOut }: { onSignOut: () => void }) {
  const [view, setView] = useState<View>("dashboard");
  const [blocks, setBlocks] = useState<Block[]>(SEED_BLOCKS);
  const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>(SEED_LEADERBOARD);
  const [repos, setRepos] = useState<RepoEntry[]>(SEED_REPOS);
  const [logs, setLogs] = useState<LogEntry[]>(SEED_LOGS);
  const [chainValid, setChainValid] = useState<boolean | null>(true);
  const [loading, setLoading] = useState(false);
  const [verifying, setVerifying] = useState(false);
  const [toasts, setToasts] = useState<{ id: number; msg: string; type: "ok" | "error" | "warn" }[]>([]);

  const addLog = useCallback((level: LogEntry["level"], message: string) => {
    setLogs((prev) => [
      ...prev,
      { id: Date.now(), time: nowTime(), level, message },
    ]);
  }, []);

  const showToast = useCallback((msg: string, type: "ok" | "error" | "warn" = "ok") => {
    const id = Date.now();
    setToasts((t) => [...t, { id, msg, type }]);
    setTimeout(() => setToasts((t) => t.filter((x) => x.id !== id)), 3500);
  }, []);

  // Simulate WebSocket ping
  useEffect(() => {
    const interval = setInterval(() => {
      if (Math.random() < 0.15) {
        addLog("info", "WebSocket heartbeat — connection active");
      }
    }, 15000);
    return () => clearInterval(interval);
  }, [addLog]);

  // ── Submit a new contribution block ──────────────────────
  const handleSubmit = useCallback(
    async (data: Omit<Block, "index" | "hash" | "previous_hash" | "timestamp">) => {
      setLoading(true);
      try {
        await new Promise((r) => setTimeout(r, 900));

        const prev = blocks[blocks.length - 1];
        const index = prev.index + 1;
        const timestamp = new Date().toISOString();
        const fakeHash = Array.from({ length: 60 }, () =>
          "0123456789abcdef"[Math.floor(Math.random() * 16)]
        ).join("");

        const newBlock: Block = {
          index,
          timestamp,
          contributor: data.contributor,
          repository: data.repository,
          contribution_type: data.contribution_type,
          contribution_link: data.contribution_link,
          previous_hash: prev.hash,
          hash: fakeHash,
        };

        setBlocks((b) => [...b, newBlock]);

        const pts = reputationFor(data.contribution_type);
        setLeaderboard((lb) => {
          const existing = lb.find((e) => e.github_username === data.contributor);
          let updated: LeaderboardEntry[];
          if (existing) {
            updated = lb.map((e) =>
              e.github_username === data.contributor
                ? {
                    ...e,
                    reputation_score: e.reputation_score + pts,
                    verified_contributions: e.verified_contributions + 1,
                    total_contributions: e.total_contributions + 1,
                  }
                : e
            );
          } else {
            updated = [
              ...lb,
              {
                rank: lb.length + 1,
                github_username: data.contributor,
                reputation_score: pts,
                verified_contributions: 1,
                total_contributions: 1,
              },
            ];
          }
          return updated
            .sort((a, b) => b.reputation_score - a.reputation_score)
            .map((e, i) => ({ ...e, rank: i + 1 }));
        });

        setChainValid(true);
        addLog("ok", `Block #${String(index).padStart(4, "0")} appended — contributor: ${data.contributor}, type: ${data.contribution_type}, +${pts} pts`);
        showToast(`Block #${index} appended to chain — +${pts} reputation pts`, "ok");
      } catch {
        addLog("err", "Failed to append block — check server connection");
        showToast("Failed to append block", "error");
      } finally {
        setLoading(false);
      }
    },
    [blocks, addLog, showToast]
  );

  // ── Validate chain ────────────────────────────────────────
  const handleVerify = useCallback(async () => {
    setVerifying(true);
    try {
      await new Promise((r) => setTimeout(r, 1200));
      addLog("info", `Chain validation started — ${blocks.length} blocks to check`);
      let valid = true;
      for (let i = 1; i < blocks.length; i++) {
        if (blocks[i].previous_hash !== blocks[i - 1].hash) {
          valid = false;
          break;
        }
      }
      setChainValid(valid);
      if (valid) {
        addLog("ok", `Chain validation passed — all ${blocks.length} blocks verified`);
        showToast("Chain integrity verified — all blocks valid", "ok");
      } else {
        addLog("err", "Chain validation FAILED — hash mismatch detected");
        showToast("Chain tampered — hash mismatch detected", "error");
      }
    } finally {
      setVerifying(false);
    }
  }, [blocks, addLog, showToast]);

  // ── Add repo ──────────────────────────────────────────────
  const handleAddRepo = useCallback(
    (owner: string, repo: string) => {
      const existing = repos.find((r) => r.owner === owner && r.name === repo);
      if (existing) {
        showToast(`${owner}/${repo} already tracked`, "warn");
        return;
      }
      const newRepo: RepoEntry = {
        owner,
        name: repo,
        tracked_since: new Date().toISOString().slice(0, 10),
        contributions: 0,
        status: "syncing",
      };
      setRepos((r) => [...r, newRepo]);
      addLog("info", `Repository connected — ${owner}/${repo} — webhook registration pending`);
      showToast(`${owner}/${repo} connected — syncing...`, "ok");
      setTimeout(() => {
        setRepos((r) =>
          r.map((x) =>
            x.owner === owner && x.name === repo ? { ...x, status: "active" } : x
          )
        );
        addLog("ok", `Repository ${owner}/${repo} is now active and receiving webhooks`);
      }, 2500);
    },
    [repos, addLog, showToast]
  );

  // ── Render current view ───────────────────────────────────
  const renderView = () => {
    switch (view) {
      case "dashboard":
        return (
          <Dashboard
            blocks={blocks}
            leaderboard={leaderboard}
            chainValid={chainValid}
            logs={logs}
            onNavigate={(v) => setView(v as View)}
          />
        );
      case "chain-explorer":
        return (
          <ChainExplorer
            blocks={blocks}
            loading={loading}
            onRefresh={() => {
              addLog("info", "Block explorer refreshed");
              showToast("Chain refreshed", "ok");
            }}
          />
        );
      case "submit-contribution":
        return <SubmitContribution onSubmit={handleSubmit} loading={loading} />;
      case "leaderboard":
        return <Leaderboard entries={leaderboard} loading={false} />;
      case "repositories":
        return <Repositories repos={repos} onAdd={handleAddRepo} />;
      case "verify":
        return (
          <VerifyChain
            blocks={blocks}
            chainValid={chainValid}
            onVerify={handleVerify}
            verifying={verifying}
          />
        );
      case "audit-log":
        return <AuditLog logs={logs} />;
      case "api-keys":
        return <ApiKeys />;
      default:
        return null;
    }
  };

  const viewTitles: Record<View, string> = {
    dashboard: "System Dashboard",
    "chain-explorer": "Block Explorer",
    "submit-contribution": "Submit Contribution",
    leaderboard: "Leaderboard",
    repositories: "Repositories",
    verify: "Verify Chain",
    "audit-log": "Audit Log",
    "api-keys": "API Keys",
  };

  return (
    <>
      {/* Subtle scanlines overlay */}
      <div className="scanlines" aria-hidden="true" />

      <div className="app-shell">
        {/* Sidebar */}
        <Sidebar
          activeView={view}
          onNavigate={setView}
          chainValid={chainValid}
          blockCount={blocks.length}
          onSignOut={onSignOut}
        />

        {/* Main */}
        <main className="main-content">
          <div className="content-header">
            <div className="content-title">{viewTitles[view]}</div>
            <div className="content-subtitle">
              BlockSmith — Contribution Verification Chain &nbsp;·&nbsp;{" "}
              {blocks.length} blocks indexed &nbsp;·&nbsp;{" "}
              {chainValid === null ? "Checking..." : chainValid ? "Chain valid" : "Chain tampered"}
            </div>
          </div>
          <div className="content-body">{renderView()}</div>
        </main>
      </div>

      {/* Toast notifications */}
      <div className="toast-container" aria-live="polite">
        {toasts.map((t) => (
          <div
            key={t.id}
            className={`toast ${t.type === "error" ? "error" : t.type === "warn" ? "warn" : ""}`}
          >
            <span
              style={{
                color:
                  t.type === "error"
                    ? "var(--accent-red)"
                    : t.type === "warn"
                    ? "var(--accent-amber)"
                    : "var(--accent-green)",
                fontSize: "11px",
              }}
            >
              {t.type === "error" ? "✕" : t.type === "warn" ? "▲" : "▲"}
            </span>
            <span style={{ color: "var(--text-dim)" }}>{t.msg}</span>
          </div>
        ))}
      </div>
    </>
  );
}

// ─── App Root (auth-aware routing) ─────────────────────────
function AppRoot() {
  const { isAuthenticated, isLoading, logout } = useAuth();
  const [page, setPage] = useState<Page>("auth");

  // When auth state resolves, if authenticated go straight to app
  useEffect(() => {
    if (!isLoading && isAuthenticated) {
      setPage("app");
    }
  }, [isAuthenticated, isLoading]);

  const handleSignOut = () => {
    logout();
    setPage("auth");
  };

  // Show a minimal loading state while rehydrating session
  if (isLoading) {
    return (
      <div className="term-root">
        <div className="term-spin" style={{ width: 16, height: 16 }} />
      </div>
    );
  }

  if (page === "auth") {
    return <AuthPage />;
  }

  return <DashboardShell onSignOut={handleSignOut} />;
}

// ─── Root export (wraps everything in AuthProvider) ─────────
export default function App() {
  return (
    <AuthProvider>
      <AppRoot />
    </AuthProvider>
  );
}