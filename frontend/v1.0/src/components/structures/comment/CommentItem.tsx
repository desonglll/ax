import { Typography } from "antd";
import { UserOutlined } from "@ant-design/icons";
import type { Comment } from "@/models/comment";

interface CommentItemProps {
  comment: Comment;
}

function CommentItem({ comment }: CommentItemProps) {
  return (
    <div style={{ padding: "8px 0", borderBottom: "1px solid #f0f0f0" }}>
      <Typography.Text strong>
        <UserOutlined style={{ marginRight: 4 }} />
        {comment.userName}
      </Typography.Text>
      <Typography.Text type="secondary" style={{ marginLeft: 8 }}>
        {comment.createdAt}
      </Typography.Text>
      <Typography.Paragraph style={{ margin: "4px 0 0" }}>{comment.content}</Typography.Paragraph>
    </div>
  );
}

export default CommentItem;
