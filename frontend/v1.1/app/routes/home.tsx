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
      <div className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950 text-xs">
        Loading system stats...
      </div>
    );
  }

  if (!stats) return null;

  return (
    <div className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950 text-xs font-mono">
      <h3 className="font-bold border-b border-gray-200 dark:border-gray-800 pb-1.5 mb-2 uppercase text-gray-700 dark:text-gray-300">
        System Monitor
      </h3>
      <p className="mb-2">Processed Requests: <strong>{stats.requestCount}</strong></p>
      <div className="text-2xs text-gray-500 font-bold mb-1">Route Latency (us):</div>
      <ul className="list-disc pl-4 flex flex-col gap-1 text-gray-600 dark:text-gray-400">
        {Object.entries(stats.responseTimes).map(([route, times]) => {
          const avgTime = times.length > 0 ? Math.round(times.reduce((a, b) => a + b, 0) / times.length) : 0;
          return (
            <li key={route} className="truncate">
              <span className="font-bold">{route}</span>: {avgTime} us
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

  const fetchPosts = async (currentOffset: number, queryText: string) => {
    setLoading(true);
    try {
      const res = await postApi.list({
        limit,
        offset: currentOffset,
        order_by: "id",
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
        <form onSubmit={handleSearchSubmit} className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950 font-mono">
          <div className="flex gap-2">
            <input
              type="text"
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
              placeholder="Search posts..."
              className="flex-1 border border-gray-300 dark:border-gray-800 p-1.5 text-xs bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white font-sans text-gray-800 dark:text-gray-200"
            />
            <button
              type="submit"
              className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 px-3.5 py-1.5 text-xs font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer text-gray-800 dark:text-gray-200"
            >
              [Search]
            </button>
            {searchQuery && (
              <Link
                to="/"
                onClick={() => setSearchInput("")}
                className="bg-red-50 dark:bg-red-955/20 text-red-750 dark:text-red-400 border border-red-350 dark:border-red-900 px-3.5 py-1.5 text-xs font-bold hover:bg-red-100 dark:hover:bg-red-955/30 flex items-center justify-center"
              >
                [Clear]
              </Link>
            )}
          </div>
        </form>

        {/* Create Post Section (authenticated only) */}
        {user ? (
          <form onSubmit={handleCreatePost} className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950 font-mono">
            <h3 className="text-sm font-bold mb-2 uppercase tracking-wide">Write a new post</h3>
            <input
              type="text"
              value={newTitle}
              onChange={(e) => setNewTitle(e.target.value)}
              disabled={submitting}
              placeholder="Title (optional)"
              className="w-full border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white mb-3 font-sans text-gray-800 dark:text-gray-200"
            />
            <textarea
              value={newContent}
              onChange={(e) => setNewContent(e.target.value)}
              disabled={submitting}
              placeholder="What is on your mind? (Markdown-like text)"
              rows={3}
              className="w-full border border-gray-300 dark:border-gray-800 p-2 text-sm bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white mb-3 resize-y font-sans text-gray-800 dark:text-gray-200"
              required
            />
            <div className="mb-4">
              <label className="block text-xs font-bold uppercase mb-1">Attachments (optional):</label>
              <input
                id="post-files"
                type="file"
                multiple
                disabled={submitting}
                onChange={handleFileChange}
                className="w-full text-xs font-sans text-gray-700 dark:text-gray-300 file:mr-2 file:py-1 file:px-2 file:border file:border-gray-300 dark:file:border-gray-800 file:bg-gray-100 dark:file:bg-gray-900 file:text-xs file:font-mono hover:file:bg-gray-200 cursor-pointer focus:outline-none"
              />
            </div>

            {selectedFiles.length > 0 && (
              <div className="mb-4 p-2.5 bg-gray-50 dark:bg-gray-900 border border-gray-300 dark:border-gray-800">
                <span className="text-[10px] font-bold uppercase block text-gray-500 mb-2 font-mono">
                  Selected Attachments ({selectedFiles.length}):
                </span>
                <div className="flex flex-col gap-2">
                  {selectedFiles.map((item) => {
                    const isImage = item.file.type.startsWith("image/");
                    const isVideo = item.file.type.startsWith("video/");
                    return (
                      <div
                        key={item.id}
                        className="flex flex-col gap-2 border border-gray-300 dark:border-gray-800 p-2 bg-white dark:bg-gray-950 rounded-sm"
                      >
                        <div className="flex items-center justify-between text-xs font-mono">
                          <span className="truncate max-w-[80%] text-gray-700 dark:text-gray-300">
                            {item.file.name} ({Math.round(item.file.size / 1024)} KB)
                          </span>
                          <button
                            type="button"
                            onClick={() => handleRemoveFile(item.id)}
                            className="text-red-600 hover:underline font-bold"
                          >
                            [Remove]
                          </button>
                        </div>
                        {isImage && item.previewUrl && (
                          <div className="max-w-[120px]">
                            <img
                              src={item.previewUrl}
                              alt={item.file.name}
                              className="max-h-24 border border-gray-300 dark:border-gray-800 object-contain"
                            />
                          </div>
                        )}
                        {isVideo && item.previewUrl && (
                          <div className="max-w-[120px]">
                            <video
                              src={item.previewUrl}
                              controls
                              muted
                              className="max-h-24 border border-gray-300 dark:border-gray-800 object-contain"
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
                className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 px-4 py-1.5 text-xs font-bold hover:bg-gray-300 dark:hover:bg-gray-750 cursor-pointer disabled:opacity-50 text-gray-800 dark:text-gray-200"
              >
                {submitting ? "Publishing..." : "[Publish Post]"}
              </button>
            </div>
          </form>
        ) : (
          <div className="border border-gray-300 dark:border-gray-800 p-4 bg-gray-50 dark:bg-gray-900 text-sm text-center font-mono">
            Please{" "}
            <Link to="/login" className="text-blue-600 hover:underline font-bold">
              [Login]
            </Link>{" "}
            or{" "}
            <Link to="/register" className="text-blue-600 hover:underline font-bold">
              [Register]
            </Link>{" "}
            to write posts and react.
          </div>
        )}

        {error && (
          <div className="bg-red-50 text-red-700 border border-red-300 p-3 text-sm font-mono">
            Error: {error}
          </div>
        )}

        {/* Timeline List */}
        <div>
          <h2 className="text-lg font-bold border-b border-gray-300 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide font-mono">
            {searchQuery ? `Search Results for "${searchQuery}"` : "Timeline"}
          </h2>

          {loading && posts.length === 0 ? (
            <div className="text-center py-8 text-sm text-gray-500 font-mono">Loading posts...</div>
          ) : posts.length === 0 ? (
            <div className="text-center py-8 text-sm text-gray-500 font-mono">No posts found.</div>
          ) : (
            <div className="flex flex-col">
              {posts.map((post) => (
                <PostItem key={post.id} post={post} onDeleteSuccess={handleDeleteSuccess} />
              ))}

              {/* Pagination Controls */}
              <div className="flex items-center justify-between border-t border-gray-300 dark:border-gray-800 pt-4 mt-2 font-mono flex-wrap gap-4">
                <div className="flex gap-2">
                  {offset === 0 ? (
                    <span className="bg-gray-100 border border-gray-300 px-3 py-1 text-xs font-bold dark:bg-gray-900 dark:border-gray-800 opacity-30 cursor-not-allowed text-gray-400">
                      [Prev Page]
                    </span>
                  ) : (
                    <a
                      href={`/?offset=${Math.max(0, offset - limit)}${searchQuery ? `&search=${encodeURIComponent(searchQuery)}` : ""}`}
                      className="bg-gray-100 border border-gray-300 px-3 py-1 text-xs font-bold dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-200 dark:hover:bg-gray-800 text-gray-800 dark:text-gray-200 cursor-pointer"
                    >
                      [Prev Page]
                    </a>
                  )}

                  {!hasMore ? (
                    <span className="bg-gray-100 border border-gray-300 px-3 py-1 text-xs font-bold dark:bg-gray-900 dark:border-gray-800 opacity-30 cursor-not-allowed text-gray-400">
                      [Next Page]
                    </span>
                  ) : (
                    <a
                      href={`/?offset=${offset + limit}${searchQuery ? `&search=${encodeURIComponent(searchQuery)}` : ""}`}
                      className="bg-gray-100 border border-gray-300 px-3 py-1 text-xs font-bold dark:bg-gray-900 dark:border-gray-700 hover:bg-gray-200 dark:hover:bg-gray-800 text-gray-800 dark:text-gray-200 cursor-pointer"
                    >
                      [Next Page]
                    </a>
                  )}
                </div>

                {totalPages > 0 && (
                  <div className="flex items-center gap-2 text-xs text-gray-700 dark:text-gray-300">
                    <span>Page:</span>
                    <select
                      value={currentPage}
                      onChange={(e) => {
                        const pageNum = Number(e.target.value);
                        const newOffset = (pageNum - 1) * limit;
                        const searchSuffix = searchQuery ? `&search=${encodeURIComponent(searchQuery)}` : "";
                        window.location.href = `/?offset=${newOffset}${searchSuffix}`;
                      }}
                      className="border border-gray-300 dark:border-gray-800 bg-white dark:bg-gray-950 px-2 py-1 text-xs font-mono focus:outline-none text-gray-800 dark:text-gray-200"
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

                <span className="text-xs text-gray-500">
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
        <div className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950 text-xs">
          <h3 className="font-bold border-b border-gray-200 dark:border-gray-800 pb-1.5 mb-2 uppercase text-gray-700 dark:text-gray-300 font-mono">
            User Session
          </h3>
          {user ? (
            <div className="flex flex-col gap-1.5 font-mono">
              <p>Logged in as: <strong className="text-black dark:text-white">{user.userName}</strong></p>
              <p>Email: <span className="text-gray-600 dark:text-gray-400">{user.email}</span></p>
              <p>Role: <span className="font-bold text-gray-700 dark:text-gray-300">{user.isAdmin ? "Administrator" : "Standard User"}</span></p>
              <div className="mt-3 pt-2 border-t border-gray-150 dark:border-gray-900">
                <Link to="/profile" className="text-blue-600 hover:underline font-bold">[Edit Profile]</Link>
              </div>
            </div>
          ) : (
            <div className="flex flex-col gap-2 font-mono">
              <p className="text-gray-600 dark:text-gray-400">You are browsing as a guest.</p>
              <div className="flex gap-2">
                <Link to="/login" className="text-blue-600 hover:underline font-bold">[Login]</Link>
                <span className="text-gray-300 dark:text-gray-700">|</span>
                <Link to="/register" className="text-blue-600 hover:underline font-bold">[Register]</Link>
              </div>
            </div>
          )}
        </div>

        {/* System Stats Widget */}
        <SystemStatsWidget />

        {/* About Info Widget */}
        <div className="border border-gray-300 dark:border-gray-800 p-4 bg-white dark:bg-gray-950 text-xs">
          <h3 className="font-bold border-b border-gray-200 dark:border-gray-800 pb-1.5 mb-2 uppercase text-gray-700 dark:text-gray-300 font-mono">
            About AX Project
          </h3>
          <p className="text-gray-650 dark:text-gray-400 leading-relaxed mb-2 font-sans">
            AX is a minimalist microblogging site designed after traditional software directory sites. It values structural clarity and free software principles.
          </p>
          <p className="text-gray-650 dark:text-gray-400 leading-relaxed font-sans">
            It is licensed under the GNU General Public License. You are free to study, modify, and run the system.
          </p>
        </div>
      </div>
    </div>
  );
}
