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
  const plClass = depth > 0 ? "pl-4 border-l border-base-300 mt-3" : "";

  return (
    <div className={`${plClass} font-mono text-sm`}>
      <div className="card card-border bg-base-200/50 p-3">
        {/* Comment Header */}
        <div className="flex justify-between items-center text-xs opacity-60 mb-2 border-b border-base-300 pb-1">
          <div>
            By:{" "}
            <Link to={`/profile/${comment.userId}`} className="link link-primary font-bold">
              {comment.userName}
            </Link>
          </div>
          <div>{formattedDate}</div>
        </div>

        {/* Comment Content */}
        <div className="text-sm text-base-content mb-3 break-all whitespace-pre-wrap">
          {comment.content}
        </div>

        {/* Comment Controls */}
        <div className="flex items-center justify-between text-xs pt-1 border-t border-base-300 mt-2">
          <div className="flex items-center gap-2">
            <button
              onClick={handleLike}
              className={`btn btn-xs cursor-pointer ${
                userReactionType === "like"
                  ? "btn-success text-success-content font-bold"
                  : "btn-outline btn-neutral"
              }`}
            >
              ▲ Like {likes}
            </button>

            <button
              onClick={handleDislike}
              className={`btn btn-xs cursor-pointer ${
                userReactionType === "dislike"
                  ? "btn-error text-error-content font-bold"
                  : "btn-outline btn-neutral"
              }`}
            >
              ▼ Dislike {dislikes}
            </button>

            {user && (
              <button
                onClick={() => setShowReplyForm(!showReplyForm)}
                className="btn btn-xs btn-outline btn-neutral cursor-pointer"
              >
                💬 Reply
              </button>
            )}
          </div>

          {isOwnerOrAdmin && (
            <button
              onClick={handleDelete}
              disabled={deleting}
              className="btn btn-xs btn-outline btn-error cursor-pointer disabled:opacity-50"
            >
              {deleting ? "Deleting..." : "✕ Delete"}
            </button>
          )}
        </div>

        {/* Reply Form */}
        {showReplyForm && (
          <form onSubmit={handleCreateReply} className="mt-3 border-t border-base-300 pt-3 flex flex-col gap-2">
            <textarea
              value={replyContent}
              onChange={(e) => setReplyContent(e.target.value)}
              disabled={submittingReply}
              placeholder={`Replying to ${comment.userName}...`}
              rows={2}
              className="textarea textarea-bordered w-full font-sans text-xs"
              required
            />
            <div className="flex justify-end gap-2">
              <button
                type="button"
                onClick={() => setShowReplyForm(false)}
                className="btn btn-ghost btn-xs font-bold cursor-pointer"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={submittingReply || !replyContent.trim()}
                className="btn btn-neutral btn-xs font-bold cursor-pointer disabled:opacity-50"
              >
                {submittingReply ? "Submitting..." : "Submit"}
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
