import { sqliteTable, text, integer, real } from "drizzle-orm/sqlite-core";
import { sql } from "drizzle-orm";

// ===== USERS =====
export const users = sqliteTable("users", {
  id: text("id").primaryKey(), // UUID string
  githubId: text("github_id").unique(),
  githubUsername: text("github_username").unique(),
  githubAccessToken: text("github_access_token"),
  email: text("email").unique(),
  name: text("name"),
  avatarUrl: text("avatar_url"),
  reputationScore: integer("reputation_score").default(0),
  trustScore: integer("trust_score").default(0),
  totalContributions: integer("total_contributions").default(0),
  xp: integer("xp").default(0),
  level: integer("level").default(1),
  createdAt: text("created_at").default(sql`(datetime('now'))`),
  updatedAt: text("updated_at").default(sql`(datetime('now'))`),
});

// ===== PROFILES =====
export const profiles = sqliteTable("profiles", {
  userId: text("user_id")
    .primaryKey()
    .references(() => users.id, { onDelete: "cascade" }),
  bio: text("bio"),
  location: text("location"),
  website: text("website"),
  twitter: text("twitter"),
  linkedin: text("linkedin"),
  company: text("company"),
  skills: text("skills").default("[]"), // JSON array stored as text
  interests: text("interests").default("[]"), // JSON array stored as text
  updatedAt: text("updated_at").default(sql`(datetime('now'))`),
});

// ===== REPOSITORIES =====
export const repositories = sqliteTable("repositories", {
  id: text("id").primaryKey(), // UUID string
  githubId: integer("github_id").unique().notNull(),
  name: text("name").notNull(),
  owner: text("owner").notNull(),
  description: text("description"),
  language: text("language"),
  stars: integer("stars").default(0),
  forks: integer("forks").default(0),
  createdAt: text("created_at").default(sql`(datetime('now'))`),
});

// ===== ACTIVITIES =====
export const activities = sqliteTable("activities", {
  id: text("id").primaryKey(), // UUID string
  userId: text("user_id")
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
  activityType: text("activity_type").notNull(),
  repositoryId: text("repository_id").references(() => repositories.id, {
    onDelete: "set null",
  }),
  title: text("title").notNull(),
  description: text("description"),
  link: text("link"),
  metadata: text("metadata").default("{}"), // JSON stored as text
  xpEarned: integer("xp_earned").default(0),
  createdAt: text("created_at").default(sql`(datetime('now'))`),
});

// ===== ISSUES =====
export const issues = sqliteTable("issues", {
  id: text("id").primaryKey(), // UUID string
  githubId: integer("github_id").unique().notNull(),
  repositoryId: text("repository_id")
    .notNull()
    .references(() => repositories.id, { onDelete: "cascade" }),
  number: integer("number").notNull(),
  title: text("title").notNull(),
  body: text("body"),
  state: text("state").notNull(),
  labels: text("labels").default("[]"), // JSON array stored as text
  creatorUsername: text("creator_username").notNull(),
  aiComplexityScore: integer("ai_complexity_score"),
  aiMatchScore: integer("ai_match_score"),
  aiAnalysis: text("ai_analysis"),
  createdAt: text("created_at").default(sql`(datetime('now'))`),
});

// ===== BADGES =====
export const badges = sqliteTable("badges", {
  id: text("id").primaryKey(), // UUID string
  name: text("name").unique().notNull(),
  description: text("description"),
  iconUrl: text("icon_url"),
  xpRequired: integer("xp_required").default(0),
});

// ===== USER BADGES =====
export const userBadges = sqliteTable("user_badges", {
  userId: text("user_id")
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
  badgeId: text("badge_id")
    .notNull()
    .references(() => badges.id, { onDelete: "cascade" }),
  awardedAt: text("awarded_at").default(sql`(datetime('now'))`),
});

// ===== CONVERSATIONS =====
export const conversations = sqliteTable("conversations", {
  id: text("id").primaryKey(), // UUID string
  isGroup: integer("is_group", { mode: "boolean" }).default(false),
  name: text("name"),
  createdAt: text("created_at").default(sql`(datetime('now'))`),
});

// ===== CONVERSATION PARTICIPANTS =====
export const conversationParticipants = sqliteTable(
  "conversation_participants",
  {
    conversationId: text("conversation_id")
      .notNull()
      .references(() => conversations.id, { onDelete: "cascade" }),
    userId: text("user_id")
      .notNull()
      .references(() => users.id, { onDelete: "cascade" }),
  }
);

// ===== MESSAGES =====
export const messages = sqliteTable("messages", {
  id: text("id").primaryKey(), // UUID string
  conversationId: text("conversation_id")
    .notNull()
    .references(() => conversations.id, { onDelete: "cascade" }),
  senderId: text("sender_id")
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
  content: text("content").notNull(),
  createdAt: text("created_at").default(sql`(datetime('now'))`),
});

// ===== CONTRIBUTION STATS =====
export const contributionStats = sqliteTable("contribution_stats", {
  userId: text("user_id")
    .primaryKey()
    .references(() => users.id, { onDelete: "cascade" }),
  prsOpened: integer("prs_opened").default(0),
  prsMerged: integer("prs_merged").default(0),
  issuesOpened: integer("issues_opened").default(0),
  commitsPushed: integer("commits_pushed").default(0),
  starsGiven: integer("stars_given").default(0),
});

// ===== REPUTATION HISTORY =====
export const reputationHistory = sqliteTable("reputation_history", {
  id: text("id").primaryKey(), // UUID string
  userId: text("user_id")
    .notNull()
    .references(() => users.id, { onDelete: "cascade" }),
  amount: integer("amount").notNull(),
  reason: text("reason").notNull(),
  createdAt: text("created_at").default(sql`(datetime('now'))`),
});
