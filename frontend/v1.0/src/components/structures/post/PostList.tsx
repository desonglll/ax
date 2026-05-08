import { useEffect, useState } from "react";
import { Button, Empty, Spin } from "antd";
import { PlusOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";
import { useAuth } from "@/contexts/AuthContext";
import type { Post } from "@/models/post";
import PostListItem from "./PostListItem";
import ReleasePost from "./ReleasePost";

function PostList() {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(true);
  const [showEditor, setShowEditor] = useState(false);
  const navigate = useNavigate();
  const { loggedIn } = useAuth();

  const fetchPosts = async () => {
    setLoading(true);
    try {
      const res = await getData(AxiosEndpoint.PostList, "GET", undefined, { limit: 20, offset: 0 });
      const list: Post[] = Array.isArray(res?.body?.data) ? res.body.data : [];
      setPosts(list);
    } catch {
      // API may return 401 for unauthenticated users — show empty state
      setPosts([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPosts();
  }, []);

  if (loading) {
    return <Spin style={{ display: "block", margin: "80px auto" }} />;
  }

  if (showEditor) {
    return <ReleasePost onDone={() => { setShowEditor(false); fetchPosts(); }} />;
  }

  return (
    <div>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 16 }}>
        <h2 style={{ margin: 0 }}>Posts</h2>
        {loggedIn && (
          <Button type="primary" icon={<PlusOutlined />} onClick={() => setShowEditor(true)}>
            New Post
          </Button>
        )}
      </div>
      {posts.length === 0 ? (
        <Empty description="No posts yet" />
      ) : (
        <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
          {posts.map((post) => (
            <PostListItem key={post.id} post={post} onClick={() => navigate(`/posts/${post.id}`)} />
          ))}
        </div>
      )}
    </div>
  );
}

export default PostList;
