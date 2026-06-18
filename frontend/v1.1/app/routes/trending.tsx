import React, { useEffect, useState } from "react";
import { postApi, type Post } from "../utils/api";
import { useScrollPreservation } from "../utils/scroll";
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

  useScrollPreservation("trending", loading, posts.length > 0);

  const handleDeleteSuccess = (deletedId: string) => {
    setPosts((prev) => prev.filter((p) => p.id !== deletedId));
  };

  return (
    <div className="flex flex-col gap-4 font-mono">
      <h2 className="text-lg font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide opacity-85">
        Trending / Recommended Posts
      </h2>

      {error && (
        <div role="alert" className="alert alert-error text-xs mb-4">
          Error: {error}
        </div>
      )}

      {loading ? (
        <div className="text-center py-8 text-sm opacity-50 font-mono">Loading trending posts...</div>
      ) : posts.length === 0 ? (
        <div className="text-center py-8 text-sm opacity-50 font-mono">No trending posts found.</div>
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
