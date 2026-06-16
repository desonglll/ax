import React, { useEffect, useState } from "react";
import { useParams, useNavigate, Link } from "react-router";
import { userApi, postApi, type User, type Post } from "../utils/api";
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

  const handlePostDeleteSuccess = (deletedId: number) => {
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
      <div className="border border-red-300 bg-red-50 text-red-700 p-6 font-mono text-sm">
        <h3 className="font-bold mb-2">Error:</h3>
        <p>{error || "Profile could not be loaded."}</p>
        <div className="mt-4">
          <Link to="/" className="text-blue-600 hover:underline">
            [Back to Timeline]
          </Link>
        </div>
      </div>
    );
  }

  const isOwnProfile = profileUser.id === currentUser?.id;
  const joinDate = profileUser.createdAt ? new Date(profileUser.createdAt).toLocaleDateString() : "Unknown";

  return (
    <div className="flex flex-col gap-8 font-mono">
      {/* Profile Overview Card */}
      <div className="border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950">
        <h2 className="text-xl font-bold border-b border-gray-300 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
          User Profile
        </h2>

        <table className="w-full text-sm border-collapse text-left mb-6">
          <tbody>
            <tr className="border-b border-gray-200 dark:border-gray-900">
              <td className="py-2 font-bold w-1/3">Username:</td>
              <td className="py-2 text-gray-700 dark:text-gray-300">{profileUser.userName}</td>
            </tr>
            <tr className="border-b border-gray-200 dark:border-gray-900">
              <td className="py-2 font-bold">Email:</td>
              <td className="py-2 text-gray-700 dark:text-gray-300">{profileUser.email}</td>
            </tr>
            <tr className="border-b border-gray-200 dark:border-gray-900">
              <td className="py-2 font-bold">Member Since:</td>
              <td className="py-2 text-gray-700 dark:text-gray-300">{joinDate}</td>
            </tr>
            <tr>
              <td className="py-2 font-bold">Role:</td>
              <td className="py-2 text-gray-700 dark:text-gray-300">
                {profileUser.isAdmin ? "Site Administrator" : "Regular Member"}
              </td>
            </tr>
          </tbody>
        </table>

        {/* Local computed statistics */}
        <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-1.5 mb-3 uppercase tracking-wide">
          User Statistics
        </h3>
        <div className="grid grid-cols-2 sm:grid-cols-3 gap-4 text-xs font-mono mb-2">
          <div className="border border-gray-300 dark:border-gray-800 p-3 bg-gray-50 dark:bg-gray-900">
            <div className="font-bold text-gray-500 mb-1">Total Posts</div>
            <div className="text-lg font-bold">{stats.postCount}</div>
          </div>
          <div className="border border-gray-300 dark:border-gray-800 p-3 bg-gray-50 dark:bg-gray-900">
            <div className="font-bold text-gray-500 mb-1">Average Likes</div>
            <div className="text-lg font-bold">{stats.averageLikes}</div>
          </div>
          <div className="border border-gray-300 dark:border-gray-800 p-3 bg-gray-50 dark:bg-gray-900">
            <div className="font-bold text-gray-500 mb-1">Average Dislikes</div>
            <div className="text-lg font-bold">{stats.averageDislikes}</div>
          </div>
          <div className="border border-gray-300 dark:border-gray-800 p-3 bg-gray-50 dark:bg-gray-900">
            <div className="font-bold text-gray-500 mb-1">Total Likes</div>
            <div className="text-lg font-bold">{stats.totalLikes}</div>
          </div>
          <div className="border border-gray-300 dark:border-gray-800 p-3 bg-gray-50 dark:bg-gray-900">
            <div className="font-bold text-gray-500 mb-1">Total Dislikes</div>
            <div className="text-lg font-bold">{stats.totalDislikes}</div>
          </div>
          <div className="border border-gray-300 dark:border-gray-800 p-3 bg-gray-50 dark:bg-gray-900">
            <div className="font-bold text-gray-500 mb-1">Engagement Rate</div>
            <div className="text-lg font-bold">{stats.averageEngagementRate}%</div>
          </div>
        </div>
      </div>

      {/* Edit own profile settings */}
      {isOwnProfile && (
        <div className="border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950">
          <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
            Account Settings
          </h3>

          {updateSuccess && (
            <div className="bg-green-50 text-green-700 border border-green-300 p-3 mb-4 text-xs">
              Profile updated successfully.
            </div>
          )}
          {updateError && (
            <div className="bg-red-50 text-red-700 border border-red-300 p-3 mb-4 text-xs">
              Error: {updateError}
            </div>
          )}

          <form onSubmit={handleUpdateProfile} className="flex flex-col gap-4 max-w-sm">
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold">Username:</label>
              <input
                type="text"
                value={editUserName}
                onChange={(e) => setEditUserName(e.target.value)}
                disabled={updating}
                className="border border-gray-300 dark:border-gray-800 p-1.5 text-xs bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
                required
              />
            </div>
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold">Email Address:</label>
              <input
                type="email"
                value={editEmail}
                onChange={(e) => setEditEmail(e.target.value)}
                disabled={updating}
                className="border border-gray-300 dark:border-gray-800 p-1.5 text-xs bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
                required
              />
            </div>
            <div className="flex flex-col gap-1 text-xs">
              <label className="font-bold">New Password (leave blank to keep unchanged):</label>
              <input
                type="password"
                value={editPassword}
                onChange={(e) => setEditPassword(e.target.value)}
                disabled={updating}
                placeholder="********"
                className="border border-gray-300 dark:border-gray-800 p-1.5 text-xs bg-gray-50 dark:bg-gray-900 focus:outline-none focus:border-black dark:focus:border-white"
              />
            </div>
            <div>
              <button
                type="submit"
                disabled={updating}
                className="bg-gray-200 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 px-4 py-1.5 text-xs font-bold hover:bg-gray-300 dark:hover:bg-gray-700 cursor-pointer disabled:opacity-50"
              >
                {updating ? "Saving..." : "[Update Profile]"}
              </button>
            </div>
          </form>
        </div>
      )}

      {/* Admin User Control Panel */}
      {currentUser?.isAdmin && isOwnProfile && (
        <div className="border border-gray-300 dark:border-gray-800 p-6 bg-white dark:bg-gray-950">
          <h3 className="text-sm font-bold border-b border-gray-200 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide text-red-600">
            Administration Panel: Users List
          </h3>

          <div className="overflow-x-auto">
            <table className="w-full text-xs text-left border border-gray-350 dark:border-gray-800 border-collapse">
              <thead>
                <tr className="bg-gray-100 dark:bg-gray-900 border-b border-gray-300 dark:border-gray-800">
                  <th className="p-2 border-r border-gray-300 dark:border-gray-850">ID</th>
                  <th className="p-2 border-r border-gray-300 dark:border-gray-850">Username</th>
                  <th className="p-2 border-r border-gray-300 dark:border-gray-850">Email</th>
                  <th className="p-2 border-r border-gray-300 dark:border-gray-850">Role</th>
                  <th className="p-2">Action</th>
                </tr>
              </thead>
              <tbody>
                {users.map((u) => (
                  <tr key={u.id} className="border-b border-gray-200 dark:border-gray-900 hover:bg-gray-50 dark:hover:bg-gray-900">
                    <td className="p-2 border-r border-gray-300 dark:border-gray-850">{u.id}</td>
                    <td className="p-2 border-r border-gray-300 dark:border-gray-850">
                      <Link to={`/profile/${u.id}`} className="text-blue-600 hover:underline">
                        {u.userName}
                      </Link>
                    </td>
                    <td className="p-2 border-r border-gray-300 dark:border-gray-850">{u.email}</td>
                    <td className="p-2 border-r border-gray-300 dark:border-gray-850">{u.isAdmin ? "Admin" : "User"}</td>
                    <td className="p-2">
                      {u.id !== currentUser.id ? (
                        <button
                          onClick={() => handleUserDelete(u.id)}
                          className="text-red-600 font-bold hover:underline cursor-pointer"
                        >
                          [Delete User]
                        </button>
                      ) : (
                        <span className="text-gray-400 font-mono">(You)</span>
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
        <h3 className="text-sm font-bold border-b border-gray-300 dark:border-gray-800 pb-2 mb-4 uppercase tracking-wide">
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
