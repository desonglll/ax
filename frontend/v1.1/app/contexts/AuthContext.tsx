import React, { createContext, useContext, useState, useEffect } from "react";
import { authApi, userApi, type User } from "../utils/api";

interface AuthContextType {
  user: User | null;
  loading: boolean;
  login: (userName: string, password: string) => Promise<User>;
  logout: () => Promise<void>;
  register: (userName: string, email: string, password: string) => Promise<User>;
  refreshUser: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  const checkAuth = async () => {
    try {
      const res = await userApi.profile();
      if (res.code === 200 && res.body?.data) {
        setUser(res.body.data);
      } else {
        setUser(null);
      }
    } catch (error) {
      // Ignore network errors or unauthorized checks on initial load
      setUser(null);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    checkAuth();
  }, []);

  const login = async (userName: string, password: string): Promise<User> => {
    setLoading(true);
    try {
      const res = await authApi.login(userName, password);
      if (res.code === 200 && res.body.data) {
        setUser(res.body.data);
        return res.body.data;
      }
      throw new Error(res.message || "Failed to log in.");
    } catch (err: any) {
      setUser(null);
      throw new Error(err.response?.data?.message || err.message || "Failed to log in.");
    } finally {
      setLoading(false);
    }
  };

  const logout = async (): Promise<void> => {
    setLoading(true);
    try {
      await authApi.logout();
    } catch (err) {
      console.error("Logout request failed", err);
    } finally {
      setUser(null);
      setLoading(false);
    }
  };

  const register = async (userName: string, email: string, password: string): Promise<User> => {
    setLoading(true);
    try {
      const res = await userApi.register(userName, email, password);
      if (res.code === 200 && res.body.data) {
        // Log in automatically after registration if backend doesn't,
        // or just return the user record.
        return res.body.data;
      }
      throw new Error(res.message || "Failed to register.");
    } catch (err: any) {
      throw new Error(err.response?.data?.message || err.message || "Failed to register.");
    } finally {
      setLoading(false);
    }
  };

  const refreshUser = async (): Promise<void> => {
    try {
      const res = await authApi.loginCheck();
      if (res.code === 200 && res.body.data) {
        setUser(res.body.data);
      } else {
        setUser(null);
      }
    } catch (err) {
      setUser(null);
    }
  };

  return (
    <AuthContext.Provider value={{ user, loading, login, logout, register, refreshUser }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
