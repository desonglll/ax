import { useEffect, useState } from "react";
import { Input, Button, List, Spin, Empty, App, Typography } from "antd";
import { SendOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";
import { useAuth } from "@/contexts/AuthContext";
import type { Comment } from "@/models/comment";
import CommentItem from "./CommentItem";

interface CommentListProps {
  postId: number;
}

function CommentList({ postId }: CommentListProps) {
  const navigate = useNavigate();
  const { loggedIn } = useAuth();
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState(true);
  const [content, setContent] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const { message } = App.useApp();

  const fetchComments = async () => {
    setLoading(true);
    try {
      const res = await getData(AxiosEndpoint.GetComment, "GET", undefined, { replyTo: postId, replyToType: "post" });
      const rawData = res?.body?.data ?? [];
      const list = Array.isArray(rawData) ? rawData : rawData ? [rawData] : [];
      setComments(list);
    } catch {
      setComments([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchComments();
  }, [postId]);

  const handleSubmit = async () => {
    if (!content.trim()) return;
    setSubmitting(true);
    try {
      await getData(AxiosEndpoint.CreateComment, "POST", {
        content,
        replyTo: postId,
        replyToType: "post",
      });
      setContent("");
      message.success("Comment posted");
      fetchComments();
    } catch {
      message.error("Failed to post comment");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div>
      <h3>Comments</h3>
      {loggedIn ? (
        <div style={{ display: "flex", gap: 8, marginBottom: 16 }}>
          <Input
            value={content}
            onChange={(e) => setContent(e.target.value)}
            placeholder="Write a comment..."
            onPressEnter={handleSubmit}
          />
          <Button type="primary" icon={<SendOutlined />} loading={submitting} onClick={handleSubmit}>
            Send
          </Button>
        </div>
      ) : (
        <div style={{ marginBottom: 16 }}>
          <Typography.Text type="secondary">
            <Typography.Link onClick={() => navigate("/signin")}>Sign in</Typography.Link> to post a comment
          </Typography.Text>
        </div>
      )}
      {loading ? (
        <Spin />
      ) : comments.length === 0 ? (
        <Empty description="No comments yet" />
      ) : (
        <List
          dataSource={comments}
          renderItem={(comment) => <CommentItem key={comment.id} comment={comment} />}
        />
      )}
    </div>
  );
}

export default CommentList;
