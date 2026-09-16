import { createMiddleware } from "hono/factory";
import { verifyJWT } from "../auth/jwt";
import type { Env, Variables } from "../types";

export const authMiddleware = createMiddleware<{ Bindings: Env; Variables: Variables }>(
  async (c, next) => {
    const authHeader = c.req.header("Authorization");
    if (!authHeader?.startsWith("Bearer ")) {
      return c.json({ error: "Missing or invalid Authorization header" }, 401);
    }

    const token = authHeader.slice(7);
    try {
      const claims = await verifyJWT(c.env.JWT_SECRET, token);
      c.set("userId", claims.sub);
      await next();
    } catch {
      return c.json({ error: "Invalid token" }, 401);
    }
  }
);
