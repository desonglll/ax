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
    <div className="card card-border bg-base-100 p-6 max-w-md mx-auto font-mono">
      <h2 className="text-xl font-bold mb-4 border-b border-base-300 pb-2">
        Login to AX
      </h2>

      {error && (
        <div role="alert" className="alert alert-error text-xs mb-4">
          Error: {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <div className="flex flex-col gap-1">
          <label className="text-xs font-bold uppercase opacity-85">Username:</label>
          <input
            type="text"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
            disabled={submitting}
            className="input input-bordered input-sm font-sans w-full"
            required
          />
        </div>

        <div className="flex flex-col gap-1">
          <label className="text-xs font-bold uppercase opacity-85">Password:</label>
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={submitting}
            className="input input-bordered input-sm font-sans w-full"
            required
          />
        </div>

        <button
          type="submit"
          disabled={submitting}
          className="btn btn-neutral btn-sm w-full font-bold cursor-pointer disabled:opacity-50"
        >
          {submitting ? "Logging in..." : "[Submit]"}
        </button>
      </form>

      <div className="mt-4 text-xs opacity-60 border-t border-base-300 pt-3 flex justify-between">
        <span>No account?</span>
        <Link to="/register" className="link link-primary font-bold">
          [Register here]
        </Link>
      </div>
    </div>
  );
}
