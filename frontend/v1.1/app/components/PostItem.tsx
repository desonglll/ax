import React, { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router";
import { postApi, fileApi, reactionApi, type Post, type FileRecord } from "../utils/api";
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
  const [postAttachments, setPostAttachments] = useState<FileRecord[]>(post.attachments || []);
  const [isEditing, setIsEditing] = useState(false);
  const [editContent, setEditContent] = useState(post.content);
  const [editTitle, setEditTitle] = useState(post.title);
  const [editAttachments, setEditAttachments] = useState<FileRecord[]>(post.attachments || []);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    setPostContent(post.content);
    setEditContent(post.content);
    setPostTitle(post.title);
    setEditTitle(post.title);
    setPostAttachments(post.attachments || []);
    setEditAttachments(post.attachments || []);
  }, [post.content, post.title, post.attachments]);

  const handleSaveEdit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editContent.trim()) return;
    setSaving(true);
    try {
      const attachmentIds = editAttachments.map((a) => a.id);
      const res = await postApi.update(
        post.id,
        editContent.trim(),
        editTitle.trim(),
        attachmentIds
      );
      if (res.code === 200 && res.body.data) {
        setPostContent(editContent.trim());
        setPostTitle(editTitle.trim());
        setPostAttachments(res.body.data.attachments || []);
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
    <div className="card card-border bg-base-100 p-4 mb-4 font-mono">
      <div className="flex justify-between items-center text-xs opacity-60 border-b border-base-200 pb-2 mb-3">
        <div>
          By:{" "}
          <Link to={`/profile/${post.userId}`} className="link link-primary font-bold">
            {post.userName}
          </Link>
        </div>
        <div>{formattedDate}</div>
      </div>

      {!isDetail ? (
        <div className="mb-2">
          <Link
            to={`/posts/${post.id}`}
            className="text-sm font-bold link hover:link-primary block mb-1 font-sans"
          >
            {postTitle || "Untitled Post"}
          </Link>
        </div>
      ) : (
        <h1 className="text-lg font-bold mb-3 font-sans text-base-content">
          {postTitle || "Untitled Post"}
        </h1>
      )}

      {isEditing ? (
        <form onSubmit={handleSaveEdit} className="mt-2 font-mono flex flex-col gap-2">
          <input
            type="text"
            value={editTitle}
            onChange={(e) => setEditTitle(e.target.value)}
            disabled={saving}
            placeholder="Title (optional)"
            className="input input-bordered input-sm font-sans w-full"
          />
          <textarea
            value={editContent}
            onChange={(e) => setEditContent(e.target.value)}
            className="textarea textarea-bordered w-full font-sans text-sm resize-y"
            rows={4}
            required
          />

          {/* Edit attachments list (remove items) */}
          {editAttachments.length > 0 && (
            <div className="card card-border bg-base-200 p-3 mb-2 font-mono text-xs">
              <span className="text-[10px] font-bold uppercase block opacity-60 mb-2">
                Current Attachments ({editAttachments.length}):
              </span>
              <div className="flex flex-col gap-2">
                {editAttachments.map((file) => (
                  <div
                    key={file.id}
                    className="flex items-center justify-between border border-base-300 p-2 bg-base-100 rounded-btn"
                  >
                    <span className="truncate max-w-[80%] opacity-85">
                      {file.name} ({Math.round(file.size / 1024)} KB)
                    </span>
                    <button
                      type="button"
                      onClick={() => setEditAttachments((prev) => prev.filter((a) => a.id !== file.id))}
                      className="btn btn-ghost btn-xs text-error font-bold"
                    >
                      [Remove]
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          <div className="flex gap-2 mb-3">
            <button
              type="submit"
              disabled={saving}
              className="btn btn-neutral btn-xs cursor-pointer disabled:opacity-50"
            >
              {saving ? "Saving..." : "Save"}
            </button>
            <button
              type="button"
              onClick={() => setIsEditing(false)}
              className="btn btn-ghost btn-xs cursor-pointer"
            >
              Cancel
            </button>
          </div>
        </form>
      ) : (
        <div className="text-sm whitespace-pre-wrap break-all mb-4 text-base-content leading-relaxed font-sans">
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
                    className="ml-2 link link-primary cursor-pointer font-bold font-mono text-xs"
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
      {postAttachments && postAttachments.length > 0 && (
        <div className="border-t border-dashed border-base-300 pt-3 pb-2 mb-4">
          <span className="text-[10px] font-bold uppercase block opacity-60 mb-2 font-mono">Attachments:</span>
          <div className="flex flex-col gap-2">
            {postAttachments.map((file) => (
              <AttachmentItemRenderer key={file.id} file={file} />
            ))}
          </div>
        </div>
      )}

      <div className="flex items-center justify-between text-xs border-t border-base-200 pt-3">
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

          {!isDetail && (
            <Link
              to={`/posts/${post.id}`}
              className="btn btn-xs btn-outline btn-neutral"
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
                  setEditAttachments(postAttachments);
                  setIsEditing(true);
                }}
                className="btn btn-xs btn-outline btn-neutral cursor-pointer"
              >
                ✎ Edit
              </button>
            )}
            <button
              onClick={handleDelete}
              disabled={deleting}
              className="btn btn-xs btn-outline btn-error cursor-pointer disabled:opacity-50"
            >
              {deleting ? "Deleting..." : "✕ Delete"}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

// Helper component for rendering attachments with stateful preview toggle
export const AttachmentItemRenderer: React.FC<{ file: FileRecord }> = ({ file }) => {
  const [showPreview, setShowPreview] = useState(false);
  const isImage = file.contentType.startsWith("image/");
  const isVideo = file.contentType.startsWith("video/");
  const downloadUrl = fileApi.getDownloadUrl(file.id);

  if (!isImage && !isVideo) {
    return (
      <div className="flex items-center gap-1.5 border border-base-300 p-2 bg-base-200 rounded-btn font-mono text-xs w-full max-w-md">
        <span className="opacity-60">📄</span>
        <a
          href={downloadUrl}
          download
          className="link link-primary font-bold"
        >
          {file.name}
        </a>
        <span className="opacity-60">({Math.round(file.size / 1024)} KB)</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-start gap-2 border border-base-300 p-2 bg-base-200 rounded-btn font-mono text-xs w-full max-w-md">
      <div className="flex items-center gap-2 flex-wrap">
        <span className="opacity-60">{isImage ? "🖼️" : "🎥"}</span>
        <a
          href={downloadUrl}
          target="_blank"
          rel="noopener noreferrer"
          className="link link-primary font-bold truncate max-w-[200px]"
          title={file.name}
        >
          {file.name}
        </a>
        <span className="opacity-60">({Math.round(file.size / 1024)} KB)</span>
        <button
          type="button"
          onClick={() => setShowPreview(!showPreview)}
          className="btn btn-xs btn-neutral font-bold cursor-pointer"
        >
          {showPreview ? "[Hide Preview]" : "[Show Preview]"}
        </button>
      </div>

      {showPreview && isImage && (
        <div className="mt-1 max-w-full">
          <a href={downloadUrl} target="_blank" rel="noopener noreferrer">
            <img
              src={downloadUrl}
              alt={file.name}
              className="max-w-full max-h-96 border border-base-300 object-contain hover:opacity-95 bg-base-100"
            />
          </a>
        </div>
      )}

      {showPreview && isVideo && (
        <div className="mt-1 w-full">
          <video
            src={downloadUrl}
            controls
            className="max-w-full max-h-96 border border-base-300 object-contain bg-base-100"
          />
        </div>
      )}
    </div>
  );
};
