import { Hono } from "hono";
import { createDB } from "../db";
import { users, profiles, badges, activities, userBadges } from "../db/schema";
import { eq, desc } from "drizzle-orm";
import type { Env } from "../types";

export const profileRoutes = new Hono<{ Bindings: Env }>();

// GET /api/v1/profile/:username
profileRoutes.get("/:username", async (c) => {
  const username = c.req.param("username");
  const db = createDB(c.env.DB);

  // Find user by GitHub username
  const user = await db
    .select()
    .from(users)
    .where(eq(users.githubUsername, username))
    .get();

  if (!user) {
    // Return mock data like the Rust backend
    return c.json({
      user: {
        id: crypto.randomUUID(),
        name: `Mock ${username}`,
        github_username: username,
        avatar_url: "https://avatars.githubusercontent.com/u/5832347?v=4",
        reputation_score: 120,
        xp: 120,
        level: 2,
        bio: "Passionate developer exploring BlockSmith",
        company: "OpenSource",
        location: "Earth",
        website: "https://blocksmith.dev",
        skills: ["Rust", "TypeScript", "React"],
        interests: ["Compilers", "Distributed Systems"],
      },
      badges: [
        {
          id: "b1",
          name: "First Merge",
          description: "Merged your first pull request",
          icon_url: null,
        },
      ],
      activities: [
        {
          id: "a1",
          activity_type: "pr_merged",
          title: "Merged PR #12 in NirvanJain/BlockSmith",
          description: "feat: Add verification engine",
          link: "https://github.com/NirvanJain/BlockSmith",
          xp_earned: 10,
          created_at: new Date().toISOString(),
        },
      ],
    });
  }

  // Get profile
  const profile = await db
    .select()
    .from(profiles)
    .where(eq(profiles.userId, user.id))
    .get();

  // Get user badges
  const userBadgesList = await db
    .select({
      id: badges.id,
      name: badges.name,
      description: badges.description,
      iconUrl: badges.iconUrl,
    })
    .from(userBadges)
    .innerJoin(badges, eq(userBadges.badgeId, badges.id))
    .where(eq(userBadges.userId, user.id))
    .all();

  // Get recent activities
  const recentActivities = await db
    .select()
    .from(activities)
    .where(eq(activities.userId, user.id))
    .orderBy(desc(activities.createdAt))
    .limit(10)
    .all();

  // Parse skills/interests from JSON
  const skills = profile?.skills ? JSON.parse(profile.skills) : [];
  const interests = profile?.interests ? JSON.parse(profile.interests) : [];

  return c.json({
    user: {
      id: user.id,
      name: user.name,
      github_username: user.githubUsername,
      avatar_url: user.avatarUrl,
      reputation_score: user.reputationScore,
      xp: user.xp,
      level: user.level,
      bio: profile?.bio,
      company: profile?.company,
      location: profile?.location,
      website: profile?.website,
      skills,
      interests,
    },
    badges: userBadgesList.map((b) => ({
      id: b.id,
      name: b.name,
      description: b.description,
      icon_url: b.iconUrl,
    })),
    activities: recentActivities.map((a) => ({
      id: a.id,
      activity_type: a.activityType,
      title: a.title,
      description: a.description,
      link: a.link,
      xp_earned: a.xpEarned,
      created_at: a.createdAt,
    })),
  });
});
