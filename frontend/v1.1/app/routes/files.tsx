import React, { useEffect, useState } from "react";
import { fileApi, type FileRecord } from "../utils/api";
import { useScrollPreservation } from "../utils/scroll";
import { useAuth } from "../contexts/AuthContext";
import { Link } from "react-router";

export default function Files() {
  const { user } = useAuth();

  if (!user || !user.isAdmin) {
    return (
      <div className="max-w-5xl mx-auto p-4 font-mono">
        <div role="alert" className="alert alert-error text-center p-6">
          <div>
            <h2 className="font-bold mb-2 uppercase text-sm">Access Denied</h2>
            <p className="text-xs mb-4">You do not have permission to view or manage files.</p>
            <Link to="/" className="btn btn-neutral btn-sm font-mono">[Back to Timeline]</Link>
          </div>
        </div>
      </div>
    );
  }

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
        <div className="card card-border bg-base-100 p-6">
          <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide">
            Upload a File
          </h3>

          {uploadSuccess && (
            <div role="alert" className="alert alert-success text-xs mb-4">
              Success: File uploaded and saved.
            </div>
          )}
          {uploadError && (
            <div role="alert" className="alert alert-error text-xs mb-4">
              Error: {uploadError}
            </div>
          )}

          <form onSubmit={handleUploadSubmit} className="flex flex-col gap-4 max-w-md">
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold opacity-80 uppercase">Select File:</label>
              <input
                id="file-input"
                type="file"
                onChange={handleFileChange}
                disabled={uploading}
                className="file-input file-input-bordered file-input-sm w-full font-sans"
                required
              />
            </div>

            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold opacity-80 uppercase">Visibility Type:</label>
              <select
                value={fileType}
                onChange={(e) => setFileType(e.target.value as "public" | "private")}
                disabled={uploading}
                className="select select-bordered select-sm font-mono w-full"
              >
                <option value="public">Public (anyone can view/download)</option>
                <option value="private">Private (only you can view/download)</option>
              </select>
            </div>

            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold opacity-80 uppercase">File Description (optional):</label>
              <input
                type="text"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                disabled={uploading}
                placeholder="Brief description of the file contents"
                className="input input-bordered input-sm font-sans w-full"
              />
            </div>

            <div>
              <button
                type="submit"
                disabled={uploading || !selectedFile}
                className="btn btn-neutral btn-sm font-bold cursor-pointer disabled:opacity-50"
              >
                {uploading ? "Uploading..." : "[Upload File]"}
              </button>
            </div>
          </form>
        </div>
      ) : (
        <div className="card bg-base-200 border border-base-300 p-4 text-sm text-center font-mono rounded-box">
          Please{" "}
          <Link to="/login" className="link link-primary font-bold">
            [Login]
          </Link>{" "}
          to upload and manage files.
        </div>
      )}

      {error && (
        <div role="alert" className="alert alert-error text-xs">
          Error: {error}
        </div>
      )}

      {loading ? (
        <div className="text-center py-12 text-sm opacity-50 font-mono">Loading files list...</div>
      ) : (
        <div className="flex flex-col gap-8">
          {/* Admin overall files table */}
          {user?.isAdmin && (
            <div className="card card-border bg-base-100 p-6">
              <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide text-error">
                Administration: All System Files
              </h3>
              <div className="overflow-x-auto">
                <table className="table table-zebra table-sm w-full">
                  <thead>
                    <tr>
                      <th>Filename</th>
                      <th>Owner ID</th>
                      <th>Size</th>
                      <th>Type</th>
                      <th>Description</th>
                      <th>Download</th>
                    </tr>
                  </thead>
                  <tbody>
                    {allFiles.length === 0 ? (
                      <tr>
                        <td colSpan={6} className="text-center text-base-content/40">No files uploaded in the system yet.</td>
                      </tr>
                    ) : (
                      allFiles.map((file) => (
                        <tr key={file.id}>
                          <td className="font-bold">{file.name}</td>
                          <td>{file.userId}</td>
                          <td>{formatBytes(file.size)}</td>
                          <td>
                            {file.isPub ? <span className="badge badge-neutral badge-xs">Public</span> : <span className="badge badge-ghost badge-xs">Private</span>}
                          </td>
                          <td>
                            {file.description || <span className="text-base-content/40 font-mono text-2xs">(none)</span>}
                          </td>
                          <td>
                            <a
                              href={fileApi.getDownloadUrl(file.id)}
                              target="_blank"
                              rel="noreferrer"
                              className="link link-primary font-bold"
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
            <div className="card card-border bg-base-100 p-6">
              <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide">
                My Uploaded Files ({userFiles.length})
              </h3>
              <div className="overflow-x-auto">
                <table className="table table-zebra table-sm w-full">
                  <thead>
                    <tr>
                      <th>Filename</th>
                      <th>Size</th>
                      <th>Visibility</th>
                      <th>Description</th>
                      <th>Download</th>
                    </tr>
                  </thead>
                  <tbody>
                    {userFiles.length === 0 ? (
                      <tr>
                        <td colSpan={5} className="text-center text-base-content/40">You haven't uploaded any files yet.</td>
                      </tr>
                    ) : (
                      userFiles.map((file) => (
                        <tr key={file.id}>
                          <td className="font-bold">{file.name}</td>
                          <td>{formatBytes(file.size)}</td>
                          <td>
                            {file.isPub ? <span className="badge badge-neutral badge-xs">Public</span> : <span className="badge badge-ghost badge-xs">Private</span>}
                          </td>
                          <td>
                            {file.description || <span className="text-base-content/40 font-mono text-2xs">(none)</span>}
                          </td>
                          <td>
                            <a
                              href={fileApi.getDownloadUrl(file.id)}
                              target="_blank"
                              rel="noreferrer"
                              className="link link-primary font-bold"
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
          <div className="card card-border bg-base-100 p-6">
            <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide">
              Shared Public Files ({publicFiles.length})
            </h3>
            <div className="overflow-x-auto">
              <table className="table table-zebra table-sm w-full">
                <thead>
                  <tr>
                    <th>Filename</th>
                    <th>Size</th>
                    <th>Description</th>
                    <th>Download</th>
                  </tr>
                </thead>
                <tbody>
                  {publicFiles.length === 0 ? (
                    <tr>
                      <td colSpan={4} className="text-center text-base-content/40">No public files available.</td>
                    </tr>
                  ) : (
                    publicFiles.map((file) => (
                      <tr key={file.id}>
                        <td className="font-bold">{file.name}</td>
                        <td>{formatBytes(file.size)}</td>
                        <td>
                          {file.description || <span className="text-base-content/40 font-mono text-2xs">(none)</span>}
                        </td>
                        <td>
                          <a
                            href={fileApi.getDownloadUrl(file.id)}
                            target="_blank"
                            rel="noreferrer"
                            className="link link-primary font-bold"
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
