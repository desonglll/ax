import { Card, Typography } from "antd";
import { CalendarOutlined, UserOutlined } from "@ant-design/icons";
import type { Post } from "@/models/post";
import ReactionItem from "@/components/structures/reaction/ReactionItem";

interface PostListItemProps {
  post: Post;
  onClick: () => void;
}

function PostListItem({ post, onClick }: PostListItemProps) {
  return (
    <Card hoverable onClick={onClick} style={{ cursor: "pointer" }}>
      <Typography.Paragraph ellipsis={{ rows: 1 }} style={{ margin: 0, fontWeight: 600, fontSize: 16 }}>
        {post.content}
      </Typography.Paragraph>
      <div style={{ marginTop: 8, display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <div style={{ display: "flex", gap: 16 }}>
          <Typography.Text type="secondary">
            <UserOutlined style={{ marginRight: 4 }} />
            {post.userName}
          </Typography.Text>
          <Typography.Text type="secondary">
            <CalendarOutlined style={{ marginRight: 4 }} />
            {post.createdAt}
          </Typography.Text>
        </div>
        <div onClick={(e) => e.stopPropagation()}>
          <ReactionItem postId={post.id} likeCount={post.likeCount} dislikeCount={post.dislikeCount} />
        </div>
      </div>
    </Card>
  );
}

export default PostListItem;
