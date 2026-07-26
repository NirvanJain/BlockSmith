import { Hono } from "hono";
import { createDB } from "../db";
import { users } from "../db/schema";
import { desc } from "drizzle-orm";
import type { Env } from "../types";

export const leaderboardRoutes = new Hono<{ Bindings: Env }>();

// GET /api/v1/leaderboard
leaderboardRoutes.get("/", async (c) => {
  const db = createDB(c.env.DB);

  const topUsers = await db
    .select()
    .from(users)
    .orderBy(desc(users.reputationScore))
    .limit(100)
    .all();

  if (topUsers.length === 0) {
    // Return mock data
    return c.json([
      {
        rank: 1,
        id: "1",
        name: "Nirvan Jain",
        github_username: "NirvanJain",
        avatar_url: "https://avatars.githubusercontent.com/u/5832347?v=4",
        reputation_score: 1250,
        xp: 1250,
        level: 13,
      },
      {
        rank: 2,
        id: "2",
        name: "The Octocat",
        github_username: "octocat",
        avatar_url: "https://avatars.githubusercontent.com/u/5832347?v=4",
        reputation_score: 890,
        xp: 890,
        level: 9,
      },
    ]);
  }

  return c.json(
    topUsers.map((user, index) => ({
      rank: index + 1,
      id: user.id,
      name: user.name,
      github_username: user.githubUsername,
      avatar_url: user.avatarUrl,
      reputation_score: user.reputationScore,
      xp: user.xp,
      level: user.level,
    }))
  );
});
