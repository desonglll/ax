import React, { useEffect, useState } from "react";
import { postApi, type Post } from "../utils/api";
import { PostItem } from "../components/PostItem";

export default function Trending() {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTrending = async () => {
    setLoading(true);
    try {
      const res = await postApi.trending();
      if (res.code === 200 && res.body.data) {
        setPosts(res.body.data);
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to load trending posts.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTrending();
  }, []);

  const handleDeleteSuccess = (deletedId: number) => {
    setPosts((prev) => prev.filter((p) => p.id !== deletedId));
  };

  return (
    <div className="flex flex-col gap-4 font-mono">
      <h2 className="text-lg font-bold border-b border-gray-300 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
        Trending / Recommended Posts
      </h2>

      {error && (
        <div className="bg-red-50 text-red-700 border border-red-300 p-3 text-sm">
          Error: {error}
        </div>
      )}

      {loading ? (
        <div className="text-center py-8 text-sm text-gray-500 font-mono">Loading trending posts...</div>
      ) : posts.length === 0 ? (
        <div className="text-center py-8 text-sm text-gray-500 font-mono">No trending posts found.</div>
      ) : (
        <div className="flex flex-col">
          {posts.map((post) => (
            <PostItem key={post.id} post={post} onDeleteSuccess={handleDeleteSuccess} />
          ))}
        </div>
      )}
    </div>
  );
}
