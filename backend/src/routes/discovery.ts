import { Hono } from "hono";
import { createDB } from "../db";
import { issues, repositories } from "../db/schema";
import { desc, eq } from "drizzle-orm";
import type { Env } from "../types";

export const discoveryRoutes = new Hono<{ Bindings: Env }>();

// GET /api/v1/discovery
discoveryRoutes.get("/", async (c) => {
  const db = createDB(c.env.DB);

  // Get recent issues with repository info
  const discoveryItems = await db
    .select({
      issueId: issues.id,
      title: issues.title,
      body: issues.body,
      state: issues.state,
      labels: issues.labels,
      creatorUsername: issues.creatorUsername,
      aiComplexityScore: issues.aiComplexityScore,
      aiMatchScore: issues.aiMatchScore,
      aiAnalysis: issues.aiAnalysis,
      repositoryName: repositories.name,
      repositoryOwner: repositories.owner,
      createdAt: issues.createdAt,
    })
    .from(issues)
    .leftJoin(repositories, eq(issues.repositoryId, repositories.id))
    .orderBy(desc(issues.createdAt))
    .limit(50)
    .all();

  if (discoveryItems.length === 0) {
    // Return mock data
    return c.json([
      {
        id: "iss_1",
        number: 42,
        title: "Optimize performance of Merkle tree generation",
        body: "The current merkle tree implementation is single-threaded. We should parallelize it using Rayon.",
        state: "open",
        labels: ["performance", "rust"],
        creator_username: "octocat",
        repo: {
          name: "BlockSmith",
          owner: "NirvanJain",
        },
        ai_complexity_score: 8,
        ai_match_score: 95,
        ai_analysis: "Highly matches your profile due to your experience with Rust and performance tuning.",
      },
      {
        id: "iss_2",
        number: 105,
        title: "Add Clerk authentication middleware to Axum backend",
        body: "We need to authenticate user endpoints using Clerk JWKS key validation.",
        state: "open",
        labels: ["good first issue", "backend", "auth"],
        creator_username: "NirvanJain",
        repo: {
          name: "BlockSmith",
          owner: "NirvanJain",
        },
        ai_complexity_score: 4,
        ai_match_score: 82,
        ai_analysis: "Excellent choice for a developer focused on API building and backend architecture.",
      },
    ]);
  }

  return c.json(
    discoveryItems.map((item) => {
      const labels = item.labels ? JSON.parse(item.labels) : [];
      return {
        id: item.issueId,
        title: item.title,
        body: item.body,
        state: item.state,
        labels,
        creator_username: item.creatorUsername,
        repo: {
          name: item.repositoryName,
          owner: item.repositoryOwner,
        },
        ai_complexity_score: item.aiComplexityScore ?? 5,
        ai_match_score: item.aiMatchScore ?? 75,
        ai_analysis: item.aiAnalysis ?? "Complexity score fits your technical skills.",
      };
    })
  );
});
