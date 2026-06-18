import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router";
import { postApi, fileApi, reactionApi, type Post } from "../utils/api";
import { useAuth } from "../contexts/AuthContext";

interface PostItemProps {
  post: Post;
  onDeleteSuccess?: (postId: string) => void;
  isDetail?: boolean;
}

export const PostItem: React.FC<PostItemProps> = ({ post, onDeleteSuccess, isDetail = false }) => {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [likes, setLikes] = useState<number>(0);
  const [dislikes, setDislikes] = useState<number>(0);
  const [userReactionId, setUserReactionId] = useState<number | null>(null);
  const [userReactionType, setUserReactionType] = useState<string | null>(null);
  const [deleting, setDeleting] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);

  // Inline post editing states
  const [postContent, setPostContent] = useState(post.content);
  const [postTitle, setPostTitle] = useState(post.title);
  const [isEditing, setIsEditing] = useState(false);
  const [editContent, setEditContent] = useState(post.content);
  const [editTitle, setEditTitle] = useState(post.title);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    setPostContent(post.content);
    setEditContent(post.content);
    setPostTitle(post.title);
    setEditTitle(post.title);
  }, [post.content, post.title]);

  const handleSaveEdit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editContent.trim()) return;
    setSaving(true);
    try {
      const res = await postApi.update(post.id, editContent.trim(), editTitle.trim());
      if (res.code === 200) {
        setPostContent(editContent.trim());
        setPostTitle(editTitle.trim());
        setIsEditing(false);
      }
    } catch (err) {
      alert("Failed to save post");
    } finally {
      setSaving(false);
    }
  };

  const fetchReactions = async () => {
    try {
      // 1. Fetch counts
      const res = await reactionApi.getTable(post.id, "post");
      if (res.code === 200 && res.body.data) {
        setLikes(res.body.data.like);
        setDislikes(res.body.data.dislike);
      }

      // 2. Fetch logged in user's active reaction (Like or Dislike)
      if (user) {
        const likesRes = await reactionApi.getReactions({
          toId: post.id,
          toType: "post",
          reactionName: "Like",
          userId: user.id,
        });
        if (likesRes.code === 200 && likesRes.body.data && likesRes.body.data.length > 0) {
          setUserReactionId(likesRes.body.data[0].id);
          setUserReactionType("like");
          return;
        }

        const dislikesRes = await reactionApi.getReactions({
          toId: post.id,
          toType: "post",
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

      {!isDetail ? (
        <div className="mb-2">
          <Link
            to={`/posts/${post.id}`}
            className="text-sm font-bold text-blue-600 dark:text-blue-400 hover:underline block mb-1 font-sans"
          >
            {postTitle || "Untitled Post"}
          </Link>
        </div>
      ) : (
        <h1 className="text-lg font-bold mb-3 font-sans text-gray-900 dark:text-gray-100">
          {postTitle || "Untitled Post"}
        </h1>
      )}

      {isEditing ? (
        <form onSubmit={handleSaveEdit} className="mt-2 font-mono">
          <input
            type="text"
            value={editTitle}
            onChange={(e) => setEditTitle(e.target.value)}
            disabled={saving}
            placeholder="Title (optional)"
            className="w-full border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white mb-2 font-sans text-gray-800 dark:text-gray-200"
          />
          <textarea
            value={editContent}
            onChange={(e) => setEditContent(e.target.value)}
            className="w-full border border-gray-300 dark:border-gray-800 p-2 font-sans text-sm focus:outline-none focus:border-gray-500 bg-white dark:bg-gray-950 text-gray-850 dark:text-gray-200 mb-2"
            rows={4}
            required
          />
          <div className="flex gap-2 mb-3">
            <button
              type="submit"
              disabled={saving}
              className="cursor-pointer border border-gray-300 dark:border-gray-800 px-2.5 py-1 text-xs font-mono bg-green-50 text-green-700 hover:bg-green-100 disabled:opacity-50"
            >
              {saving ? "[Saving...]" : "[Save]"}
            </button>
            <button
              type="button"
              onClick={() => setIsEditing(false)}
              className="cursor-pointer border border-gray-300 dark:border-gray-800 px-2.5 py-1 text-xs font-mono bg-gray-50 text-gray-650 hover:bg-gray-100"
            >
              [Cancel]
            </button>
          </div>
        </form>
      ) : (
        <div className="text-sm whitespace-pre-wrap break-all mb-4 text-gray-800 dark:text-gray-200 leading-relaxed font-sans">
          {(() => {
            const isLong = !isDetail && postContent.length > 280;
            const contentToShow = isLong && !isExpanded
              ? postContent.substring(0, 280) + "..."
              : postContent;
            return (
              <>
                {contentToShow}
                {isLong && (
                  <button
                    onClick={() => setIsExpanded(!isExpanded)}
                    className="ml-2 text-blue-600 hover:underline cursor-pointer font-bold font-mono text-xs"
                  >
                    {isExpanded ? "[Collapse]" : "[Read More]"}
                  </button>
                )}
              </>
            );
          })()}
        </div>
      )}

      {/* Attachments list */}
      {post.attachments && post.attachments.length > 0 && (
        <div className="border-t border-dashed border-gray-200 dark:border-gray-800 pt-3 pb-2 mb-4">
          <span className="text-[10px] font-bold uppercase block text-gray-500 mb-2 font-mono">Attachments:</span>
          <div className="flex flex-col gap-2">
            {post.attachments.map((file) => {
              const isImage = file.contentType.startsWith("image/");
              const isVideo = file.contentType.startsWith("video/");
              const downloadUrl = fileApi.getDownloadUrl(file.id);
              return (
                <div key={file.id} className="flex flex-col items-start gap-1 font-mono text-xs">
                  {isImage ? (
                    <div className="max-w-full">
                      <a href={downloadUrl} target="_blank" rel="noopener noreferrer">
                        <img
                          src={downloadUrl}
                          alt={file.name}
                          className="max-w-md max-h-96 border border-gray-300 dark:border-gray-800 object-contain hover:opacity-95"
                        />
                      </a>
                      <span className="text-[10px] text-gray-400 mt-1 block">
                        {file.name} ({Math.round(file.size / 1024)} KB)
                      </span>
                    </div>
                  ) : isVideo ? (
                    <div className="max-w-full">
                      <video
                        src={downloadUrl}
                        controls
                        className="max-w-md max-h-96 border border-gray-300 dark:border-gray-800 object-contain"
                      />
                      <span className="text-[10px] text-gray-400 mt-1 block">
                        {file.name} ({Math.round(file.size / 1024)} KB)
                      </span>
                    </div>
                  ) : (
                    <div className="flex items-center gap-1.5 border border-gray-300 dark:border-gray-800 p-2 bg-gray-50 dark:bg-gray-900 rounded-sm">
                      <span className="text-gray-500">📄</span>
                      <a
                        href={downloadUrl}
                        download
                        className="text-blue-600 hover:underline font-bold"
                      >
                        {file.name}
                      </a>
                      <span className="text-gray-400">({Math.round(file.size / 1024)} KB)</span>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      )}

      <div className="flex items-center justify-between text-xs border-t border-gray-150 dark:border-gray-900 pt-3">
        <div className="flex items-center gap-3">
          <button
            onClick={handleLike}
            className={`cursor-pointer border border-gray-300 dark:border-gray-800 px-2.5 py-1 text-xs font-mono transition-colors ${
              userReactionType === "like"
                ? "bg-green-50 text-green-700 border-green-400 font-bold dark:bg-green-950/20 dark:text-green-400 dark:border-green-800"
                : "bg-gray-50 text-gray-600 hover:bg-gray-100 hover:border-gray-400 dark:bg-gray-900 dark:text-gray-400 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-200"
            }`}
          >
            ▲ Like {likes}
          </button>

          <button
            onClick={handleDislike}
            className={`cursor-pointer border border-gray-300 dark:border-gray-800 px-2.5 py-1 text-xs font-mono transition-colors ${
              userReactionType === "dislike"
                ? "bg-red-50 text-red-700 border-red-400 font-bold dark:bg-red-950/20 dark:text-red-400 dark:border-red-800"
                : "bg-gray-50 text-gray-600 hover:bg-gray-100 hover:border-gray-400 dark:bg-gray-900 dark:text-gray-400 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-200"
            }`}
          >
            ▼ Dislike {dislikes}
          </button>

          {!isDetail && (
            <Link
              to={`/posts/${post.id}`}
              className="cursor-pointer border border-gray-300 dark:border-gray-800 px-2.5 py-1 text-xs font-mono transition-colors bg-gray-50 text-blue-600 hover:bg-blue-50 hover:text-blue-700 hover:border-blue-300 dark:bg-gray-900 dark:text-blue-400 dark:hover:bg-blue-950/20 dark:hover:border-blue-900"
            >
              💬 Comments
            </Link>
          )}
        </div>

        {isOwnerOrAdmin && (
          <div className="flex gap-2">
            {!isEditing && (
              <button
                onClick={() => {
                  setEditContent(postContent);
                  setEditTitle(postTitle);
                  setIsEditing(true);
                }}
                className="border border-gray-300 dark:border-gray-800 px-2.5 py-1 text-xs font-mono text-gray-650 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-800 cursor-pointer"
              >
                ✎ Edit
              </button>
            )}
            <button
              onClick={handleDelete}
              disabled={deleting}
              className="border border-red-200 dark:border-red-900/50 bg-red-50/50 hover:bg-red-50 dark:bg-red-950/10 px-2.5 py-1 text-xs font-mono text-red-650 hover:text-red-700 dark:text-red-400 dark:hover:bg-red-950/20 dark:hover:text-red-300 cursor-pointer disabled:opacity-50"
            >
              {deleting ? "Deleting..." : "✕ Delete"}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
