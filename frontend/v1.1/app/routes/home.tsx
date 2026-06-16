import React, { useEffect, useState } from "react";
import { postApi, type Post } from "../utils/api";
import { useAuth } from "../contexts/AuthContext";
import { PostItem } from "../components/PostItem";
import { Link } from "react-router";

export default function Home() {
  const { user } = useAuth();
  const [posts, setPosts] = useState<Post[]>([]);
  const [newContent, setNewContent] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Pagination states
  const [offset, setOffset] = useState(0);
  const limit = 10;
  const [hasMore, setHasMore] = useState(true);

  const fetchPosts = async (currentOffset: number) => {
    setLoading(true);
    try {
      const res = await postApi.list({
        limit,
        offset: currentOffset,
        order_by: "id",
        sort: "desc",
      });
      if (res.code === 200 && res.body.data) {
        setPosts(res.body.data);
        // If we received fewer items than the limit, we've reached the end
        setHasMore(res.body.data.length === limit);
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to load timeline.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPosts(offset);
  }, [offset]);

  const handleCreatePost = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newContent.trim()) return;

    setSubmitting(true);
    setError(null);

    try {
      const res = await postApi.create(newContent.trim());
      if (res.code === 200 && res.body.data) {
        setNewContent("");
        // Reset to first page to see the new post
        if (offset === 0) {
          fetchPosts(0);
        } else {
          setOffset(0);
        }
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to publish post.");
    } finally {
      setSubmitting(false);
    }
  };

  const handleDeleteSuccess = (deletedId: number) => {
    setPosts((prev) => prev.filter((p) => p.id !== deletedId));
  };

  return (
    <div className="flex flex-col gap-6 font-mono">
      {/* Create Post Section (authenticated only) */}
      {user ? (
        <form onSubmit={handleCreatePost} className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950">
          <h3 className="text-sm font-bold mb-2 uppercase tracking-wide">Write a new post</h3>
          <textarea
            value={newContent}
            onChange={(e) => setNewContent(e.target.value)}
            disabled={submitting}
            placeholder="What is on your mind? (Markdown-like text)"
            rows={3}
            className="w-full border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white mb-3 resize-y font-sans"
            required
          />
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={submitting || !newContent.trim()}
              className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 px-4 py-1.5 text-xs font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-50"
            >
              {submitting ? "Publishing..." : "[Publish Post]"}
            </button>
          </div>
        </form>
      ) : (
        <div className="border border-gray-300 dark:border-gray-800 p-4 bg-gray-50 dark:bg-gray-900 text-sm text-center">
          Please{" "}
          <Link to="/login" className="text-blue-600 hover:underline font-bold">
            [Login]
          </Link>{" "}
          or{" "}
          <Link to="/register" className="text-blue-600 hover:underline font-bold">
            [Register]
          </Link>{" "}
          to write posts and react.
        </div>
      )}

      {error && (
        <div className="bg-red-50 text-red-700 border border-red-300 p-3 text-sm">
          Error: {error}
        </div>
      )}

      {/* Timeline List */}
      <div>
        <h2 className="text-lg font-bold border-b border-gray-300 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
          Timeline
        </h2>

        {loading && posts.length === 0 ? (
          <div className="text-center py-8 text-sm text-gray-500 font-mono">Loading posts...</div>
        ) : posts.length === 0 ? (
          <div className="text-center py-8 text-sm text-gray-500 font-mono">No posts found. Write one!</div>
        ) : (
          <div className="flex flex-col">
            {posts.map((post) => (
              <PostItem key={post.id} post={post} onDeleteSuccess={handleDeleteSuccess} />
            ))}

            {/* Pagination Controls */}
            <div className="flex items-center justify-between border-t border-gray-300 dark:border-gray-800 pt-4 mt-2">
              <button
                onClick={() => setOffset((o) => Math.max(0, o - limit))}
                disabled={offset === 0}
                className="bg-gray-100 border border-gray-300 px-3 py-1 text-xs font-bold dark:bg-gray-900 dark:border-gray-700 disabled:opacity-30 cursor-pointer"
              >
                [Prev Page]
              </button>

              <span className="text-xs text-gray-500 font-mono">
                Offset: {offset}
              </span>

              <button
                onClick={() => setOffset((o) => o + limit)}
                disabled={!hasMore}
                className="bg-gray-100 border border-gray-300 px-3 py-1 text-xs font-bold dark:bg-gray-900 dark:border-gray-700 disabled:opacity-30 cursor-pointer"
              >
                [Next Page]
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
