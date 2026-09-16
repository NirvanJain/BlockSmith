# BlockSmith

A GitHub-flavored social platform (feed, profiles, leaderboard, chat) with a blockchain component.

## Layout

| Folder | Stack | Purpose |
|---|---|---|
| `frontend/` | React 19, Vite, Tailwind 4, React Router 7 | Web UI |
| `backend/` | Rust (Axum, Tokio), MongoDB, Redis, WebSockets | Full API server (original implementation) |
| `workers/` | TypeScript (Hono), Cloudflare Workers, D1 + Drizzle | Port of the API to Cloudflare Workers |

The `workers/` service exposes the same `/api/v1/*` routes as the Rust `backend/` — it's the deploy target for Cloudflare; the Rust backend is the feature-complete reference.

## Development

**Frontend**
```bash
cd frontend
npm install
npm run dev        # http://localhost:5173
```

**Rust backend**
```bash
cd backend
cargo run          # needs MongoDB; config via .env (see .env.example)
```

**Workers API**
```bash
cd workers
npm install
npm run db:migrate # apply D1 migrations locally
npm run dev        # wrangler dev on http://localhost:8787
```

## Notes

- `workers/wrangler.toml` still has a placeholder `database_id` — replace it before deploying.
- Secrets live in `backend/.env` (Rust) and `workers/.dev.vars` (Workers, create as needed); both are gitignored.
