import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router";
import { postApi, reactionApi, type Post } from "../utils/api";
import { useAuth } from "../contexts/AuthContext";

interface PostItemProps {
  post: Post;
  onDeleteSuccess?: (postId: number) => void;
}

export const PostItem: React.FC<PostItemProps> = ({ post, onDeleteSuccess }) => {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [likes, setLikes] = useState<number>(0);
  const [dislikes, setDislikes] = useState<number>(0);
  const [userReactionId, setUserReactionId] = useState<number | null>(null);
  const [userReactionType, setUserReactionType] = useState<string | null>(null);
  const [deleting, setDeleting] = useState(false);

  const fetchReactions = async () => {
    try {
      const res = await reactionApi.getTable(post.id, "post");
      if (res.code === 200 && res.body.data) {
        setLikes(res.body.data.like);
        setDislikes(res.body.data.dislike);
        setUserReactionId(res.body.data.userReactionId || null);
        setUserReactionType(res.body.data.userReactionType || null);
      }
    } catch (err) {
      console.error("Failed to load reactions for post", post.id, err);
    }
  };

  useEffect(() => {
    fetchReactions();
  }, [post.id]);

  const handleLike = async () => {
    if (!user) {
      navigate("/login");
      return;
    }
    try {
      if (userReactionType === "like" && userReactionId) {
        await reactionApi.delete(userReactionId);
      } else {
        await reactionApi.like(post.id, "post");
      }
      fetchReactions();
    } catch (err) {
      console.error("Failed to toggle like", err);
    }
  };

  const handleDislike = async () => {
    if (!user) {
      navigate("/login");
      return;
    }
    try {
      if (userReactionType === "dislike" && userReactionId) {
        await reactionApi.delete(userReactionId);
      } else {
        await reactionApi.dislike(post.id, "post");
      }
      fetchReactions();
    } catch (err) {
      console.error("Failed to toggle dislike", err);
    }
  };

  const handleDelete = async () => {
    if (!confirm("Are you sure you want to delete this post?")) {
      return;
    }
    setDeleting(true);
    try {
      const res = await postApi.delete(post.id);
      if (res.code === 200) {
        if (onDeleteSuccess) {
          onDeleteSuccess(post.id);
        } else {
          // Fallback refresh or redirect
          window.location.reload();
        }
      }
    } catch (err) {
      alert("Failed to delete post");
    } finally {
      setDeleting(false);
    }
  };

  const formattedDate = new Date(post.createdAt).toLocaleString();
  const isOwnerOrAdmin = user && (user.id === post.userId || user.isAdmin);

  return (
    <div className="border border-gray-300 dark:border-gray-800 p-4 mb-4 bg-white dark:bg-gray-950 font-mono">
      <div className="flex justify-between items-center text-xs text-gray-500 border-b border-gray-200 dark:border-gray-800 pb-2 mb-3">
        <div>
          By:{" "}
          <Link to={`/profile/${post.userId}`} className="text-blue-600 hover:underline font-bold">
            {post.userName}
          </Link>
        </div>
        <div>{formattedDate}</div>
      </div>

      <div className="text-sm whitespace-pre-wrap break-all mb-4 text-gray-800 dark:text-gray-200 leading-relaxed">
        {post.content}
      </div>

      <div className="flex items-center justify-between text-xs border-t border-gray-100 dark:border-gray-900 pt-3">
        <div className="flex items-center gap-4">
          <button
            onClick={handleLike}
            className={`cursor-pointer hover:underline ${
              userReactionType === "like" ? "text-green-700 font-bold" : "text-gray-600 dark:text-gray-400"
            }`}
          >
            {userReactionType === "like" ? `[*Like* (${likes})]` : `[Like (${likes})]`}
          </button>

          <button
            onClick={handleDislike}
            className={`cursor-pointer hover:underline ${
              userReactionType === "dislike" ? "text-red-700 font-bold" : "text-gray-600 dark:text-gray-400"
            }`}
          >
            {userReactionType === "dislike" ? `[*Dislike* (${dislikes})]` : `[Dislike (${dislikes})]`}
          </button>

          <Link to={`/posts/${post.id}`} className="text-blue-600 hover:underline">
            [Comments]
          </Link>
        </div>

        {isOwnerOrAdmin && (
          <button
            onClick={handleDelete}
            disabled={deleting}
            className="text-red-600 hover:underline font-bold cursor-pointer disabled:opacity-50"
          >
            {deleting ? "[Deleting...]" : "[Delete]"}
          </button>
        )}
      </div>
    </div>
  );
};
