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

  return (
    <div className="flex-1 flex flex-col">
      {/* GNU-Style Minimalist Navigation Header */}
      <header className="bg-gray-100 border-b border-gray-300 dark:bg-gray-900 dark:border-gray-800 py-3 px-4">
        <div className="max-w-5xl mx-auto flex flex-col sm:flex-row items-center justify-between gap-4">
          <div className="flex items-center gap-2">
            <Link to="/" className="text-xl font-bold tracking-tight text-black dark:text-white hover:underline">
              AX Microblog
            </Link>
            <span className="text-xs text-gray-500 font-mono">v1.1</span>
          </div>

          <nav className="flex items-center gap-4 text-sm font-medium">
            <Link to="/" className="hover:underline">Timeline</Link>
            <Link to="/trending" className="hover:underline">Trending</Link>
            <Link to="/files" className="hover:underline">Files</Link>
            <Link to="/profile" className="hover:underline">Profile</Link>

            <span className="text-gray-300 dark:text-gray-700">|</span>

            {loading ? (
              <span className="text-xs text-gray-500 font-mono">Loading...</span>
            ) : user ? (
              <div className="flex items-center gap-2">
                <span className="text-gray-700 dark:text-gray-300">
                  {user.userName}
                </span>
                <button
                  onClick={() => logout()}
                  className="text-red-600 hover:text-red-700 hover:underline cursor-pointer font-bold"
                >
                  [Logout]
                </button>
              </div>
            ) : (
              <div className="flex items-center gap-2">
                <Link to="/login" className="text-blue-600 hover:underline font-bold">[Login]</Link>
                <Link to="/register" className="text-blue-600 hover:underline font-bold">[Register]</Link>
              </div>
            )}
          </nav>
        </div>
      </header>

      {/* Main Content Area */}
      <main className="flex-1 max-w-5xl w-full mx-auto p-4 py-8">
        <Outlet />
      </main>

      {/* GNU-Style Minimalist Footer */}
      <footer className="bg-gray-50 border-t border-gray-200 dark:bg-gray-950 dark:border-gray-900 py-6 px-4 text-center text-xs text-gray-500 font-mono mt-auto">
        <div className="max-w-5xl mx-auto flex flex-col gap-2">
          <p>
            AX Microblog is free software. You can redistribute it and/or modify it under the terms of the GNU General Public License.
          </p>
          <p>
            Project AX backend powered by Rust/Actix, frontend by React/React Router v7/Bun.
          </p>
        </div>
      </footer>
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
    <main className="max-w-5xl mx-auto p-8 py-16 text-center">
      <h1 className="text-2xl font-bold text-red-600 mb-4">{message}</h1>
      <p className="text-gray-700 dark:text-gray-300 mb-6">{details}</p>
      <Link to="/" className="text-blue-600 hover:underline font-mono">
        [Back to Timeline]
      </Link>
      {stack && (
        <pre className="w-full mt-8 p-4 bg-gray-100 dark:bg-gray-900 border border-gray-300 dark:border-gray-800 text-left overflow-x-auto text-xs font-mono">
          <code>{stack}</code>
        </pre>
      )}
    </main>
  );
}
