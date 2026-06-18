import { useState, useEffect } from "react";
import {
  isRouteErrorResponse,
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
  Link,
} from "react-router";

import type { Route } from "./+types/root";
import "./app.css";
import { AuthProvider, useAuth } from "./contexts/AuthContext";

export const links: Route.LinksFunction = () => [
  { rel: "preconnect", href: "https://fonts.googleapis.com" },
  {
    rel: "preconnect",
    href: "https://fonts.gstatic.com",
    crossOrigin: "anonymous",
  },
  {
    rel: "stylesheet",
    href: "https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap",
  },
];

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body className="bg-white text-gray-900 dark:bg-gray-950 dark:text-gray-100 min-h-screen flex flex-col font-sans">
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
  return (
    <AuthProvider>
      <AppLayout />
    </AuthProvider>
  );
}

function AppLayout() {
  const { user, loading, logout } = useAuth();
  const [showScrollTop, setShowScrollTop] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      if (window.scrollY > 300) {
        setShowScrollTop(true);
      } else {
        setShowScrollTop(false);
      }
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  const scrollToTop = () => {
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  return (
    <div className="flex-1 flex flex-col bg-base-100 text-base-content min-h-screen">
      {/* Minimalist Navigation Header with daisyUI */}
      <header className="bg-base-200 border-b border-base-300 py-1">
        <div className="navbar flex-col sm:flex-row gap-3 sm:gap-0 max-w-5xl mx-auto px-4">
          <div className="navbar-start justify-center sm:justify-start w-full sm:w-auto gap-2">
            <Link to="/" className="text-xl font-bold tracking-tight hover:opacity-90">
              AX Microblog
            </Link>
            <span className="badge badge-sm font-mono opacity-60">v1.1</span>
          </div>

          <div className="navbar-end justify-center sm:justify-end w-full sm:w-auto">
            <nav className="flex flex-wrap items-center justify-center sm:justify-end gap-x-4 gap-y-2 text-sm font-medium">
              <Link to="/" className="hover:text-primary transition-colors">Timeline</Link>
              <Link to="/trending" className="hover:text-primary transition-colors">Trending</Link>
              {user?.isAdmin && <Link to="/files" className="hover:text-primary transition-colors">Files</Link>}
              <Link to="/profile" className="hover:text-primary transition-colors">Profile</Link>

              <span className="opacity-20 hidden sm:inline">|</span>

              {loading ? (
                <span className="loading loading-spinner loading-xs text-base-content/50"></span>
              ) : user ? (
                <div className="flex items-center gap-2">
                  <span className="font-mono text-sm opacity-80">{user.userName}</span>
                  <button
                    onClick={() => logout()}
                    className="btn btn-ghost btn-xs text-error font-bold"
                  >
                    [Logout]
                  </button>
                </div>
              ) : (
                <div className="flex items-center gap-1">
                  <Link to="/login" className="btn btn-ghost btn-xs text-primary font-bold">[Login]</Link>
                  <Link to="/register" className="btn btn-ghost btn-xs text-primary font-bold">[Register]</Link>
                </div>
              )}
            </nav>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <main className="flex-1 max-w-5xl w-full mx-auto p-4 py-8">
        <Outlet />
      </main>

      {/* Footer using daisyUI footer */}
      <footer className="footer footer-center p-6 bg-base-200 text-base-content/70 border-t border-base-300 font-mono text-xs mt-auto">
        <div className="max-w-5xl mx-auto flex flex-col gap-2">
          <p>
            AX Microblog is free software. You can redistribute it and/or modify it under the terms of the GNU General Public License.
          </p>
          <p>
            Project AX backend powered by Rust/Actix, frontend by React/React Router v7/Bun.
          </p>
        </div>
      </footer>

      {showScrollTop && (
        <button
          onClick={scrollToTop}
          className="btn btn-neutral btn-sm fixed bottom-6 right-6 z-50 cursor-pointer shadow-md"
        >
          ↑ Top
        </button>
      )}
    </div>
  );
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  let message = "Oops!";
  let details = "An unexpected error occurred.";
  let stack: string | undefined;

  if (isRouteErrorResponse(error)) {
    message = error.status === 404 ? "404 Not Found" : "Error";
    details =
      error.status === 404
        ? "The requested page could not be found."
        : error.statusText || details;
  } else if (import.meta.env.DEV && error && error instanceof Error) {
    details = error.message;
    stack = error.stack;
  }

  return (
    <main className="max-w-xl mx-auto p-8 py-16 text-center">
      <div className="alert alert-error mb-6 text-left">
        <div>
          <h3 className="font-bold text-lg mb-1">{message}</h3>
          <p className="text-xs opacity-90">{details}</p>
        </div>
      </div>
      <Link to="/" className="btn btn-neutral btn-sm font-mono">
        [Back to Timeline]
      </Link>
      {stack && (
        <pre className="w-full mt-8 p-4 bg-base-200 border border-base-300 text-left overflow-x-auto text-xs font-mono rounded-box">
          <code>{stack}</code>
        </pre>
      )}
    </main>
  );
}
