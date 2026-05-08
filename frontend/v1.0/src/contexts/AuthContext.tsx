import { createContext, useContext, useEffect, useState, type ReactNode } from "react";
import type { User } from "@/models/user";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";

interface AuthState {
  loggedIn: boolean;
  currentUser: User | null;
  loading: boolean;
  refresh: () => void;
}

const AuthContext = createContext<AuthState>({
  loggedIn: false,
  currentUser: null,
  loading: true,
  refresh: () => {},
});

export function AuthProvider({ children }: { children: ReactNode }) {
  const [loggedIn, setLoggedIn] = useState(false);
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  const refresh = () => {
    setLoading(true);
    getData(AxiosEndpoint.LoginCheck)
      .then((res) => {
        const ok = res?.code === 200;
        setLoggedIn(ok);
        if (ok) {
          return getData(AxiosEndpoint.Profile);
        }
        return null;
      })
      .then((res) => {
        if (res?.body?.data) setCurrentUser(res.body.data as User);
        else setCurrentUser(null);
      })
      .catch(() => {
        setLoggedIn(false);
        setCurrentUser(null);
      })
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    refresh();
  }, []);

  return (
    <AuthContext.Provider value={{ loggedIn, currentUser, loading, refresh }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  return useContext(AuthContext);
}
