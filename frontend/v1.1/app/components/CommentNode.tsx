import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router";
import { commentApi, reactionApi, type Comment } from "../utils/api";
import { useAuth } from "../contexts/AuthContext";

interface CommentNodeProps {
  comment: Comment;
  onDeleteSuccess: (commentId: string) => void;
  depth?: number;
}

export const CommentNode: React.FC<CommentNodeProps> = ({ comment, onDeleteSuccess, depth = 0 }) => {
  const { user } = useAuth();
  const navigate = useNavigate();

  // Child comments (replies)
  const [replies, setReplies] = useState<Comment[]>([]);
  const [showReplyForm, setShowReplyForm] = useState(false);
  const [replyContent, setReplyContent] = useState("");
  const [submittingReply, setSubmittingReply] = useState(false);

  // Reaction states
  const [likes, setLikes] = useState(0);
  const [dislikes, setDislikes] = useState(0);
  const [userReactionId, setUserReactionId] = useState<number | null>(null);
  const [userReactionType, setUserReactionType] = useState<string | null>(null);

  const [deleting, setDeleting] = useState(false);

  const fetchReplies = async () => {
    try {
      const res = await commentApi.list({
        replyTo: comment.id,
      });
      if (res.code === 200 && res.body.data) {
        setReplies(res.body.data);
      }
    } catch (err) {
      console.error("Failed to load replies for comment", comment.id, err);
    }
  };

  const fetchReactions = async () => {
    try {
      // 1. Fetch counts
      const res = await reactionApi.getTable(comment.id, "comment");
      if (res.code === 200 && res.body.data) {
        setLikes(res.body.data.like);
        setDislikes(res.body.data.dislike);
      }

      // 2. Fetch logged in user's active reaction (Like or Dislike)
      if (user) {
        const likesRes = await reactionApi.getReactions({
          toId: comment.id,
          toType: "comment",
          reactionName: "Like",
          userId: user.id,
        });
        if (likesRes.code === 200 && likesRes.body.data && likesRes.body.data.length > 0) {
          setUserReactionId(likesRes.body.data[0].id);
          setUserReactionType("like");
          return;
        }

        const dislikesRes = await reactionApi.getReactions({
          toId: comment.id,
          toType: "comment",
          reactionName: "Dislike",
          userId: user.id,
        });
        if (dislikesRes.code === 200 && dislikesRes.body.data && dislikesRes.body.data.length > 0) {
          setUserReactionId(dislikesRes.body.data[0].id);
          setUserReactionType("dislike");
          return;
        }
      }

      // Fallback if no reaction
      setUserReactionId(null);
      setUserReactionType(null);
    } catch (err) {
      console.error("Failed to load reactions for comment", comment.id, err);
    }
  };

  useEffect(() => {
    fetchReplies();
    fetchReactions();
  }, [comment.id]);

  const handleLike = async () => {
    if (!user) {
      navigate("/login");
      return;
    }
    try {
      if (userReactionType === "like" && userReactionId) {
        await reactionApi.delete(userReactionId);
      } else {
        await reactionApi.like(comment.id, "comment");
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
        await reactionApi.dislike(comment.id, "comment");
      }
      fetchReactions();
    } catch (err) {
      console.error("Failed to toggle dislike", err);
    }
  };

  const handleCreateReply = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!replyContent.trim()) return;

    setSubmittingReply(true);
    try {
      const res = await commentApi.create(replyContent.trim(), comment.id);
      if (res.code === 200 && res.body.data) {
        setReplyContent("");
        setShowReplyForm(false);
        fetchReplies();
      }
    } catch (err) {
      alert("Failed to submit reply");
    } finally {
      setSubmittingReply(false);
    }
  };

  const handleDelete = async () => {
    if (!confirm("Are you sure you want to delete this comment?")) {
      return;
    }
    setDeleting(true);
    try {
      const res = await commentApi.delete(comment.id);
      if (res.code === 200) {
        onDeleteSuccess(comment.id);
      }
    } catch (err) {
      alert("Failed to delete comment");
    } finally {
      setDeleting(false);
    }
  };

  const handleChildDeleteSuccess = (childId: string) => {
    setReplies((prev) => prev.filter((r) => r.id !== childId));
  };

  const isOwnerOrAdmin = user && (user.id === comment.userId || user.isAdmin);
  const formattedDate = new Date(comment.createdAt).toLocaleString();

  // Limit nesting depth visual offset to prevent rendering too far right
  const maxDepthOffset = 4;
  const plClass = depth > 0 ? "pl-4 border-l border-gray-300 dark:border-gray-800 mt-3" : "";

  return (
    <div className={`${plClass} font-mono text-sm`}>
      <div className="bg-gray-50 dark:bg-gray-950 border border-gray-200 dark:border-gray-900 p-3">
        {/* Comment Header */}
        <div className="flex justify-between items-center text-xs text-gray-500 mb-2 border-b border-gray-100 dark:border-gray-900 pb-1">
          <div>
            By:{" "}
            <Link to={`/profile/${comment.userId}`} className="text-blue-600 hover:underline font-bold">
              {comment.userName}
            </Link>
          </div>
          <div>{formattedDate}</div>
        </div>

        {/* Comment Content */}
        <div className="text-sm text-gray-800 dark:text-gray-200 mb-3 break-all whitespace-pre-wrap">
          {comment.content}
        </div>

        {/* Comment Controls */}
        <div className="flex items-center justify-between text-xs pt-1 border-t border-gray-100 dark:border-gray-900 mt-2">
          <div className="flex items-center gap-3">
            <button
              onClick={handleLike}
              className={`cursor-pointer border border-gray-300 dark:border-gray-800 px-2 py-0.5 text-xs font-mono transition-colors ${
                userReactionType === "like"
                  ? "bg-green-50 text-green-700 border-green-400 font-bold dark:bg-green-950/20 dark:text-green-400 dark:border-green-800"
                  : "bg-gray-50 text-gray-650 hover:bg-gray-100 hover:border-gray-400 dark:bg-gray-900 dark:text-gray-400 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-200"
              }`}
            >
              ▲ Like {likes}
            </button>

            <button
              onClick={handleDislike}
              className={`cursor-pointer border border-gray-300 dark:border-gray-800 px-2 py-0.5 text-xs font-mono transition-colors ${
                userReactionType === "dislike"
                  ? "bg-red-50 text-red-700 border-red-400 font-bold dark:bg-red-950/20 dark:text-red-400 dark:border-red-800"
                  : "bg-gray-50 text-gray-650 hover:bg-gray-100 hover:border-gray-400 dark:bg-gray-900 dark:text-gray-400 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-200"
              }`}
            >
              ▼ Dislike {dislikes}
            </button>

            {user && (
              <button
                onClick={() => setShowReplyForm(!showReplyForm)}
                className="cursor-pointer border border-gray-300 dark:border-gray-800 px-2 py-0.5 text-xs font-mono transition-colors bg-gray-50 text-blue-600 hover:bg-blue-50 hover:text-blue-700 hover:border-blue-300 dark:bg-gray-900 dark:text-blue-400 dark:hover:bg-blue-950/20 dark:hover:border-blue-900"
              >
                💬 Reply
              </button>
            )}
          </div>

          {isOwnerOrAdmin && (
            <button
              onClick={handleDelete}
              disabled={deleting}
              className="border border-red-200 dark:border-red-900/50 bg-red-50/50 hover:bg-red-50 dark:bg-red-950/10 px-2 py-0.5 text-xs font-mono text-red-600 hover:text-red-700 dark:text-red-400 dark:hover:bg-red-950/20 dark:hover:text-red-300 cursor-pointer disabled:opacity-50"
            >
              {deleting ? "Deleting..." : "✕ Delete"}
            </button>
          )}
        </div>

        {/* Reply Form */}
        {showReplyForm && (
          <form onSubmit={handleCreateReply} className="mt-3 border-t border-gray-200 dark:border-gray-800 pt-3 flex flex-col gap-2">
            <textarea
              value={replyContent}
              onChange={(e) => setReplyContent(e.target.value)}
              disabled={submittingReply}
              placeholder={`Replying to ${comment.userName}...`}
              rows={2}
              className="w-full border border-gray-300 dark:border-gray-850 p-1.5 text-xs bg-white dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white font-sans"
              required
            />
            <div className="flex justify-end gap-2">
              <button
                type="button"
                onClick={() => setShowReplyForm(false)}
                className="border border-gray-300 px-2.5 py-1 text-2xs font-bold hover:bg-gray-100 cursor-pointer"
              >
                [Cancel]
              </button>
              <button
                type="submit"
                disabled={submittingReply || !replyContent.trim()}
                className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 px-3 py-1 text-2xs font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-50"
              >
                {submittingReply ? "Submitting..." : "[Submit]"}
              </button>
            </div>
          </form>
        )}
      </div>

      {/* Render Nested Replies */}
      {replies.length > 0 && (
        <div className="flex flex-col">
          {replies.map((reply) => (
            <CommentNode
              key={reply.id}
              comment={reply}
              onDeleteSuccess={handleChildDeleteSuccess}
              depth={Math.min(depth + 1, maxDepthOffset)}
            />
          ))}
        </div>
      )}
    </div>
  );
};
