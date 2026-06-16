import React, { useState } from "react";
import { useNavigate, Link } from "react-router";
import { useAuth } from "../contexts/AuthContext";

export default function Login() {
  const { login, user } = useAuth();
  const navigate = useNavigate();
  const [userName, setUserName] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  // If already logged in, redirect to home
  React.useEffect(() => {
    if (user) {
      navigate("/");
    }
  }, [user, navigate]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!userName.trim() || !password.trim()) {
      setError("Username and password are required.");
      return;
    }

    setError(null);
    setSubmitting(true);

    try {
      await login(userName, password);
      navigate("/");
    } catch (err: any) {
      setError(err.message || "Invalid credentials.");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="max-w-md mx-auto border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950 font-mono">
      <h2 className="text-xl font-bold mb-4 border-b border-gray-300 dark:border-gray-800 pb-2">
        Login to AX
      </h2>

      {error && (
        <div className="bg-red-50 text-red-700 border border-red-300 p-3 mb-4 text-sm">
          Error: {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <div className="flex flex-col gap-1">
          <label className="text-sm font-bold">Username:</label>
          <input
            type="text"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
            disabled={submitting}
            className="border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
            required
          />
        </div>

        <div className="flex flex-col gap-1">
          <label className="text-sm font-bold">Password:</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={submitting}
            className="border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
            required
          />
        </div>

        <button
          type="submit"
          disabled={submitting}
          className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 p-2 text-sm font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-50"
        >
          {submitting ? "Logging in..." : "[Submit]"}
        </button>
      </form>

      <div className="mt-4 text-xs text-gray-500 border-t border-gray-200 dark:border-gray-800 pt-3 flex justify-between">
        <span>No account?</span>
        <Link to="/register" className="text-blue-600 hover:underline">
          [Register here]
        </Link>
      </div>
    </div>
  );
}
