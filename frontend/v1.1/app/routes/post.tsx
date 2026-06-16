import React, { useEffect, useState } from "react";
import { useParams, Link } from "react-router";
import { postApi, commentApi, type Post, type Comment } from "../utils/api";
import { useAuth } from "../contexts/AuthContext";
import { PostItem } from "../components/PostItem";
import { CommentNode } from "../components/CommentNode";

export default function PostDetail() {
  const { postId } = useParams<{ postId: string }>();
  const { user } = useAuth();
  const parsedPostId = Number(postId);

  const [post, setPost] = useState<Post | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [newCommentText, setNewCommentText] = useState("");
  const [loadingPost, setLoadingPost] = useState(true);
  const [loadingComments, setLoadingComments] = useState(true);
  const [submittingComment, setSubmittingComment] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Pagination for top-level comments
  const [offset, setOffset] = useState(0);
  const limit = 5;
  const [hasMore, setHasMore] = useState(true);

  const fetchPostDetail = async () => {
    if (isNaN(parsedPostId)) {
      setError("Invalid post ID.");
      setLoadingPost(false);
      return;
    }
    try {
      const res = await postApi.getById(parsedPostId);
      if (res.code === 200 && res.body.data) {
        setPost(res.body.data);
      } else {
        setError("Post not found.");
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to load post details.");
    } finally {
      setLoadingPost(false);
    }
  };

  const fetchComments = async (currentOffset: number) => {
    if (isNaN(parsedPostId)) return;
    setLoadingComments(true);
    try {
      const res = await commentApi.list({
        replyTo: parsedPostId,
        replyToType: "post",
        limit,
        offset: currentOffset,
      });
      if (res.code === 200 && res.body.data) {
        setComments(res.body.data);
        setHasMore(res.body.data.length === limit);
      }
    } catch (err: any) {
      console.error("Failed to load comments", err);
    } finally {
      setLoadingComments(false);
    }
  };

  useEffect(() => {
    fetchPostDetail();
  }, [parsedPostId]);

  useEffect(() => {
    fetchComments(offset);
  }, [parsedPostId, offset]);

  const handleCreateComment = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newCommentText.trim() || isNaN(parsedPostId)) return;

    setSubmittingComment(true);
    try {
      const res = await commentApi.create(newCommentText.trim(), parsedPostId, "post");
      if (res.code === 200 && res.body.data) {
        setNewCommentText("");
        // Reset to first page of comments to see the new comment
        if (offset === 0) {
          fetchComments(0);
        } else {
          setOffset(0);
        }
      }
    } catch (err: any) {
      alert(err.response?.data?.message || err.message || "Failed to submit comment.");
    } finally {
      setSubmittingComment(false);
    }
  };

  const handleCommentDeleteSuccess = (deletedId: number) => {
    setComments((prev) => prev.filter((c) => c.id !== deletedId));
  };

  const handlePostDeleteSuccess = () => {
    // Redirect to home if the post is deleted
    window.location.href = "/";
  };

  if (loadingPost) {
    return <div className="text-center py-12 font-mono text-sm">Loading post details...</div>;
  }

  if (error || !post) {
    return (
      <div className="border border-red-300 bg-red-50 text-red-700 p-6 font-mono text-sm">
        <h3 className="font-bold mb-2">Error:</h3>
        <p>{error || "Post not found."}</p>
        <div className="mt-4">
          <Link to="/" className="text-blue-600 hover:underline">
            [Back to Timeline]
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-6 font-mono">
      <div>
        <Link to="/" className="text-blue-600 hover:underline text-xs">
          &lt; [Back to Timeline]
        </Link>
      </div>

      {/* Main Post */}
      <PostItem post={post} onDeleteSuccess={handlePostDeleteSuccess} />

      {/* Comments Section */}
      <div className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950">
        <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
          Comments
        </h3>

        {/* Create Direct Comment Form */}
        {user ? (
          <form onSubmit={handleCreateComment} className="mb-6 flex flex-col gap-2">
            <textarea
              value={newCommentText}
              onChange={(e) => setNewCommentText(e.target.value)}
              disabled={submittingComment}
              placeholder="Write a comment..."
              rows={2}
              className="w-full border border-gray-300 dark:border-gray-800 p-2 text-xs bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white font-sans"
              required
            />
            <div className="flex justify-end">
              <button
                type="submit"
                disabled={submittingComment || !newCommentText.trim()}
                className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 px-3 py-1.5 text-xs font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-50"
              >
                {submittingComment ? "Submitting..." : "[Add Comment]"}
              </button>
            </div>
          </form>
        ) : (
          <div className="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-800 p-3 text-xs text-center text-gray-500 mb-6">
            Please login to participate in the conversation.
          </div>
        )}

        {/* Comments List */}
        {loadingComments && comments.length === 0 ? (
          <div className="text-center py-4 text-xs text-gray-500">Loading comments...</div>
        ) : comments.length === 0 ? (
          <div className="text-center py-6 text-xs text-gray-500">No comments yet. Be the first to comment!</div>
        ) : (
          <div className="flex flex-col gap-4">
            {comments.map((comment) => (
              <CommentNode
                key={comment.id}
                comment={comment}
                onDeleteSuccess={handleCommentDeleteSuccess}
              />
            ))}

            {/* Pagination Controls */}
            <div className="flex items-center justify-between border-t border-gray-200 dark:border-gray-800 pt-3 mt-2">
              <button
                onClick={() => setOffset((o) => Math.max(0, o - limit))}
                disabled={offset === 0}
                className="bg-gray-100 border border-gray-300 px-3 py-0.5 text-2xs font-bold dark:bg-gray-900 dark:border-gray-700 disabled:opacity-30 cursor-pointer"
              >
                [Prev Comments]
              </button>

              <span className="text-xs text-gray-500">
                Offset: {offset}
              </span>

              <button
                onClick={() => setOffset((o) => o + limit)}
                disabled={!hasMore}
                className="bg-gray-100 border border-gray-300 px-3 py-0.5 text-2xs font-bold dark:bg-gray-900 dark:border-gray-700 disabled:opacity-30 cursor-pointer"
              >
                [Next Comments]
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
