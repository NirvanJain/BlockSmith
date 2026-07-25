import { Hono } from "hono";
import { cors } from "hono/cors";
import { createDB } from "./db";
import type { Env } from "./types";
import { authRoutes } from "./routes/auth";
import { feedRoutes } from "./routes/feed";
import { leaderboardRoutes } from "./routes/leaderboard";
import { profileRoutes } from "./routes/profile";
import { discoveryRoutes } from "./routes/discovery";
import { chatRoutes } from "./routes/chat";
import { meRoutes } from "./routes/me";

const app = new Hono<{ Bindings: Env }>();

// CORS
app.use("*", cors({
  origin: ["http://localhost:5173", "http://localhost:3000"],
  allowMethods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
  allowHeaders: ["Content-Type", "Authorization"],
}));

// Health check
app.get("/health", (c) => c.text("OK"));

// API routes
app.route("/api/v1/auth", authRoutes);
app.route("/api/v1/feed", feedRoutes);
app.route("/api/v1/leaderboard", leaderboardRoutes);
app.route("/api/v1/profile", profileRoutes);
app.route("/api/v1/discovery", discoveryRoutes);
app.route("/api/v1/chat", chatRoutes);
app.route("/api/v1/me", meRoutes);

// Root
app.get("/", (c) => c.text("BlockSmith API Running"));

export default app;
