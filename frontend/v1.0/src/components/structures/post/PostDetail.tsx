import { useEffect, useState } from "react";
import { Typography, Card, Spin, Button, Divider, Modal, App } from "antd";
import { ArrowLeftOutlined, UserOutlined, CalendarOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";
import { useParams, useNavigate } from "react-router-dom";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";
import { useAuth } from "@/contexts/AuthContext";
import type { Post } from "@/models/post";
import ReactionItem from "@/components/structures/reaction/ReactionItem";
import CommentList from "@/components/structures/comment/CommentList";
import Editor from "./Editor";

function PostDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const { message } = App.useApp();
  const { loggedIn, currentUser } = useAuth();
  const [post, setPost] = useState<Post | null>(null);
  const [loading, setLoading] = useState(true);
  const [editing, setEditing] = useState(false);
  const [editContent, setEditContent] = useState("");
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    const fetchPost = async () => {
      setLoading(true);
      try {
        const res = await getData(`${AxiosEndpoint.PostDetail}/${id}`);
        const rawData = res?.body?.data ?? null;
        const item = Array.isArray(rawData) ? rawData[0] : rawData;
        if (item) setPost(item as Post);
      } catch {
        // 401 for unauthenticated
      } finally {
        setLoading(false);
      }
    };
    fetchPost();
  }, [id]);

  if (loading) {
    return <Spin style={{ display: "block", margin: "80px auto" }} />;
  }

  if (!post) {
    return <Typography.Text type="danger">Post not found</Typography.Text>;
  }

  const isOwner = loggedIn && currentUser && currentUser.id === post.userId;

  const handleEdit = () => {
    setEditContent(post.content);
    setEditing(true);
  };

  const handleSave = async () => {
    if (!editContent.trim()) return;
    setSaving(true);
    try {
      await getData(`${AxiosEndpoint.UpdatePost}/${post.id}`, "PUT", { content: editContent });
      setPost({ ...post, content: editContent });
      setEditing(false);
      message.success("Post updated");
    } catch {
      message.error("Failed to update post");
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = () => {
    Modal.confirm({
      title: "Delete Post",
      content: "Are you sure you want to delete this post?",
      okText: "Delete",
      okType: "danger",
      onOk: async () => {
        try {
          await getData(`${AxiosEndpoint.DeletePost}/${post.id}`, "DELETE");
          message.success("Post deleted");
          navigate("/");
        } catch {
          message.error("Failed to delete post");
        }
      },
    });
  };

  return (
    <div>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 16 }}>
        <Button icon={<ArrowLeftOutlined />} onClick={() => navigate(-1)}>
          Back
        </Button>
        {isOwner && !editing && (
          <div style={{ display: "flex", gap: 8 }}>
            <Button icon={<EditOutlined />} onClick={handleEdit}>Edit</Button>
            <Button danger icon={<DeleteOutlined />} onClick={handleDelete}>Delete</Button>
          </div>
        )}
      </div>

      <Card>
        <div style={{ display: "flex", gap: 16, marginBottom: 8 }}>
          <Typography.Text type="secondary">
            <UserOutlined style={{ marginRight: 4 }} />
            {post.userName}
          </Typography.Text>
          <Typography.Text type="secondary">
            <CalendarOutlined style={{ marginRight: 4 }} />
            {post.createdAt}
          </Typography.Text>
        </div>

        <Divider style={{ margin: "12px 0" }} />

        {editing ? (
          <div>
            <Editor value={editContent} onChange={setEditContent} />
            <div style={{ display: "flex", justifyContent: "flex-end", gap: 8, marginTop: 12 }}>
              <Button onClick={() => setEditing(false)}>Cancel</Button>
              <Button type="primary" loading={saving} onClick={handleSave}>Save</Button>
            </div>
          </div>
        ) : (
          <div style={{ whiteSpace: "pre-wrap" }}>{post.content}</div>
        )}
      </Card>

      {!editing && (
        <>
          <Divider />
          <ReactionItem postId={post.id} likeCount={post.likeCount} dislikeCount={post.dislikeCount} />
          <Divider />
          <CommentList postId={post.id} />
        </>
      )}
    </div>
  );
}

export default PostDetail;
