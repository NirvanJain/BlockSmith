import { Hono } from "hono";
import { createDB } from "../db";
import { users, profiles } from "../db/schema";
import { eq } from "drizzle-orm";
import { authMiddleware } from "../middleware/auth";
import type { Env, Variables } from "../types";

export const meRoutes = new Hono<{ Bindings: Env; Variables: Variables }>();

// Apply auth middleware
meRoutes.use("*", authMiddleware);

// GET /api/v1/me
meRoutes.get("/", async (c) => {
  const userId = c.get("userId");
  const db = createDB(c.env.DB);

  const user = await db
    .select()
    .from(users)
    .where(eq(users.id, userId))
    .get();

  if (!user) {
    return c.json({ error: "User not found" }, 404);
  }

  const profile = await db
    .select()
    .from(profiles)
    .where(eq(profiles.userId, userId))
    .get();

  const skills = profile?.skills ? JSON.parse(profile.skills) : [];
  const interests = profile?.interests ? JSON.parse(profile.interests) : [];

  return c.json({
    id: user.id,
    github_username: user.githubUsername,
    github_id: user.githubId,
    email: user.email,
    name: user.name,
    avatar_url: user.avatarUrl,
    reputation_score: user.reputationScore,
    trust_score: user.trustScore,
    total_contributions: user.totalContributions,
    xp: user.xp,
    level: user.level,
    bio: profile?.bio,
    company: profile?.company,
    location: profile?.location,
    website: profile?.website,
    skills,
    interests,
  });
});
