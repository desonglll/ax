import { useEffect, useState } from "react";
import { Button, Space, Tooltip } from "antd";
import { LikeOutlined, LikeFilled, DislikeOutlined, DislikeFilled } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";
import { useAuth } from "@/contexts/AuthContext";
import type { Reaction } from "@/models/reaction";

interface ReactionItemProps {
  postId: number;
  likeCount: number | null;
  dislikeCount: number | null;
}

function ReactionItem({ postId, likeCount, dislikeCount }: ReactionItemProps) {
  const navigate = useNavigate();
  const { loggedIn } = useAuth();
  const [likes, setLikes] = useState(likeCount ?? 0);
  const [dislikes, setDislikes] = useState(dislikeCount ?? 0);
  const [userReaction, setUserReaction] = useState<Reaction | null>(null);

  const fetchUserReaction = async () => {
    if (!loggedIn) return;
    try {
      const res = await getData(AxiosEndpoint.GetReaction, "GET");
      const list: Reaction[] = Array.isArray(res?.body?.data) ? res.body.data : [];
      const found = list.find((r) => r.toId === postId && r.toType === "post");
      setUserReaction(found ?? null);
    } catch {
      setUserReaction(null);
    }
  };

  useEffect(() => {
    fetchUserReaction();
  }, [postId, loggedIn]);

  useEffect(() => {
    setLikes(likeCount ?? 0);
    setDislikes(dislikeCount ?? 0);
  }, [likeCount, dislikeCount]);

  const requireLogin = () => {
    navigate("/signin");
  };

  const handleLike = async () => {
    if (!loggedIn) { requireLogin(); return; }
    try {
      if (userReaction && userReaction.reactionName === "like") {
        await getData(AxiosEndpoint.DeleteReaction, "DELETE", undefined, { reactionId: userReaction.id });
        setUserReaction(null);
        setLikes((c) => Math.max(0, c - 1));
      } else {
        if (userReaction && userReaction.reactionName === "dislike") {
          setDislikes((c) => Math.max(0, c - 1));
        }
        await getData(AxiosEndpoint.LikeReaction, "POST", undefined, { toId: postId, toType: "post" });
        setLikes((c) => c + 1);
        setUserReaction({ id: -1, userId: -1, toId: postId, createdAt: "", reactionName: "like", toType: "post" });
        fetchUserReaction();
      }
    } catch {
      // ignore
    }
  };

  const handleDislike = async () => {
    if (!loggedIn) { requireLogin(); return; }
    try {
      if (userReaction && userReaction.reactionName === "dislike") {
        await getData(AxiosEndpoint.DeleteReaction, "DELETE", undefined, { reactionId: userReaction.id });
        setUserReaction(null);
        setDislikes((c) => Math.max(0, c - 1));
      } else {
        if (userReaction && userReaction.reactionName === "like") {
          setLikes((c) => Math.max(0, c - 1));
        }
        await getData(AxiosEndpoint.DislikeReaction, "POST", undefined, { toId: postId, toType: "post" });
        setDislikes((c) => c + 1);
        setUserReaction({ id: -1, userId: -1, toId: postId, createdAt: "", reactionName: "dislike", toType: "post" });
        fetchUserReaction();
      }
    } catch {
      // ignore
    }
  };

  const likeBtn = (
    <Button
      size="small"
      type={userReaction?.reactionName === "like" ? "primary" : "default"}
      icon={userReaction?.reactionName === "like" ? <LikeFilled /> : <LikeOutlined />}
      onClick={handleLike}
    >
      {likes}
    </Button>
  );

  const dislikeBtn = (
    <Button
      size="small"
      danger={userReaction?.reactionName === "dislike"}
      icon={userReaction?.reactionName === "dislike" ? <DislikeFilled /> : <DislikeOutlined />}
      onClick={handleDislike}
    >
      {dislikes}
    </Button>
  );

  return (
    <Space>
      {loggedIn ? likeBtn : <Tooltip title="Sign in to like">{likeBtn}</Tooltip>}
      {loggedIn ? dislikeBtn : <Tooltip title="Sign in to dislike">{dislikeBtn}</Tooltip>}
    </Space>
  );
}

export default ReactionItem;
