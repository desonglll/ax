import React, { useEffect, useState } from "react";
import { useParams, useNavigate, Link } from "react-router";
import { userApi, postApi, type User, type Post } from "../utils/api";
import { useScrollPreservation } from "../utils/scroll";
import { useAuth } from "../contexts/AuthContext";
import { PostItem } from "../components/PostItem";

export default function Profile() {
  const { userId } = useParams<{ userId?: string }>();
  const { user: currentUser, refreshUser } = useAuth();
  const navigate = useNavigate();

  const [profileUser, setProfileUser] = useState<User | null>(null);
  const [posts, setPosts] = useState<Post[]>([]);
  const [users, setUsers] = useState<User[]>([]); // For admin list
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Edit settings form states
  const [editUserName, setEditUserName] = useState("");
  const [editEmail, setEditEmail] = useState("");
  const [editPassword, setEditPassword] = useState("");
  const [updating, setUpdating] = useState(false);
  const [updateSuccess, setUpdateSuccess] = useState(false);
  const [updateError, setUpdateError] = useState<string | null>(null);

  // Stats computed locally
  const [userPosts, setUserPosts] = useState<Post[]>([]);
  const [stats, setStats] = useState({
    postCount: 0,
    totalLikes: 0,
    totalDislikes: 0,
    averageLikes: "0.00",
    averageDislikes: "0.00",
    averageEngagementRate: "0.00",
  });

  const targetUserId = userId ? Number(userId) : currentUser?.id;

  const fetchProfileAndStats = async () => {
    setLoading(true);
    setError(null);
    try {
      if (!targetUserId) {
        // Not logged in and no userId in URL
        navigate("/login");
        return;
      }

      // 1. Fetch user detail
      let userObj: User;
      if (targetUserId === currentUser?.id) {
        const res = await userApi.profile();
        userObj = res.body.data;
      } else {
        const res = await userApi.getById(targetUserId);
        userObj = res.body.data;
      }
      setProfileUser(userObj);

      if (targetUserId === currentUser?.id) {
        setEditUserName(userObj.userName);
        setEditEmail(userObj.email);
      }

      // 2. Fetch all posts to compute local user stats & list user posts
      const postsRes = await postApi.list({ limit: 1000 });
      if (postsRes.code === 200 && postsRes.body.data) {
        const allPosts = postsRes.body.data;
        const filtered = allPosts.filter((p) => p.userId === userObj.id);
        setUserPosts(filtered);

        const postCount = filtered.length;
        const totalLikes = filtered.reduce((acc, p) => acc + (p.likeCount || 0), 0);
        const totalDislikes = filtered.reduce((acc, p) => acc + (p.dislikeCount || 0), 0);
        const avgLikes = postCount > 0 ? (totalLikes / postCount).toFixed(2) : "0.00";
        const avgDislikes = postCount > 0 ? (totalDislikes / postCount).toFixed(2) : "0.00";
        const avgEngagement =
          postCount > 0
            ? (filtered.reduce((acc, p) => acc + (p.engagementRate || 0.0), 0.0) / postCount).toFixed(2)
            : "0.00";

        setStats({
          postCount,
          totalLikes,
          totalDislikes,
          averageLikes: avgLikes,
          averageDislikes: avgDislikes,
          averageEngagementRate: avgEngagement,
        });
      }

      // 3. If current user is Admin, fetch user list for admin control
      if (currentUser?.isAdmin && targetUserId === currentUser.id) {
        const usersRes = await userApi.list();
        if (usersRes.code === 200 && usersRes.body.data) {
          setUsers(usersRes.body.data);
        }
      }
    } catch (err: any) {
      setError(err.response?.data?.message || err.message || "Failed to load profile.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchProfileAndStats();
  }, [userId, currentUser]);

  useScrollPreservation(`profile_${userId || 'self'}`, loading, !loading);

  const handleUpdateProfile = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentUser) return;

    setUpdating(true);
    setUpdateSuccess(false);
    setUpdateError(null);

    const updateData: { userName?: string; email?: string; password?: string } = {};
    if (editUserName.trim() && editUserName !== profileUser?.userName) {
      updateData.userName = editUserName.trim();
    }
    if (editEmail.trim() && editEmail !== profileUser?.email) {
      updateData.email = editEmail.trim();
    }
    if (editPassword.trim()) {
      updateData.password = editPassword;
    }

    if (Object.keys(updateData).length === 0) {
      setUpdating(false);
      return;
    }

    try {
      const res = await userApi.update(currentUser.id, updateData);
      if (res.code === 200) {
        setUpdateSuccess(true);
        setEditPassword("");
        await refreshUser();
      }
    } catch (err: any) {
      setUpdateError(err.response?.data?.message || err.message || "Failed to update profile settings.");
    } finally {
      setUpdating(false);
    }
  };

  const handleUserDelete = async (idToDelete: number) => {
    if (idToDelete === currentUser?.id) {
      alert("You cannot delete your own admin account.");
      return;
    }
    if (!confirm("Are you sure you want to delete this user? This action cannot be undone.")) {
      return;
    }
    try {
      const res = await userApi.delete(idToDelete);
      if (res.code === 200) {
        setUsers((prev) => prev.filter((u) => u.id !== idToDelete));
        alert("User deleted successfully.");
      }
    } catch (err) {
      alert("Failed to delete user.");
    }
  };

  const handlePostDeleteSuccess = (deletedId: string) => {
    setUserPosts((prev) => prev.filter((p) => p.id !== deletedId));
    // Re-evaluate stats locally
    const filtered = userPosts.filter((p) => p.id !== deletedId);
    const postCount = filtered.length;
    const totalLikes = filtered.reduce((acc, p) => acc + (p.likeCount || 0), 0);
    const totalDislikes = filtered.reduce((acc, p) => acc + (p.dislikeCount || 0), 0);
    const avgLikes = postCount > 0 ? (totalLikes / postCount).toFixed(2) : "0.00";
    const avgDislikes = postCount > 0 ? (totalDislikes / postCount).toFixed(2) : "0.00";
    const avgEngagement =
      postCount > 0
        ? (filtered.reduce((acc, p) => acc + (p.engagementRate || 0.0), 0.0) / postCount).toFixed(2)
        : "0.00";

    setStats({
      postCount,
      totalLikes,
      totalDislikes,
      averageLikes: avgLikes,
      averageDislikes: avgDislikes,
      averageEngagementRate: avgEngagement,
    });
  };

  if (loading) {
    return <div className="text-center py-12 font-mono text-sm">Loading profile...</div>;
  }

  if (error || !profileUser) {
    return (
      <div role="alert" className="alert alert-error font-mono text-sm max-w-xl mx-auto">
        <div>
          <h3 className="font-bold mb-1">Error:</h3>
          <p className="text-xs">{error || "Profile could not be loaded."}</p>
          <div className="mt-4">
            <Link to="/" className="btn btn-neutral btn-sm font-mono">
              [Back to Timeline]
            </Link>
          </div>
        </div>
      </div>
    );
  }

  const isOwnProfile = profileUser.id === currentUser?.id;
  const joinDate = profileUser.createdAt ? new Date(profileUser.createdAt).toLocaleDateString() : "Unknown";

  return (
    <div className="flex flex-col gap-8 font-mono">
      {/* Profile Overview Card */}
      <div className="card card-border bg-base-100 p-6">
        <h2 className="text-xl font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide">
          User Profile
        </h2>

        <div className="overflow-x-auto mb-6">
          <table className="table table-zebra table-sm w-full">
            <tbody>
              <tr>
                <td className="font-bold w-1/3">Username:</td>
                <td>{profileUser.userName}</td>
              </tr>
              <tr>
                <td className="font-bold">Email:</td>
                <td>{profileUser.email}</td>
              </tr>
              <tr>
                <td className="font-bold">Member Since:</td>
                <td>{joinDate}</td>
              </tr>
              <tr>
                <td className="font-bold">Role:</td>
                <td>
                  {profileUser.isAdmin ? "Site Administrator" : "Regular Member"}
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        {/* Local computed statistics using daisyui stats */}
        <h3 className="text-sm font-bold border-b border-base-300 pb-1.5 mb-3 uppercase tracking-wide">
          User Statistics
        </h3>
        <div className="stats stats-vertical sm:stats-horizontal w-full card-border bg-base-100 mb-2 font-mono">
          <div className="stat">
            <div className="stat-title text-[10px] uppercase font-bold tracking-wider">Total Posts</div>
            <div className="stat-value text-lg font-bold text-primary">{stats.postCount}</div>
          </div>
          <div className="stat">
            <div className="stat-title text-[10px] uppercase font-bold tracking-wider">Avg Likes</div>
            <div className="stat-value text-lg font-bold">{stats.averageLikes}</div>
          </div>
          <div className="stat">
            <div className="stat-title text-[10px] uppercase font-bold tracking-wider">Avg Dislikes</div>
            <div className="stat-value text-lg font-bold">{stats.averageDislikes}</div>
          </div>
        </div>
        <div className="stats stats-vertical sm:stats-horizontal w-full card-border bg-base-100 mb-2 font-mono">
          <div className="stat">
            <div className="stat-title text-[10px] uppercase font-bold tracking-wider">Total Likes</div>
            <div className="stat-value text-lg font-bold">{stats.totalLikes}</div>
          </div>
          <div className="stat">
            <div className="stat-title text-[10px] uppercase font-bold tracking-wider">Total Dislikes</div>
            <div className="stat-value text-lg font-bold">{stats.totalDislikes}</div>
          </div>
          <div className="stat">
            <div className="stat-title text-[10px] uppercase font-bold tracking-wider">Engagement Rate</div>
            <div className="stat-value text-lg font-bold text-secondary">{stats.averageEngagementRate}%</div>
          </div>
        </div>
      </div>

      {/* Edit own profile settings */}
      {isOwnProfile && (
        <div className="card card-border bg-base-100 p-6">
          <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide">
            Account Settings
          </h3>

          {updateSuccess && (
            <div role="alert" className="alert alert-success text-xs mb-4">
              Profile updated successfully.
            </div>
          )}
          {updateError && (
            <div role="alert" className="alert alert-error text-xs mb-4">
              Error: {updateError}
            </div>
          )}

          <form onSubmit={handleUpdateProfile} className="flex flex-col gap-4 max-w-sm">
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold opacity-80 uppercase">Username:</label>
              <input
                type="text"
                value={editUserName}
                onChange={(e) => setEditUserName(e.target.value)}
                disabled={updating}
                className="input input-bordered input-sm font-sans w-full"
                required
              />
            </div>
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold opacity-80 uppercase">Email Address:</label>
              <input
                type="email"
                value={editEmail}
                onChange={(e) => setEditEmail(e.target.value)}
                disabled={updating}
                className="input input-bordered input-sm font-sans w-full"
                required
              />
            </div>
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold opacity-80 uppercase">New Password (leave blank to keep unchanged):</label>
              <input
                type="password"
                value={editPassword}
                onChange={(e) => setEditPassword(e.target.value)}
                disabled={updating}
                placeholder="********"
                className="input input-bordered input-sm font-sans w-full"
              />
            </div>
            <div>
              <button
                type="submit"
                disabled={updating}
                className="btn btn-neutral btn-sm font-bold cursor-pointer disabled:opacity-50"
              >
                {updating ? "Saving..." : "[Update Profile]"}
              </button>
            </div>
          </form>
        </div>
      )}

      {/* Admin User Control Panel */}
      {currentUser?.isAdmin && isOwnProfile && (
        <div className="card card-border bg-base-100 p-6">
          <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide text-error">
            Administration Panel: Users List
          </h3>

          <div className="overflow-x-auto">
            <table className="table table-zebra table-sm w-full">
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Username</th>
                  <th>Email</th>
                  <th>Role</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {users.map((u) => (
                  <tr key={u.id}>
                    <td>{u.id}</td>
                    <td className="font-bold">
                      <Link to={`/profile/${u.id}`} className="link link-primary">
                        {u.userName}
                      </Link>
                    </td>
                    <td>{u.email}</td>
                    <td>{u.isAdmin ? <span className="badge badge-error badge-sm">Admin</span> : <span className="badge badge-neutral badge-sm">User</span>}</td>
                    <td>
                      {u.id !== currentUser.id ? (
                        <button
                          onClick={() => handleUserDelete(u.id)}
                          className="btn btn-ghost btn-xs text-error font-bold"
                        >
                          [Delete User]
                        </button>
                      ) : (
                        <span className="text-base-content/40 font-mono text-xs">(You)</span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* User's Posts list */}
      <div>
        <h3 className="text-sm font-bold border-b border-base-300 pb-2 mb-4 uppercase tracking-wide opacity-85">
          Posts by {profileUser.userName} ({userPosts.length})
        </h3>

        {userPosts.length === 0 ? (
          <div className="text-center py-6 text-xs text-gray-500 font-mono">No posts published yet.</div>
        ) : (
          <div className="flex flex-col">
            {userPosts.map((post) => (
              <PostItem key={post.id} post={post} onDeleteSuccess={handlePostDeleteSuccess} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
