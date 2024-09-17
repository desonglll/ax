import React, { useEffect, useState } from "react";
import {
  Box,
  Button,
  CardActions,
  CardContent,
  Typography,
} from "@mui/material";
import ThumbUpOffAltIcon from "@mui/icons-material/ThumbUpOffAlt";
import ThumbUpAltIcon from "@mui/icons-material/ThumbUpAlt";
import CommentIcon from "@mui/icons-material/Comment";
import ThumbDownOffAltIcon from "@mui/icons-material/ThumbDownOffAlt";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDown";
import axios from "axios";
import { AxiosEndpoint } from "../libs/axios_endpoint";
import { Reaction } from "../models/reaction";

function ReactionItem({ toId }: { toId: number }) {
  const [like, setLike] = useState<boolean>(false);
  const [dislike, setDislike] = useState<boolean>(false);
  const [reactionItem, setReactionItem] = useState<Reaction>();
  useEffect(() => {
    axios.get(`${AxiosEndpoint.GetReaction}?toId=${toId}`).then((resp) => {
      console.log("reactions in reactionItem:", resp.data);
      const reactions_vec: [Reaction] = resp.data.body.data;
      setReactionItem(reactions_vec[0]);
      if (reactions_vec.length > 0) {
        if (reactions_vec[0].reactionName == "Like") {
          setLike(true);
          setDislike(false);
        }
        if (reactions_vec[0].reactionName == "Dislike") {
          setLike(false);
          setDislike(true);
        }
      }
    });
  }, []);
  const handleLike = () => {
    if (like) {
      axios
        .delete(
          `${AxiosEndpoint.DeleteReaction}?reactionId=${reactionItem?.id}`
        )
        .then(() => {
          setLike(!like);
        });
    } else {
      axios.post(`${AxiosEndpoint.LikeReaction}?toId=${toId}`).then(() => {
        setDislike(false);
        setLike(!like);
      });
    }
  };
  const handleDislike = () => {
    if (dislike) {
      axios
        .delete(
          `${AxiosEndpoint.DeleteReaction}?reactionId=${reactionItem?.id}`
        )
        .then(() => {
          setDislike(!dislike);
        });
    } else {
      axios.post(`${AxiosEndpoint.DislikeReaction}?toId=${toId}`).then(() => {
        setLike(false);
        setDislike(!dislike);
      });
    }
  };
  return (
    <>
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
    </>
  );
}

export default ReactionItem;
