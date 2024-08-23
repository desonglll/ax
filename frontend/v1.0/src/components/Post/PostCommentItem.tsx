import {
  Box,
  Button,
  Card,
  CardActions,
  CardContent,
  Typography,
} from "@mui/material";
import type { Comment } from "../../models/post";
import ThumbUpAltIcon from "@mui/icons-material/ThumbUpAlt";
import ThumbDownOffAltIcon from "@mui/icons-material/ThumbDownOffAlt";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDown";
import CommentIcon from "@mui/icons-material/Comment";
import ThumbUpOffAltIcon from "@mui/icons-material/ThumbUpOffAlt";
import { useEffect, useState } from "react";
import getData from "../../utils/data_fetch";
import axios from "axios";
function PostCommentItem({ comment }: { comment: Comment }) {
  const [like, setLike] = useState<boolean>(false);
  const [dislike, setDislike] = useState<boolean>(false);

  useEffect(() => {
    getData(`comment-reaction/comment/${comment.id}`).then((resp) => {
      const resp_reactions = resp.data.body.data;
      resp_reactions.map((reaction_item: { reaction_name: string }) => {
        if (reaction_item.reaction_name === "like") {
          setLike(true);
        } else if (reaction_item.reaction_name === "dislike") {
          setDislike(true);
        }
      });
    });
  }, [comment.id]);

  const handleReaction = async (
    status: boolean,
    setStatus: (arg0: boolean) => void,
    reaction_name: string
  ) => {
    const data = {
      commentId: Number(comment.id),
      reactionName: reaction_name,
    };
    console.log(data);

    try {
      if (status) {
        await axios.post("comment-reaction/delete", data);
        setStatus(false);
      } else {
        await axios.post("comment-reaction/insert", data);
        setStatus(true);
      }
    } catch (error) {
      console.error("Error handling reaction", error);
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

  return (
    <>
      <Card sx={{ width: "100%" }}>
        <CardContent>
          <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
            {comment.user_name}
          </Typography>
          <Button sx={{ color: "black" }}>
            <Typography
              variant="body1"
              sx={{
                whiteSpace: "pre-line", // Preserve whitespace and line breaks
                textAlign: "left",
                overflow: "hidden", // Hide overflowed content
                textOverflow: "ellipsis", // Display ellipsis for overflowed content
                display: "-webkit-box",
                WebkitBoxOrient: "vertical",
                WebkitLineClamp: 3, // Number of lines to show before truncating
                lineClamp: 3, // Number of lines to show before truncating
              }}
            >
              {comment.content}
            </Typography>
          </Button>
        </CardContent>
        <CardActions sx={{ justifyContent: "space-between" }}>
          {/*<Button size="small" onClick={() => handleDetail(postItem.id)}>Detail</Button>*/}
          <Box sx={{ display: "flex" }}>
            <div>
              <Button size="small" onClick={() => handleLike()}>
                {like ? <ThumbUpAltIcon /> : <ThumbUpOffAltIcon />}
              </Button>
            </div>
            <div>
              <Button size="small" onClick={() => handleDislike()}>
                {dislike ? <ThumbDownAltIcon /> : <ThumbDownOffAltIcon />}
              </Button>
            </div>
            <Button size="small">
              <CommentIcon />
            </Button>
          </Box>
        </CardActions>
      </Card>
    </>
  );
}

export default PostCommentItem;
