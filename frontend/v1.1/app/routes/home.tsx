import React, { useEffect, useState } from "react";
import { postApi, fileApi, getSystemStats, type Post } from "../utils/api";
import { useScrollPreservation } from "../utils/scroll";
import { useAuth } from "../contexts/AuthContext";
import { PostItem } from "../components/PostItem";
import { Link, useSearchParams } from "react-router";

function SystemStatsWidget() {
  const [stats, setStats] = useState<{ requestCount: number; responseTimes: Record<string, number[]> } | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const data = await getSystemStats();
        setStats(data);
      } catch (err) {
        console.error("Failed to load server stats", err);
      } finally {
        setLoading(false);
      }
    };
    fetchStats();
    const interval = setInterval(fetchStats, 10000);
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return (
      <div className="card card-border bg-base-100 p-4 text-xs font-mono text-center">
        <span className="loading loading-dots loading-xs"></span>
      </div>
    );
  }

  if (!stats) return null;

  return (
    <div className="card card-border bg-base-100 p-4 text-xs font-mono">
      <h3 className="font-bold border-b border-base-200 pb-1.5 mb-2 uppercase text-base-content/85">
        System Monitor
      </h3>
      <p className="mb-2">Processed Requests: <strong className="text-primary">{stats.requestCount}</strong></p>
      <div className="text-[10px] opacity-60 font-bold mb-1">Route Latency (us):</div>
      <ul className="flex flex-col gap-1.5 text-base-content/75 pl-1.5">
        {Object.entries(stats.responseTimes).map(([route, times]) => {
          const avgTime = times.length > 0 ? Math.round(times.reduce((a, b) => a + b, 0) / times.length) : 0;
          return (
            <li key={route} className="flex items-center justify-between min-w-0 text-[11px] gap-2">
              <span className="opacity-80 truncate min-w-0 flex-1" title={route}>{route}</span>
              <span className="badge badge-neutral font-bold flex-shrink-0">{avgTime} us</span>
            </li>
          );
        })}
      </ul>
    </div>
  );
}

export default function Home() {
  const { user } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const [posts, setPosts] = useState<Post[]>([]);
  const [newContent, setNewContent] = useState("");
  const [newTitle, setNewTitle] = useState("");
  interface AttachmentItem {
    id: string;
    file: File;
    previewUrl?: string;
  }
  const [selectedFiles, setSelectedFiles] = useState<AttachmentItem[]>([]);
  const [submitting, setSubmitting] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Search keyword from search parameters
  const searchQuery = searchParams.get("search") || "";
  const [searchInput, setSearchInput] = useState(searchQuery);

  // Update input text when query parameters change
  useEffect(() => {
    setSearchInput(searchQuery);
  }, [searchQuery]);

  // Pagination states bound to URL query parameter
  const offset = Number(searchParams.get("offset") || "0");
  const limit = 10;
  const [totalCount, setTotalCount] = useState(0);
  const [hasMore, setHasMore] = useState(true);

  // Track previous offset to detect page changes
  const [prevOffset, setPrevOffset] = useState(offset);
  useEffect(() => {
    if (offset !== prevOffset) {
      sessionStorage.removeItem(`scroll_position_home_${offset}_${searchQuery}`);
      window.scrollTo({ top: 0, behavior: "instant" as ScrollBehavior });
      setPrevOffset(offset);
    }
  }, [offset, prevOffset, searchQuery]);

  const fetchPosts = async (currentOffset: number, queryText: string) => {
    setLoading(true);
    try {
      const res = await postApi.list({
        limit,
        offset: currentOffset,
        order_by: "created_at",
        sort: "desc",
        search: queryText || undefined,
      });
      if (res.code === 200 && res.body.data) {
        setPosts(res.body.data);
        const countVal = res.body.pagination?.count ?? 0;
        setTotalCount(countVal);
        setHasMore(res.body.data.length === limit);
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to load timeline.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPosts(offset, searchQuery);
  }, [offset, searchQuery]);

  useScrollPreservation(`home_${offset}_${searchQuery}`, loading, posts.length > 0);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const filesArray = Array.from(e.target.files);
      const newItems = filesArray.map((file) => {
        const isImage = file.type.startsWith("image/");
        const isVideo = file.type.startsWith("video/");
        return {
          id: Math.random().toString(36).substring(2, 9),
          file,
          previewUrl: (isImage || isVideo) ? URL.createObjectURL(file) : undefined,
        };
      });
      setSelectedFiles((prev) => [...prev, ...newItems]);
    }
    e.target.value = "";
  };

  const handleRemoveFile = (id: string) => {
    setSelectedFiles((prev) => {
      const target = prev.find((item) => item.id === id);
      if (target?.previewUrl) {
        URL.revokeObjectURL(target.previewUrl);
      }
      return prev.filter((item) => item.id !== id);
    });
  };

  const handleCreatePost = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newContent.trim()) return;

    setSubmitting(true);
    setError(null);

    try {
      let attachmentIds: string[] = [];
      if (selectedFiles.length > 0) {
        const formData = new FormData();
        for (let i = 0; i < selectedFiles.length; i++) {
          formData.append("file", selectedFiles[i].file);
        }
        const uploadRes = await fileApi.uploadPublic(formData);
        if (uploadRes.code === 200 && uploadRes.body.data) {
          attachmentIds = uploadRes.body.data.map((file) => file.id);
        } else {
          throw new Error("Failed to upload attachments.");
        }
      }

      const res = await postApi.create(newContent.trim(), newTitle.trim(), attachmentIds);
      if (res.code === 200 && res.body.data) {
        setNewContent("");
        setNewTitle("");
        
        // Clean up object URLs to prevent leaks
        selectedFiles.forEach((item) => {
          if (item.previewUrl) URL.revokeObjectURL(item.previewUrl);
        });
        setSelectedFiles([]);

        const fileInput = document.getElementById("post-files") as HTMLInputElement;
        if (fileInput) fileInput.value = "";

        // Reset to first page to see the new post
        if (offset === 0 && !searchQuery) {
          fetchPosts(0, "");
        } else {
          setSearchParams({ offset: "0" });
        }
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to publish post.");
    } finally {
      setSubmitting(false);
    }
  };

  const handleSearchSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const cleanSearch = searchInput.trim();
    if (cleanSearch) {
      setSearchParams({ offset: "0", search: cleanSearch });
    } else {
      setSearchParams({ offset: "0" });
    }
  };

  const handleDeleteSuccess = (deletedId: string) => {
    setPosts((prev) => prev.filter((p) => p.id !== deletedId));
  };

  // Pagination details
  const totalPages = Math.ceil(totalCount / limit);
  const currentPage = Math.floor(offset / limit) + 1;

  return (
    <div className="grid grid-cols-1 md:grid-cols-12 gap-6">
      {/* Left Column: Timeline & Editor */}
      <div className="md:col-span-8 flex flex-col gap-6">
        {/* Search Bar Form */}
        <form onSubmit={handleSearchSubmit} className="card card-border bg-base-100 p-4 font-mono">
          <div className="flex gap-2">
            <input
              type="text"
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
              placeholder="Search posts..."
              className="input input-bordered input-sm flex-1 font-sans text-xs"
            />
            <button
              type="submit"
              className="btn btn-neutral btn-sm"
            >
              Search
            </button>
            {searchQuery && (
              <Link
                to="/"
                onClick={() => setSearchInput("")}
                className="btn btn-ghost btn-sm text-error"
              >
                Clear
              </Link>
            )}
          </div>
        </form>

        {/* Create Post Section (authenticated only) */}
        {user ? (
          <form onSubmit={handleCreatePost} className="card card-border bg-base-100 p-4 font-mono">
            <h3 className="text-sm font-bold mb-3 uppercase tracking-wide">Write a new post</h3>
            <input
              type="text"
              value={newTitle}
              onChange={(e) => setNewTitle(e.target.value)}
              disabled={submitting}
              placeholder="Title (optional)"
              className="input input-bordered input-sm w-full font-sans mb-3 text-xs"
            />
            <textarea
              value={newContent}
              onChange={(e) => setNewContent(e.target.value)}
              disabled={submitting}
              placeholder="What is on your mind? (Markdown-like text)"
              rows={3}
              className="textarea textarea-bordered w-full font-sans mb-3 text-xs resize-y"
              required
            />
            <div className="mb-4">
              <label className="block text-xs font-bold uppercase mb-1 opacity-70">Attachments (optional):</label>
              <input
                id="post-files"
                type="file"
                multiple
                disabled={submitting}
                onChange={handleFileChange}
                className="file-input file-input-bordered file-input-sm w-full text-xs font-sans"
              />
            </div>

            {selectedFiles.length > 0 && (
              <div className="mb-4 p-3 bg-base-200 border border-base-300 rounded-box">
                <span className="text-[10px] font-bold uppercase block text-base-content/60 mb-2 font-mono">
                  Selected Attachments ({selectedFiles.length}):
                </span>
                <div className="flex flex-col gap-2">
                  {selectedFiles.map((item) => {
                    const isImage = item.file.type.startsWith("image/");
                    const isVideo = item.file.type.startsWith("video/");
                    return (
                      <div
                        key={item.id}
                        className="flex flex-col gap-2 border border-base-300 p-2 bg-base-100 rounded-btn"
                      >
                        <div className="flex items-center justify-between text-xs font-mono">
                          <span className="truncate max-w-[80%] opacity-85">
                            {item.file.name} ({Math.round(item.file.size / 1024)} KB)
                          </span>
                          <button
                            type="button"
                            onClick={() => handleRemoveFile(item.id)}
                            className="btn btn-ghost btn-xs text-error font-bold"
                          >
                            [Remove]
                          </button>
                        </div>
                        {isImage && item.previewUrl && (
                          <div className="max-w-[120px]">
                            <img
                              src={item.previewUrl}
                              alt={item.file.name}
                              className="max-h-24 border border-base-300 object-contain rounded-btn"
                            />
                          </div>
                        )}
                        {isVideo && item.previewUrl && (
                          <div className="max-w-[120px]">
                            <video
                              src={item.previewUrl}
                              controls
                              muted
                              className="max-h-24 border border-base-300 object-contain rounded-btn"
                            />
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              </div>
            )}
            <div className="flex justify-end">
              <button
                type="submit"
                disabled={submitting || !newContent.trim()}
                className="btn btn-primary btn-sm"
              >
                {submitting ? "Publishing..." : "Publish Post"}
              </button>
            </div>
          </form>
        ) : (
          <div className="card bg-base-200 border border-base-300 p-4 text-sm text-center font-mono rounded-box">
            Please{" "}
            <Link to="/login" className="text-primary hover:underline font-bold">
              [Login]
            </Link>{" "}
            or{" "}
            <Link to="/register" className="text-primary hover:underline font-bold">
              [Register]
            </Link>{" "}
            to write posts and react.
          </div>
        )}

        {error && (
          <div className="alert alert-error text-xs font-mono">
            Error: {error}
          </div>
        )}

        {/* Timeline List */}
        <div>
          <h2 className="text-lg font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide font-mono opacity-85">
            {searchQuery ? `Search Results for "${searchQuery}"` : "Timeline"}
          </h2>

          {loading && posts.length === 0 ? (
            <div className="text-center py-8 text-sm opacity-50 font-mono">Loading posts...</div>
          ) : posts.length === 0 ? (
            <div className="text-center py-8 text-sm opacity-50 font-mono">No posts found.</div>
          ) : (
            <div className="flex flex-col gap-4">
              {posts.map((post) => (
                <PostItem key={post.id} post={post} onDeleteSuccess={handleDeleteSuccess} />
              ))}

              {/* Pagination Controls */}
              <div className="flex items-center justify-between border-t border-base-300 pt-4 mt-2 font-mono flex-wrap gap-4">
                <div className="join">
                  {offset === 0 ? (
                    <span className="join-item btn btn-outline btn-sm btn-disabled opacity-50">
                      Prev Page
                    </span>
                  ) : (
                    <Link
                      to={`/?offset=${Math.max(0, offset - limit)}${searchQuery ? `&search=${encodeURIComponent(searchQuery)}` : ""}`}
                      className="join-item btn btn-outline btn-sm"
                    >
                      Prev Page
                    </Link>
                  )}

                  {!hasMore ? (
                    <span className="join-item btn btn-outline btn-sm btn-disabled opacity-50">
                      Next Page
                    </span>
                  ) : (
                    <Link
                      to={`/?offset=${offset + limit}${searchQuery ? `&search=${encodeURIComponent(searchQuery)}` : ""}`}
                      className="join-item btn btn-outline btn-sm"
                    >
                      Next Page
                    </Link>
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
                        const nextParams = new URLSearchParams(searchParams);
                        nextParams.set("offset", newOffset.toString());
                        setSearchParams(nextParams);
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

      {/* Right Column: Sidebar */}
      <div className="md:col-span-4 flex flex-col gap-6">
        {/* User Session Widget */}
        <div className="card card-border bg-base-100 p-4 text-xs font-mono">
          <h3 className="font-bold border-b border-base-200 pb-1.5 mb-2 uppercase text-base-content/85">
            User Session
          </h3>
          {user ? (
            <div className="flex flex-col gap-1.5">
              <p>Logged in as: <strong className="text-base-content">{user.userName}</strong></p>
              <p>Email: <span className="opacity-70">{user.email}</span></p>
              <p>Role: <span className="font-bold text-primary">{user.isAdmin ? "Administrator" : "Standard User"}</span></p>
              <div className="mt-3 pt-2 border-t border-base-200">
                <Link to="/profile" className="btn btn-link btn-xs p-0 text-primary font-bold min-h-0 h-auto">Edit Profile</Link>
              </div>
            </div>
          ) : (
            <div className="flex flex-col gap-2">
              <p className="opacity-70">You are browsing as a guest.</p>
              <div className="flex gap-2">
                <Link to="/login" className="btn btn-link btn-xs p-0 text-primary font-bold min-h-0 h-auto">Login</Link>
                <span className="opacity-30">|</span>
                <Link to="/register" className="btn btn-link btn-xs p-0 text-primary font-bold min-h-0 h-auto">Register</Link>
              </div>
            </div>
          )}
        </div>

        {/* System Stats Widget */}
        <SystemStatsWidget />

        {/* About Info Widget */}
        <div className="card card-border bg-base-100 p-4 text-xs">
          <h3 className="font-bold border-b border-base-200 pb-1.5 mb-2 uppercase text-base-content/85 font-mono">
            About AX Project
          </h3>
          <p className="opacity-70 leading-relaxed mb-2 font-sans">
            AX is a minimalist microblogging site designed after traditional software directory sites. It values structural clarity and free software principles.
          </p>
          <p className="opacity-70 leading-relaxed font-sans">
            It is licensed under the GNU General Public License. You are free to study, modify, and run the system.
          </p>
        </div>
      </div>
    </div>
  );
}
