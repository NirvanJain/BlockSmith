import { SignJWT, jwtVerify } from "jose";

export interface Claims {
  sub: string;
  exp: number;
}

export async function createJWT(secret: string, userId: string): Promise<string> {
  const key = new TextEncoder().encode(secret);

  return new SignJWT({ sub: userId })
    .setProtectedHeader({ alg: "HS256" })
    .setIssuedAt()
    .setExpirationTime("7d")
    .sign(key);
}

export async function verifyJWT(secret: string, token: string): Promise<Claims> {
  const key = new TextEncoder().encode(secret);

  const { payload } = await jwtVerify(token, key, {
    algorithms: ["HS256"],
  });

  return payload as unknown as Claims;
}
