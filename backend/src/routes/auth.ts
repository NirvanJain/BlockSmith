import { Hono } from "hono";
import { createDB } from "../db";
import { users, profiles } from "../db/schema";
import { eq } from "drizzle-orm";
import { createJWT, verifyJWT } from "../auth/jwt";
import type { Env } from "../types";

export const authRoutes = new Hono<{ Bindings: Env }>();

// GET /api/v1/auth/github - Redirect to GitHub OAuth
authRoutes.get("/github", (c) => {
  const state = crypto.randomUUID();
  const url = `https://github.com/login/oauth/authorize?client_id=${c.env.GITHUB_CLIENT_ID}&redirect_uri=${encodeURIComponent(c.env.GITHUB_REDIRECT_URI)}&scope=read:user user:email&state=${state}`;
  return c.redirect(url);
});

// GET /api/v1/auth/github/callback
authRoutes.get("/github/callback", async (c) => {
  const code = c.req.query("code");
  if (!code) {
    return c.json({ error: "Missing code parameter" }, 400);
  }

  const db = createDB(c.env.DB);

  // 1. Exchange code for access token
  const tokenRes = await fetch("https://github.com/login/oauth/access_token", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Accept: "application/json",
    },
    body: JSON.stringify({
      client_id: c.env.GITHUB_CLIENT_ID,
      client_secret: c.env.GITHUB_CLIENT_SECRET,
      code,
    }),
  });

  const tokenData = await tokenRes.json() as { access_token?: string; error?: string };
  if (!tokenData.access_token) {
    return c.json({ error: "Failed to exchange code", details: tokenData }, 401);
  }

  const accessToken = tokenData.access_token;

  // 2. Fetch GitHub user profile
  const ghUserRes = await fetch("https://api.github.com/user", {
    headers: {
      Authorization: `Bearer ${accessToken}`,
      Accept: "application/json",
    },
  });

  const ghUser = await ghUserRes.json() as {
    id: number;
    login: string;
    name?: string;
    email?: string;
    avatar_url?: string;
    bio?: string;
    company?: string;
  };

  // 3. Fetch email if not in profile
  let email = ghUser.email;
  if (!email) {
    const emailRes = await fetch("https://api.github.com/user/emails", {
      headers: {
        Authorization: `Bearer ${accessToken}`,
        Accept: "application/json",
      },
    });
    const emails = await emailRes.json() as Array<{ email: string; primary: boolean }>;
    email = emails.find((e) => e.primary)?.email ?? emails[0]?.email;
  }

  const githubId = ghUser.id.toString();

  // 4. Upsert user
  const existingUser = await db
    .select()
    .from(users)
    .where(eq(users.githubId, githubId))
    .get();

  let user;
  if (existingUser) {
    // Update
    await db
      .update(users)
      .set({
        githubUsername: ghUser.login,
        githubAccessToken: accessToken,
        updatedAt: new Date().toISOString(),
      })
      .where(eq(users.githubId, githubId));

    user = await db.select().from(users).where(eq(users.githubId, githubId)).get();
  } else {
    // Create
    const userId = crypto.randomUUID();
    const now = new Date().toISOString();

    await db.insert(users).values({
      id: userId,
      githubId,
      githubUsername: ghUser.login,
      githubAccessToken: accessToken,
      email: email ?? null,
      name: ghUser.name ?? null,
      avatarUrl: ghUser.avatar_url ?? null,
      reputationScore: 0,
      trustScore: 0,
      totalContributions: 0,
      xp: 0,
      level: 1,
      createdAt: now,
      updatedAt: now,
    });

    // Create profile
    await db.insert(profiles).values({
      userId,
      bio: ghUser.bio ?? null,
      company: ghUser.company ?? null,
      skills: "[]",
      interests: "[]",
      updatedAt: now,
    });

    user = await db.select().from(users).where(eq(users.id, userId)).get();
  }

  if (!user) {
    return c.json({ error: "Failed to create/find user" }, 500);
  }

  // 5. Generate JWT
  const token = await createJWT(c.env.JWT_SECRET, user.id);

  return c.json({
    token,
    user: {
      id: user.id,
      githubUsername: user.githubUsername,
      githubId: user.githubId,
      email: user.email,
      name: user.name,
      avatarUrl: user.avatarUrl,
      reputationScore: user.reputationScore,
      xp: user.xp,
      level: user.level,
    },
  });
});
