import { Hono } from "hono";
import { createDB } from "../db";
import { activities, users } from "../db/schema";
import { desc, eq } from "drizzle-orm";
import type { Env } from "../types";

export const feedRoutes = new Hono<{ Bindings: Env }>();

// GET /api/v1/feed
feedRoutes.get("/", async (c) => {
  const db = createDB(c.env.DB);

  // Get recent activities with author info (join)
  const feedItems = await db
    .select({
      id: activities.id,
      activityType: activities.activityType,
      title: activities.title,
      description: activities.description,
      link: activities.link,
      xpEarned: activities.xpEarned,
      createdAt: activities.createdAt,
      authorName: users.name,
      authorUsername: users.githubUsername,
      authorAvatar: users.avatarUrl,
    })
    .from(activities)
    .leftJoin(users, eq(activities.userId, users.id))
    .orderBy(desc(activities.createdAt))
    .limit(50)
    .all();

  if (feedItems.length === 0) {
    // Return mock data like the Rust backend
    return c.json([
      {
        id: "act_1",
        activity_type: "pr_merged",
        title: "Merged PR #21 in axum-rs/axum",
        description: "feat: Add high performance WebSocket channels",
        link: "https://github.com/tokio-rs/axum",
        xp_earned: 10,
        created_at: new Date().toISOString(),
        actor: {
          name: "The Octocat",
          avatar_url: "https://avatars.githubusercontent.com/u/5832347?v=4",
          github_username: "octocat",
        },
      },
      {
        id: "act_2",
        activity_type: "badge_awarded",
        title: "Unlocked 'Contributor' Badge",
        description: "Earned 100 XP from verifiable contributions",
        link: "",
        xp_earned: 25,
        created_at: new Date().toISOString(),
        actor: {
          name: "Nirvan Jain",
          avatar_url: "https://avatars.githubusercontent.com/u/5832347?v=4",
          github_username: "NirvanJain",
        },
      },
    ]);
  }

  return c.json(
    feedItems.map((item) => ({
      id: item.id,
      activity_type: item.activityType,
      title: item.title,
      description: item.description,
      link: item.link,
      xp_earned: item.xpEarned,
      created_at: item.createdAt,
      actor: {
        name: item.authorName,
        avatar_url: item.authorAvatar,
        github_username: item.authorUsername,
      },
    }))
  );
});
