import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router";
import { commentApi, reactionApi, type Comment } from "../utils/api";
import { useAuth } from "../contexts/AuthContext";

interface CommentNodeProps {
  comment: Comment;
  onDeleteSuccess: (commentId: number) => void;
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
        replyToType: "comment",
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
      const res = await reactionApi.getTable(comment.id, "comment");
      if (res.code === 200 && res.body.data) {
        setLikes(res.body.data.like);
        setDislikes(res.body.data.dislike);
        setUserReactionId(res.body.data.userReactionId || null);
        setUserReactionType(res.body.data.userReactionType || null);
      }
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
      const res = await commentApi.create(replyContent.trim(), comment.id, "comment");
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

  const handleChildDeleteSuccess = (childId: number) => {
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
        <div className="flex items-center justify-between text-xs pt-1">
          <div className="flex items-center gap-4">
            <button
              onClick={handleLike}
              className={`cursor-pointer hover:underline ${
                userReactionType === "like" ? "text-green-700 font-bold" : "text-gray-600"
              }`}
            >
              {userReactionType === "like" ? `[*Like* (${likes})]` : `[Like (${likes})]`}
            </button>

            <button
              onClick={handleDislike}
              className={`cursor-pointer hover:underline ${
                userReactionType === "dislike" ? "text-red-700 font-bold" : "text-gray-600"
              }`}
            >
              {userReactionType === "dislike" ? `[*Dislike* (${dislikes})]` : `[Dislike (${dislikes})]`}
            </button>

            {user && (
              <button
                onClick={() => setShowReplyForm(!showReplyForm)}
                className="text-blue-600 hover:underline cursor-pointer font-bold"
              >
                [Reply]
              </button>
            )}
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
