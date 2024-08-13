import { useNavigate, useParams } from "react-router-dom";
import { Button, Fade, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import getData from "../../utils/data_fetch.ts";
import type { Post } from "../../models/post.ts";
import Box from "@mui/material/Box";
import ThumbUpAltIcon from "@mui/icons-material/ThumbUpAlt";
import ThumbUpOffAltIcon from "@mui/icons-material/ThumbUpOffAlt";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDown";
import ThumbDownOffAltIcon from "@mui/icons-material/ThumbDownOffAlt";
import axios from "axios";
import { Comment } from "./Comment.tsx";

export function PostDetail() {
  const { id } = useParams(); // 获取路径参数 id
  const [post, setPost] = useState<Post>({
    content: "",
    createdAt: "",
    id: 0,
    reactions: undefined,
    replyTo: undefined,
    updatedAt: "",
    userId: 0,
    userName: "",
  });

  const [like, setLike] = useState<boolean>(false);
  const [dislike, setDislike] = useState<boolean>(false);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  const handleReaction = async (
    status: boolean,
    setStatus: (arg0: boolean) => void,
    reaction_name: string
  ) => {
    const data = {
      post_id: Number(id),
      reaction_name: reaction_name,
    };
    try {
      if (status) {
        await axios.post("reaction/delete", data);
        setStatus(false);
      } else {
        await axios.post("reaction/insert", data);
        setStatus(true);
      }

      // Fetch updated post details
      const response = await getData(`post/detail/${id}`);
      if (response.data.code === "Success") {
        setPost(response.data.body.data);
      }
    } catch (error) {
      console.error("Error updating reaction:", error);
    }
  };

  const handleLike = () => {
    if (like) {
      handleReaction(like, setLike, "like");
    } else {
      if (dislike) {
        handleReaction(dislike, setDislike, "dislike");
      }
      handleReaction(like, setLike, "like");
    }
  };
  const handleDislike = () => {
    if (dislike) {
      handleReaction(dislike, setDislike, "dislike");
    } else {
      if (like) {
        handleReaction(like, setLike, "like");
      }
      handleReaction(dislike, setDislike, "dislike");
    }
  };

  useEffect(() => {
    getData(`reaction/post/${id}`).then((resp) => {
      if (resp.data.code !== "Success") {
        navigate("/login");
      } else {
        const resp_reactions = resp.data.body.data;
        resp_reactions.map((reaction_item: { reaction_name: string }) => {
          if (reaction_item.reaction_name === "like") {
            setLike(true);
          } else if (reaction_item.reaction_name === "dislike") {
            setDislike(true);
          }
        });
      }
    });
    getData(`post/detail/${id}`)
      .then((resp) => {
        if (resp.data.code === "Success") {
          setPost(resp.data.body.data);
        }
      })
      .finally(() => {
        setLoading(false);
      });
  }, [id, navigate]);

  return (
    <>
      <Fade in={!loading}>
        <Box
          sx={{
            marginTop: "50px",
            margin: "0px",
            marginBottom: "200px",
            padding: "10px",
          }}
        >
          <Typography
            sx={{
              whiteSpace: "pre-line",
              textAlign: "left",
              fontFamily: "Noto Sans SC, Noto Sans TC",
              marginTop: "20px",
              marginBottom: "80px",
            }}
          >
            {post?.content}
          </Typography>
          <Box
            sx={{
              display: "flex",
              marginBottom: "50px",
              justifyContent: "flex-end",
            }}
          >
            <div>
              <Button size="small" onClick={() => handleLike()}>
                {like ? <ThumbUpAltIcon /> : <ThumbUpOffAltIcon />}
                {post.reactions?.like}
              </Button>
            </div>
            <div>
              <Button size="small" onClick={() => handleDislike()}>
                {dislike ? <ThumbDownAltIcon /> : <ThumbDownOffAltIcon />}
                {post.reactions?.dislike}
              </Button>
            </div>
          </Box>
          <Box sx={{ display: "flex", justifyContent: "center" }}>
            <Comment reply_to={Number(id)} />
          </Box>
        </Box>
      </Fade>
    </>
  );
}
