import { drizzle } from "drizzle-orm/d1";
import * as schema from "./schema";

export type DB = ReturnType<typeof createDB>;

export function createDB(binding: D1Database) {
  return drizzle(binding, { schema });
}
