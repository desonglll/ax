import React, { useEffect, useState } from "react";
import { useParams, Link, useSearchParams } from "react-router";
import { postApi, commentApi, type Post, type Comment } from "../utils/api";
import { useScrollPreservation } from "../utils/scroll";
import { useAuth } from "../contexts/AuthContext";
import { PostItem } from "../components/PostItem";
import { CommentNode } from "../components/CommentNode";

export default function PostDetail() {
  const { postId } = useParams<{ postId: string }>();
  const { user } = useAuth();
  const parsedPostId = postId || "";

  const [post, setPost] = useState<Post | null>(null);
  const [comments, setComments] = useState<Comment[]>([]);
  const [newCommentText, setNewCommentText] = useState("");
  const [loadingPost, setLoadingPost] = useState(true);
  const [loadingComments, setLoadingComments] = useState(true);
  const [submittingComment, setSubmittingComment] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Pagination for top-level comments bound to URL search parameter
  const [searchParams, setSearchParams] = useSearchParams();
  const offset = Number(searchParams.get("offset") || "0");
  const limit = 5;
  const [totalComments, setTotalComments] = useState(0);
  const [hasMore, setHasMore] = useState(true);

  const fetchPostDetail = async () => {
    if (!parsedPostId) {
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
    if (!parsedPostId) return;
    setLoadingComments(true);
    try {
      const res = await commentApi.list({
        replyTo: parsedPostId,
        limit,
        offset: currentOffset,
      });
      if (res.code === 200 && res.body.data) {
        setComments(res.body.data);
        const countVal = res.body.pagination?.count ?? 0;
        setTotalComments(countVal);
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

  useScrollPreservation(`post_${parsedPostId}_${offset}`, loadingPost || loadingComments, !!post);

  const handleCreateComment = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newCommentText.trim() || !parsedPostId) return;

    setSubmittingComment(true);
    try {
      const res = await commentApi.create(newCommentText.trim(), parsedPostId);
      if (res.code === 200 && res.body.data) {
        setNewCommentText("");
        // Reset to first page of comments to see the new comment
        if (offset === 0) {
          fetchComments(0);
        } else {
          setSearchParams({ offset: "0" });
        }
      }
    } catch (err: any) {
      alert(err.response?.data?.message || err.message || "Failed to submit comment.");
    } finally {
      setSubmittingComment(false);
    }
  };

  const handleCommentDeleteSuccess = (deletedId: string) => {
    setComments((prev) => prev.filter((c) => c.id !== deletedId));
  };

  const handlePostDeleteSuccess = () => {
    // Redirect to home if the post is deleted
    window.location.href = "/";
  };

  if (error || !post) {
    return (
      <div role="alert" className="alert alert-error font-mono text-sm max-w-xl mx-auto">
        <div>
          <h3 className="font-bold mb-1">Error:</h3>
          <p className="text-xs">{error || "Post not found."}</p>
          <div className="mt-4">
            <Link to="/" className="btn btn-neutral btn-sm font-mono">
              [Back to Timeline]
            </Link>
          </div>
        </div>
      </div>
    );
  }

  const totalPages = Math.ceil(totalComments / limit);
  const currentPage = Math.floor(offset / limit) + 1;

  return (
    <div className="flex flex-col gap-6 font-mono">
      <div>
        <Link to="/" className="link link-primary text-xs font-bold">
          &lt; [Back to Timeline]
        </Link>
      </div>

      {/* Main Post */}
      <PostItem post={post} onDeleteSuccess={handlePostDeleteSuccess} isDetail={true} />

      {/* Comments Section */}
      <div className="card card-border bg-base-100 p-4">
        <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide">
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
              className="textarea textarea-bordered w-full font-sans text-xs"
              required
            />
            <div className="flex justify-end">
              <button
                type="submit"
                disabled={submittingComment || !newCommentText.trim()}
                className="btn btn-neutral btn-sm font-bold cursor-pointer disabled:opacity-50"
              >
                {submittingComment ? "Submitting..." : "[Add Comment]"}
              </button>
            </div>
          </form>
        ) : (
          <div className="bg-base-200 border border-base-300 p-3 text-xs text-center text-base-content/60 mb-6 rounded-btn">
            Please login to participate in the conversation.
          </div>
        )}

        {/* Comments List */}
        {loadingComments && comments.length === 0 ? (
          <div className="text-center py-4 text-xs opacity-50">Loading comments...</div>
        ) : comments.length === 0 ? (
          <div className="text-center py-6 text-xs opacity-50">No comments yet. Be the first to comment!</div>
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
            <div className="flex items-center justify-between border-t border-base-300 pt-3 mt-2 flex-wrap gap-4">
              <div className="join">
                {offset === 0 ? (
                  <span className="join-item btn btn-outline btn-xs btn-disabled opacity-50">
                    Prev Comments
                  </span>
                ) : (
                  <a
                    href={`/posts/${post.id}?offset=${Math.max(0, offset - limit)}`}
                    className="join-item btn btn-outline btn-xs"
                  >
                    Prev Comments
                  </a>
                )}

                {!hasMore ? (
                  <span className="join-item btn btn-outline btn-xs btn-disabled opacity-50">
                    Next Comments
                  </span>
                ) : (
                  <a
                    href={`/posts/${post.id}?offset=${offset + limit}`}
                    className="join-item btn btn-outline btn-xs"
                  >
                    Next Comments
                  </a>
                )}
              </div>

              {totalPages > 0 && (
                <div className="flex items-center gap-2 text-xs opacity-80">
                  <span>Page:</span>
                  <select
                    value={currentPage}
                    onChange={(e) => {
                      const pageNum = Number(e.target.value);
                      const newOffset = (pageNum - 1) * limit;
                      window.location.href = `/posts/${post.id}?offset=${newOffset}`;
                    }}
                    className="select select-bordered select-xs font-mono"
                  >
                    {Array.from({ length: totalPages }, (_, i) => i + 1).map((pNum) => (
                      <option key={pNum} value={pNum}>
                        {pNum}
                      </option>
                    ))}
                  </select>
                  <span>of {totalPages}</span>
                </div>
              )}

              <span className="text-xs opacity-50">
                Offset: {offset}
              </span>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
