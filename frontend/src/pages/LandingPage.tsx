import { useState, useEffect, useRef } from "react";
import { useAuth } from "../hooks/useAuth";

// ─── Animated counter ───────────────────────────────────────
function Counter({ target, suffix = "" }: { target: number; suffix?: string }) {
  const [count, setCount] = useState(0);
  const ref = useRef<HTMLSpanElement>(null);
  const started = useRef(false);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting && !started.current) {
          started.current = true;
          const duration = 2000;
          const steps = 60;
          const step = target / steps;
          let cur = 0;
          const timer = setInterval(() => {
            cur = Math.min(cur + step, target);
            setCount(Math.floor(cur));
            if (cur >= target) clearInterval(timer);
          }, duration / steps);
        }
      },
      { threshold: 0.5 }
    );
    observer.observe(el);
    return () => observer.disconnect();
  }, [target]);

  return (
    <span ref={ref}>
      {count.toLocaleString()}
      {suffix}
    </span>
  );
}

// ─── Feature Card ────────────────────────────────────────────
interface FeatureCardProps {
  icon: string;
  title: string;
  desc: string;
  tag: string;
}

function FeatureCard({ icon, title, desc, tag }: FeatureCardProps) {
  return (
    <div className="landing-feature-card">
      <div className="landing-feature-icon">{icon}</div>
      <div className="landing-feature-tag">{tag}</div>
      <h3 className="landing-feature-title">{title}</h3>
      <p className="landing-feature-desc">{desc}</p>
    </div>
  );
}

// ─── Step ────────────────────────────────────────────────────
function Step({ num, title, desc }: { num: string; title: string; desc: string }) {
  return (
    <div className="landing-step">
      <div className="landing-step-num">{num}</div>
      <div>
        <div className="landing-step-title">{title}</div>
        <div className="landing-step-desc">{desc}</div>
      </div>
    </div>
  );
}

// ─── GitHub Icon SVG ─────────────────────────────────────────
function GitHubIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
      <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0 1 12 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z" />
    </svg>
  );
}

// ─── Main Landing Page ───────────────────────────────────────
export default function LandingPage({ onGetStarted }: { onGetStarted: () => void }) {
  const { loginWithGitHub } = useAuth();
  const [scrolled, setScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => setScrolled(window.scrollY > 40);
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  const handleCTA = () => {
    onGetStarted();
  };

  return (
    <div className="landing-root">
      {/* Grid background */}
      <div className="landing-grid-bg" aria-hidden="true" />
      {/* Gradient orbs */}
      <div className="landing-orb landing-orb-1" aria-hidden="true" />
      <div className="landing-orb landing-orb-2" aria-hidden="true" />

      {/* ── Nav ── */}
      <nav className={`landing-nav${scrolled ? " scrolled" : ""}`}>
        <div className="landing-nav-inner">
          <div className="landing-nav-brand">
            <span className="landing-nav-logo">⬡</span>
            <span className="landing-nav-title">BlockSmith</span>
          </div>
          <div className="landing-nav-links">
            <a href="#features" className="landing-nav-link">Features</a>
            <a href="#how-it-works" className="landing-nav-link">How it works</a>
            <a href="#stats" className="landing-nav-link">Stats</a>
          </div>
          <button className="landing-nav-cta" onClick={handleCTA} id="nav-get-started">
            Get Started
          </button>
        </div>
      </nav>

      {/* ── Hero ── */}
      <section className="landing-hero">
        <div className="landing-hero-badge">
          <span className="landing-hero-badge-dot" />
          Open Source · Blockchain-verified · Production Ready
        </div>

        <h1 className="landing-hero-heading">
          Your GitHub contributions,
          <br />
          <span className="landing-hero-accent">immutably verified.</span>
        </h1>

        <p className="landing-hero-sub">
          BlockSmith creates an unbreakable on-chain record of every commit, pull request,
          and issue — giving developers a tamper-proof reputation they own forever.
        </p>

        <div className="landing-hero-actions">
          <button className="landing-cta-github" onClick={handleCTA} id="hero-github-cta">
            <GitHubIcon />
            Continue with GitHub
          </button>
          <a href="#features" className="landing-cta-ghost">
            See how it works
            <span className="landing-cta-arrow">↓</span>
          </a>
        </div>

        {/* Terminal preview strip */}
        <div className="landing-terminal">
          <div className="landing-terminal-bar">
            <span className="lt-dot red" />
            <span className="lt-dot amber" />
            <span className="lt-dot green" />
            <span className="lt-dot-title">blocksmith — chain explorer</span>
          </div>
          <div className="landing-terminal-body">
            <div className="lt-line">
              <span className="lt-prompt">▸</span>
              <span className="lt-cmd">blocksmith verify --chain</span>
            </div>
            <div className="lt-line lt-ok">
              <span className="lt-prefix">✓</span>
              <span>Block #0042 — nirvanjain · pull_request · +10 pts</span>
            </div>
            <div className="lt-line lt-ok">
              <span className="lt-prefix">✓</span>
              <span>Block #0043 — devraj42 · commit · +3 pts</span>
            </div>
            <div className="lt-line lt-info">
              <span className="lt-prefix">ℹ</span>
              <span>Chain validation passed — 43 blocks verified · hash integrity OK</span>
            </div>
            <div className="lt-line">
              <span className="lt-prompt">▸</span>
              <span className="lt-cmd cursor-blink">_</span>
            </div>
          </div>
        </div>
      </section>

      {/* ── Features ── */}
      <section className="landing-section" id="features">
        <div className="landing-section-inner">
          <div className="landing-section-label">Core features</div>
          <h2 className="landing-section-heading">
            Built for developers who take
            <br />
            <span className="landing-heading-accent">their reputation seriously.</span>
          </h2>

          <div className="landing-features-grid">
            <FeatureCard
              icon="⛓"
              tag="Blockchain"
              title="Immutable Chain of Contributions"
              desc="Every commit, PR, and issue is cryptographically hashed and linked to the previous block. No one — not even us — can alter your history."
            />
            <FeatureCard
              icon="◈"
              tag="Reputation"
              title="Weighted Reputation Scoring"
              desc="Pull requests score 10pts, issues 5pts, commits 3pts. A transparent algorithm surfaces real contributors — not just commit spammers."
            />
            <FeatureCard
              icon="⬡"
              tag="Verification"
              title="One-click Chain Verify"
              desc="Instantly validate the entire chain with a single API call. Detect any tampering, hash mismatches, or injected blocks in milliseconds."
            />
            <FeatureCard
              icon="⚡"
              tag="Real-time"
              title="Live WebSocket Events"
              desc="Webhook events from GitHub land in real-time. Your dashboard updates the moment you merge a PR — zero polling, zero delay."
            />
            <FeatureCard
              icon="◉"
              tag="Leaderboard"
              title="Public Developer Leaderboard"
              desc="A ranked, verifiable leaderboard anyone can inspect. Show recruiters and collaborators a score they can trust — backed by cryptographic proof."
            />
            <FeatureCard
              icon="⌗"
              tag="API"
              title="Full REST + WebSocket API"
              desc="Integrate BlockSmith into your CI/CD pipeline, bots, or dashboards. Every endpoint is documented, typed, and rate-limited for production use."
            />
          </div>
        </div>
      </section>

      {/* ── How it works ── */}
      <section className="landing-section landing-section-alt" id="how-it-works">
        <div className="landing-section-inner">
          <div className="landing-section-label">How it works</div>
          <h2 className="landing-section-heading">
            Three steps to
            <span className="landing-heading-accent"> verified reputation.</span>
          </h2>

          <div className="landing-steps">
            <Step
              num="01"
              title="Connect your GitHub"
              desc="OAuth login in one click. We read your public profile — no write access, no secrets. Your data stays yours."
            />
            <div className="landing-steps-connector" aria-hidden="true" />
            <Step
              num="02"
              title="Track repositories"
              desc="Add any GitHub repo. BlockSmith registers a webhook and starts indexing every contribution as a new block on your personal chain."
            />
            <div className="landing-steps-connector" aria-hidden="true" />
            <Step
              num="03"
              title="Share your proof"
              desc="Get a public verification URL for any contribution. Anyone can verify the hash — no account needed, no trust required."
            />
          </div>
        </div>
      </section>

      {/* ── Stats ── */}
      <section className="landing-section" id="stats">
        <div className="landing-section-inner">
          <div className="landing-stats-grid">
            <div className="landing-stat">
              <div className="landing-stat-value">
                <Counter target={12847} suffix="+" />
              </div>
              <div className="landing-stat-label">Blocks Indexed</div>
            </div>
            <div className="landing-stat">
              <div className="landing-stat-value">
                <Counter target={3291} suffix="+" />
              </div>
              <div className="landing-stat-label">Developers Verified</div>
            </div>
            <div className="landing-stat">
              <div className="landing-stat-value">
                <Counter target={847} suffix="+" />
              </div>
              <div className="landing-stat-label">Repositories Tracked</div>
            </div>
            <div className="landing-stat">
              <div className="landing-stat-value">
                <Counter target={100} suffix="%" />
              </div>
              <div className="landing-stat-label">Chain Integrity</div>
            </div>
          </div>
        </div>
      </section>

      {/* ── Final CTA ── */}
      <section className="landing-final-cta">
        <div className="landing-orb landing-orb-3" aria-hidden="true" />
        <div className="landing-section-inner" style={{ textAlign: "center" }}>
          <div className="landing-section-label">Get started free</div>
          <h2 className="landing-section-heading">
            Your contributions deserve
            <br />
            <span className="landing-heading-accent">a permanent record.</span>
          </h2>
          <p className="landing-final-sub">
            Join thousands of developers who've already verified their open-source history.
            <br />
            No credit card. No setup fees. Forever free for public repositories.
          </p>
          <button className="landing-cta-github large" onClick={handleCTA} id="final-github-cta">
            <GitHubIcon />
            Start with GitHub — it's free
          </button>
        </div>
      </section>

      {/* ── Footer ── */}
      <footer className="landing-footer">
        <div className="landing-footer-inner">
          <div className="landing-footer-brand">
            <span className="landing-nav-logo">⬡</span>
            <span>BlockSmith</span>
            <span className="landing-footer-sep">·</span>
            <span className="landing-footer-sub">Contribution Verification Chain</span>
          </div>
          <div className="landing-footer-links">
            <a href="#" className="landing-footer-link">Docs</a>
            <a href="#" className="landing-footer-link">API</a>
            <a href="#" className="landing-footer-link">Privacy</a>
            <a href="#" className="landing-footer-link">Terms</a>
            <a
              href="https://github.com"
              target="_blank"
              rel="noopener noreferrer"
              className="landing-footer-link"
            >
              GitHub
            </a>
          </div>
        </div>
        <div className="landing-footer-copy">
          © {new Date().getFullYear()} BlockSmith. Built on trust, secured by cryptography.
        </div>
      </footer>
    </div>
  );
}
