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
    <div className="max-w-md mx-auto border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950 font-mono">
      <h2 className="text-xl font-bold mb-4 border-b border-gray-300 dark:border-gray-800 pb-2">
        Register an Account
      </h2>

      {error && (
        <div className="bg-red-50 text-red-700 border border-red-300 p-3 mb-4 text-sm">
          Error: {error}
        </div>
      )}

      {success && (
        <div className="bg-green-50 text-green-700 border border-green-300 p-3 mb-4 text-sm">
          Success! Account created. Logging you in...
        </div>
      )}

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <div className="flex flex-col gap-1">
          <label className="text-sm font-bold">Username:</label>
          <input
            type="text"
            value={userName}
            onChange={(e) => setUserName(e.target.value)}
            disabled={submitting || success}
            className="border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
            required
          />
        </div>

        <div className="flex flex-col gap-1">
          <label className="text-sm font-bold">Email Address:</label>
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            disabled={submitting || success}
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
            disabled={submitting || success}
            className="border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
            required
          />
        </div>

        <div className="flex flex-col gap-1">
          <label className="text-sm font-bold">Confirm Password:</label>
          <input
            type="password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            disabled={submitting || success}
            className="border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
            required
          />
        </div>

        <button
          type="submit"
          disabled={submitting || success}
          className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 p-2 text-sm font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-50"
        >
          {submitting ? "Registering..." : "[Submit]"}
        </button>
      </form>

      <div className="mt-4 text-xs text-gray-500 border-t border-gray-200 dark:border-gray-800 pt-3 flex justify-between">
        <span>Already have an account?</span>
        <Link to="/login" className="text-blue-600 hover:underline">
          [Login here]
        </Link>
      </div>
    </div>
  );
}
