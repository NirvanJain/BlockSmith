import { createContext, useContext, useState, useEffect, useCallback, type ReactNode } from "react";

// ─── Types ─────────────────────────────────────────────────
export interface GitHubUser {
  id: number;
  login: string;         // github username
  name: string | null;
  avatar_url: string;
  bio: string | null;
  public_repos: number;
  followers: number;
}

export interface AuthState {
  user: GitHubUser | null;
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

export interface AuthContextValue extends AuthState {
  loginWithGitHub: () => void;
  logout: () => void;
  handleOAuthCallback: (code: string) => Promise<void>;
}

// ─── Context ────────────────────────────────────────────────
const AuthContext = createContext<AuthContextValue | null>(null);

// ─── Mock user for dev / demo ────────────────────────────────
const MOCK_USER: GitHubUser = {
  id: 1,
  login: "nirvanjain",
  name: "Nirvan Jain",
  avatar_url: "https://avatars.githubusercontent.com/u/1?v=4",
  bio: "Building BlockSmith 🔗",
  public_repos: 42,
  followers: 128,
};

const STORAGE_KEY = "bs_auth";

// ─── Provider ───────────────────────────────────────────────
export function AuthProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<AuthState>({
    user: null,
    token: null,
    isAuthenticated: false,
    isLoading: true,
  });

  // Rehydrate from localStorage on mount
  useEffect(() => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored);
        if (parsed.token && parsed.user) {
          setState({
            user: parsed.user,
            token: parsed.token,
            isAuthenticated: true,
            isLoading: false,
          });
          return;
        }
      }
    } catch {
      localStorage.removeItem(STORAGE_KEY);
    }
    setState((s) => ({ ...s, isLoading: false }));
  }, []);

  // Check for OAuth callback code in URL
  useEffect(() => {
    const url = new URL(window.location.href);
    const code = url.searchParams.get("code");
    if (code) {
      // Clear the code from URL immediately
      window.history.replaceState({}, "", window.location.pathname);
      handleOAuthCallback(code);
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const loginWithGitHub = useCallback(() => {
    const useMock = import.meta.env.VITE_USE_MOCK_AUTH === "true" || !import.meta.env.VITE_GITHUB_CLIENT_ID;

    if (useMock) {
      // Mock login — instantly authenticate with demo user
      const token = "mock_jwt_" + Math.random().toString(36).slice(2);
      const authData = { user: MOCK_USER, token };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(authData));
      setState({
        user: MOCK_USER,
        token,
        isAuthenticated: true,
        isLoading: false,
      });
      return;
    }

    // Real GitHub OAuth flow
    const clientId = import.meta.env.VITE_GITHUB_CLIENT_ID;
    const redirectUri = encodeURIComponent(window.location.origin + "/auth/callback");
    const scope = encodeURIComponent("read:user user:email");
    const state = Math.random().toString(36).slice(2);
    sessionStorage.setItem("oauth_state", state);

    window.location.href = `https://github.com/login/oauth/authorize?client_id=${clientId}&redirect_uri=${redirectUri}&scope=${scope}&state=${state}`;
  }, []);

  const handleOAuthCallback = useCallback(async (code: string) => {
    setState((s) => ({ ...s, isLoading: true }));
    try {
      // Exchange code via backend
      const backendUrl = import.meta.env.VITE_BACKEND_URL || "http://localhost:3000";
      const res = await fetch(`${backendUrl}/api/v1/auth/github/callback?code=${code}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
      });

      if (!res.ok) throw new Error("Auth failed");

      const data = await res.json();
      const { token, user } = data;

      localStorage.setItem(STORAGE_KEY, JSON.stringify({ token, user }));
      setState({ user, token, isAuthenticated: true, isLoading: false });
    } catch {
      // Fallback to mock on error (dev convenience)
      console.warn("OAuth callback failed — using mock auth");
      const token = "mock_jwt_fallback_" + Math.random().toString(36).slice(2);
      const authData = { user: MOCK_USER, token };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(authData));
      setState({ user: MOCK_USER, token, isAuthenticated: true, isLoading: false });
    }
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem(STORAGE_KEY);
    setState({ user: null, token: null, isAuthenticated: false, isLoading: false });
  }, []);

  return (
    <AuthContext.Provider value={{ ...state, loginWithGitHub, logout, handleOAuthCallback }}>
      {children}
    </AuthContext.Provider>
  );
}

// ─── Hook ────────────────────────────────────────────────────
export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
