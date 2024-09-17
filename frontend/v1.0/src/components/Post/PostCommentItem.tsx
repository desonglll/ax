import {
  Button,
  Card,
  CardActions,
  CardContent,
  Typography,
} from "@mui/material";
import type { Comment } from "../../models/post";
import { useEffect } from "react";
import ReactionItem from "../ReactionItem";
function PostCommentItem({ comment }: { comment: Comment }) {
  useEffect(() => {}, [comment.id]);

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
          <ReactionItem toId={comment.id} toType="comment" />
        </CardActions>
      </Card>
    </>
  );
}

export default PostCommentItem;
