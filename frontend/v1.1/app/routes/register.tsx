import React, { useState } from "react";
import { useNavigate, Link } from "react-router";
import { useAuth } from "../contexts/AuthContext";

export default function Register() {
  const { register, login, user } = useAuth();
  const navigate = useNavigate();
  const [userName, setUserName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [submitting, setSubmitting] = useState(false);

  // If already logged in, redirect to home
  React.useEffect(() => {
    if (user) {
      navigate("/");
    }
  }, [user, navigate]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!userName.trim() || !email.trim() || !password.trim()) {
      setError("All fields are required.");
      return;
    }

    if (password !== confirmPassword) {
      setError("Passwords do not match.");
      return;
    }

    setError(null);
    setSubmitting(true);

    try {
      await register(userName, email, password);
      setSuccess(true);
      // Automatically log the user in after registration
      await login(userName, password);
      navigate("/");
    } catch (err: any) {
      setError(err.message || "Registration failed.");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="card card-border bg-base-100 p-6 max-w-md mx-auto font-mono">
      <h2 className="text-xl font-bold mb-4 border-b border-base-300 pb-2">
        Register an Account
      </h2>

      {error && (
        <div role="alert" className="alert alert-error text-xs mb-4">
          Error: {error}
        </div>
      )}

      {success && (
        <div role="alert" className="alert alert-success text-xs mb-4">
          Success! Account created. Logging you in...
        </div>
      )}

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <div className="flex flex-col gap-1">
          <label className="text-xs font-bold uppercase opacity-85">Username:</label>
          <input
            type="text"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
            disabled={submitting || success}
            className="input input-bordered input-sm font-sans w-full"
            required
          />
        </div>

        <div className="flex flex-col gap-1">
          <label className="text-xs font-bold uppercase opacity-85">Email Address:</label>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            disabled={submitting || success}
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
            disabled={submitting || success}
            className="input input-bordered input-sm font-sans w-full"
            required
          />
        </div>

        <div className="flex flex-col gap-1">
          <label className="text-xs font-bold uppercase opacity-85">Confirm Password:</label>
          <input
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            disabled={submitting || success}
            className="input input-bordered input-sm font-sans w-full"
            required
          />
        </div>

        <button
          type="submit"
          disabled={submitting || success}
          className="btn btn-neutral btn-sm w-full font-bold cursor-pointer disabled:opacity-50"
        >
          {submitting ? "Registering..." : "[Submit]"}
        </button>
      </form>

      <div className="mt-4 text-xs opacity-60 border-t border-base-300 pt-3 flex justify-between">
        <span>Already have an account?</span>
        <Link to="/login" className="link link-primary font-bold">
          [Login here]
        </Link>
      </div>
    </div>
  );
}
