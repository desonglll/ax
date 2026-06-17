import React, { useEffect, useState } from "react";
import { fileApi, type FileRecord } from "../utils/api";
import { useScrollPreservation } from "../utils/scroll";
import { useAuth } from "../contexts/AuthContext";
import { Link } from "react-router";

export default function Files() {
  const { user } = useAuth();

  const [publicFiles, setPublicFiles] = useState<FileRecord[]>([]);
  const [userFiles, setUserFiles] = useState<FileRecord[]>([]);
  const [allFiles, setAllFiles] = useState<FileRecord[]>([]); // For admin

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Upload form states
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [fileType, setFileType] = useState<"public" | "private">("public");
  const [description, setDescription] = useState("");
  const [uploading, setUploading] = useState(false);
  const [uploadSuccess, setUploadSuccess] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);

  const fetchFileList = async () => {
    setLoading(true);
    try {
      // 1. Fetch public files (available to everyone)
      const pubRes = await fileApi.listPublic();
      if (pubRes.code === 200 && pubRes.body.data) {
        setPublicFiles(pubRes.body.data);
      }

      // 2. Fetch logged in user's files
      if (user) {
        const userRes = await fileApi.listByUser(user.id);
        if (userRes.code === 200 && userRes.body.data) {
          setUserFiles(userRes.body.data);
        }

        // 3. If admin, fetch all files
        if (user.isAdmin) {
          const allRes = await fileApi.listAll();
          if (allRes.code === 200 && allRes.body.data) {
            setAllFiles(allRes.body.data);
          }
        }
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to load files list.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchFileList();
  }, [user]);

  useScrollPreservation("files", loading, !loading);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      setSelectedFile(e.target.files[0]);
      setUploadSuccess(false);
      setUploadError(null);
    }
  };

  const handleUploadSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedFile) {
      setUploadError("Please select a file to upload.");
      return;
    }

    setUploading(true);
    setUploadSuccess(false);
    setUploadError(null);

    const formData = new FormData();
    formData.append("file", selectedFile);
    if (description.trim()) {
      formData.append("description", description.trim());
    }

    try {
      let res;
      if (fileType === "public") {
        res = await fileApi.uploadPublic(formData);
      } else {
        res = await fileApi.uploadPrivate(formData);
      }

      if (res.code === 200) {
        setUploadSuccess(true);
        setSelectedFile(null);
        setDescription("");
        // Reset file input element
        const fileInput = document.getElementById("file-input") as HTMLInputElement;
        if (fileInput) fileInput.value = "";
        fetchFileList();
      }
    } catch (err: any) {
      setUploadError(err.response?.data?.message || err.message || "Failed to upload file.");
    } finally {
      setUploading(false);
    }
  };

  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  return (
    <div className="flex flex-col gap-8 font-mono">
      {/* Upload File Form (authenticated only) */}
      {user ? (
        <div className="border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950">
          <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
            Upload a File
          </h3>

          {uploadSuccess && (
            <div className="bg-green-50 text-green-700 border border-green-300 p-3 mb-4 text-xs">
              Success: File uploaded and saved.
            </div>
          )}
          {uploadError && (
            <div className="bg-red-50 text-red-700 border border-red-300 p-3 mb-4 text-xs">
              Error: {uploadError}
            </div>
          )}

          <form onSubmit={handleUploadSubmit} className="flex flex-col gap-4 max-w-md">
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold">Select File:</label>
              <input
                id="file-input"
                type="file"
                onChange={handleFileChange}
                disabled={uploading}
                className="border border-gray-300 dark:border-gray-800 p-1.5 text-xs bg-gray-50 dark:bg-gray-900"
                required
              />
            </div>

            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold">Visibility Type:</label>
              <select
                value={fileType}
                onChange={(e) => setFileType(e.target.value as "public" | "private")}
                disabled={uploading}
                className="border border-gray-300 dark:border-gray-800 p-1.5 text-xs bg-gray-50 dark:bg-gray-900"
              >
                <option value="public">Public (anyone can view/download)</option>
                <option value="private">Private (only you can view/download)</option>
              </select>
            </div>

            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold">File Description (optional):</label>
              <input
                type="text"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                disabled={uploading}
                placeholder="Brief description of the file contents"
                className="border border-gray-300 dark:border-gray-800 p-1.5 text-xs bg-gray-50 dark:bg-gray-900"
              />
            </div>

            <div>
              <button
                type="submit"
                disabled={uploading || !selectedFile}
                className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 px-4 py-1.5 text-xs font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-50"
              >
                {uploading ? "Uploading..." : "[Upload File]"}
              </button>
            </div>
          </form>
        </div>
      ) : (
        <div className="border border-gray-300 dark:border-gray-800 p-4 bg-gray-50 dark:bg-gray-900 text-sm text-center">
          Please{" "}
          <Link to="/login" className="text-blue-600 hover:underline font-bold">
            [Login]
          </Link>{" "}
          to upload and manage files.
        </div>
      )}

      {error && (
        <div className="bg-red-50 text-red-700 border border-red-300 p-3 text-sm">
          Error: {error}
        </div>
      )}

      {loading ? (
        <div className="text-center py-12 text-sm text-gray-500">Loading files list...</div>
      ) : (
        <div className="flex flex-col gap-8">
          {/* Admin overall files table */}
          {user?.isAdmin && (
            <div className="border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950">
              <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide text-red-600">
                Administration: All System Files
              </h3>
              <div className="overflow-x-auto">
                <table className="w-full text-xs text-left border border-gray-300 dark:border-gray-800 border-collapse">
                  <thead>
                    <tr className="bg-gray-100 dark:bg-gray-900 border-b border-gray-300 dark:border-gray-800">
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Filename</th>
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Owner ID</th>
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Size</th>
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Type</th>
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Description</th>
                      <th className="p-2">Download</th>
                    </tr>
                  </thead>
                  <tbody>
                    {allFiles.length === 0 ? (
                      <tr>
                        <td colSpan={6} className="p-4 text-center text-gray-400">No files uploaded in the system yet.</td>
                      </tr>
                    ) : (
                      allFiles.map((file) => (
                        <tr key={file.id} className="border-b border-gray-200 dark:border-gray-900">
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850 font-bold">{file.name}</td>
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850">{file.userId}</td>
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850">{formatBytes(file.size)}</td>
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850">
                            {file.isPub ? "Public" : "Private"}
                          </td>
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850">
                            {file.description || <span className="text-gray-400 font-mono">(none)</span>}
                          </td>
                          <td className="p-2">
                            <a
                              href={fileApi.getDownloadUrl(file.id)}
                              target="_blank"
                              rel="noreferrer"
                              className="text-blue-600 hover:underline font-bold"
                            >
                              [Download]
                            </a>
                          </td>
                        </tr>
                      ))
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* User's own files table */}
          {user && (
            <div className="border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950">
              <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
                My Uploaded Files ({userFiles.length})
              </h3>
              <div className="overflow-x-auto">
                <table className="w-full text-xs text-left border border-gray-300 dark:border-gray-800 border-collapse">
                  <thead>
                    <tr className="bg-gray-100 dark:bg-gray-900 border-b border-gray-300 dark:border-gray-800">
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Filename</th>
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Size</th>
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Visibility</th>
                      <th className="p-2 border-r border-gray-300 dark:border-gray-850">Description</th>
                      <th className="p-2">Download</th>
                    </tr>
                  </thead>
                  <tbody>
                    {userFiles.length === 0 ? (
                      <tr>
                        <td colSpan={5} className="p-4 text-center text-gray-400">You haven't uploaded any files yet.</td>
                      </tr>
                    ) : (
                      userFiles.map((file) => (
                        <tr key={file.id} className="border-b border-gray-200 dark:border-gray-900">
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850 font-bold">{file.name}</td>
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850">{formatBytes(file.size)}</td>
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850">
                            {file.isPub ? "Public" : "Private"}
                          </td>
                          <td className="p-2 border-r border-gray-300 dark:border-gray-850">
                            {file.description || <span className="text-gray-400 font-mono">(none)</span>}
                          </td>
                          <td className="p-2">
                            <a
                              href={fileApi.getDownloadUrl(file.id)}
                              target="_blank"
                              rel="noreferrer"
                              className="text-blue-600 hover:underline font-bold"
                            >
                              [Download]
                            </a>
                          </td>
                        </tr>
                      ))
                    )}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* Public files table (available to everyone) */}
          <div className="border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950">
            <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
              Shared Public Files ({publicFiles.length})
            </h3>
            <div className="overflow-x-auto">
              <table className="w-full text-xs text-left border border-gray-300 dark:border-gray-800 border-collapse">
                <thead>
                  <tr className="bg-gray-100 dark:bg-gray-900 border-b border-gray-300 dark:border-gray-800">
                    <th className="p-2 border-r border-gray-300 dark:border-gray-850">Filename</th>
                    <th className="p-2 border-r border-gray-300 dark:border-gray-850">Size</th>
                    <th className="p-2 border-r border-gray-300 dark:border-gray-850">Description</th>
                    <th className="p-2">Download</th>
                  </tr>
                </thead>
                <tbody>
                  {publicFiles.length === 0 ? (
                    <tr>
                      <td colSpan={4} className="p-4 text-center text-gray-400">No public files available.</td>
                    </tr>
                  ) : (
                    publicFiles.map((file) => (
                      <tr key={file.id} className="border-b border-gray-200 dark:border-gray-900">
                        <td className="p-2 border-r border-gray-300 dark:border-gray-850 font-bold">{file.name}</td>
                        <td className="p-2 border-r border-gray-300 dark:border-gray-850">{formatBytes(file.size)}</td>
                        <td className="p-2 border-r border-gray-300 dark:border-gray-850">
                          {file.description || <span className="text-gray-400 font-mono">(none)</span>}
                        </td>
                        <td className="p-2">
                          <a
                            href={fileApi.getDownloadUrl(file.id)}
                            target="_blank"
                            rel="noreferrer"
                            className="text-blue-600 hover:underline font-bold"
                          >
                            [Download]
                          </a>
                        </td>
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
