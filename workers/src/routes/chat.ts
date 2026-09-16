import { Hono } from "hono";
import { createDB } from "../db";
import {
  conversations,
  conversationParticipants,
  messages,
  users,
} from "../db/schema";
import { eq, and } from "drizzle-orm";
import { authMiddleware } from "../middleware/auth";
import type { Env, Variables } from "../types";

export const chatRoutes = new Hono<{ Bindings: Env; Variables: Variables }>();

// Apply auth middleware to all chat routes
chatRoutes.use("*", authMiddleware);

// GET /api/v1/chat/conversations
chatRoutes.get("/conversations", async (c) => {
  const userId = c.get("userId");
  const db = createDB(c.env.DB);

  // Get conversations where user is a participant
  const participantConvos = await db
    .select({ conversationId: conversationParticipants.conversationId })
    .from(conversationParticipants)
    .where(eq(conversationParticipants.userId, userId))
    .all();

  const conversationIds = participantConvos.map((p) => p.conversationId);

  if (conversationIds.length === 0) {
    return c.json([]);
  }

  // Get conversations
  const convos = await db.select().from(conversations).all();

  const filteredConvos = convos.filter((conv) =>
    conversationIds.includes(conv.id)
  );

  // Get participants for each conversation
  const result = await Promise.all(
    filteredConvos.map(async (conv) => {
      const participants = await db
        .select({
          userId: users.id,
          name: users.name,
          githubUsername: users.githubUsername,
          avatarUrl: users.avatarUrl,
        })
        .from(conversationParticipants)
        .innerJoin(users, eq(conversationParticipants.userId, users.id))
        .where(eq(conversationParticipants.conversationId, conv.id))
        .all();

      return {
        id: conv.id,
        is_group: conv.isGroup,
        name: conv.name,
        participants: participants.map((p) => ({
          user_id: p.userId,
          name: p.name,
          github_username: p.githubUsername,
          avatar_url: p.avatarUrl,
        })),
        created_at: conv.createdAt,
      };
    })
  );

  return c.json(result);
});

// POST /api/v1/chat/conversations
chatRoutes.post("/conversations", async (c) => {
  const userId = c.get("userId");
  const body = await c.req.json<{ recipient_username: string }>();
  const db = createDB(c.env.DB);

  // Find recipient
  const recipient = await db
    .select()
    .from(users)
    .where(eq(users.githubUsername, body.recipient_username))
    .get();

  if (!recipient) {
    return c.json({ error: "Recipient not found" }, 404);
  }

  // Check for existing DM
  const userConvos = await db
    .select({ conversationId: conversationParticipants.conversationId })
    .from(conversationParticipants)
    .where(eq(conversationParticipants.userId, userId))
    .all();

  for (const uc of userConvos) {
    const recipientParticipation = await db
      .select()
      .from(conversationParticipants)
      .where(
        and(
          eq(conversationParticipants.conversationId, uc.conversationId),
          eq(conversationParticipants.userId, recipient.id)
        )
      )
      .get();

    if (recipientParticipation) {
      return c.json({ id: uc.conversationId });
    }
  }

  // Create new conversation
  const conversationId = crypto.randomUUID();
  await db.insert(conversations).values({
    id: conversationId,
    isGroup: false,
    createdAt: new Date().toISOString(),
  });

  // Add participants
  await db.insert(conversationParticipants).values([
    { conversationId, userId },
    { conversationId, userId: recipient.id },
  ]);

  return c.json({ id: conversationId });
});

// GET /api/v1/chat/conversations/:id/messages
chatRoutes.get("/conversations/:id/messages", async (c) => {
  const conversationId = c.req.param("id");
  const db = createDB(c.env.DB);

  const msgs = await db
    .select({
      id: messages.id,
      senderId: messages.senderId,
      content: messages.content,
      createdAt: messages.createdAt,
      senderName: users.name,
      senderAvatar: users.avatarUrl,
    })
    .from(messages)
    .leftJoin(users, eq(messages.senderId, users.id))
    .where(eq(messages.conversationId, conversationId))
    .all();

  return c.json(
    msgs.map((m) => ({
      id: m.id,
      sender_id: m.senderId,
      sender_name: m.senderName,
      sender_avatar: m.senderAvatar,
      content: m.content,
      created_at: m.createdAt,
    }))
  );
});
