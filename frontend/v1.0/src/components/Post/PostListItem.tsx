import type { Post } from "../../models/post.ts";
import Box from "@mui/material/Box";
import { Button, CardActions, CardContent, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Sheet } from "@mui/joy";
import Vditor from "vditor";
import Endpoint from "../../routes/common/end_point.ts";
import ReactionItem from "../ReactionItem.tsx";

const bull = (
  <Box
    component="span"
    sx={{ display: "inline-block", mx: "2px", transform: "scale(0.8)" }}
  >
    •
  </Box>
);

export default function PostListItem({ post }: { post: Post }) {
  const navigate = useNavigate();
  const [postItem, _setPostItem] = useState<Post>(post);

  // biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
  useEffect(() => {
    Vditor.preview(
      document.getElementById(`pre-${post.id}`) as HTMLDivElement,
      post.content,
      {
        cdn: "https://unpkg.com/vditor@3.10.5",
        mode: "light",
      }
    );
  }, [post.id]);
  const handleDetail = (id: number) => {
    navigate(`${Endpoint.PostDetail}/${id}`);
  };

  return (
    <>
      <Sheet
        sx={{
          width: "70%",
          mx: "auto", // margin left & right
          my: 4, // margin top & bottom
          py: 3, // padding top & bottom
          px: 2, // padding left & right
          display: "flex",
          flexDirection: "column",
          gap: 2,
          borderRadius: "sm",
          boxShadow: "md",
        }}
        variant="outlined"
      >
        <CardContent
          sx={{
            maxHeight: "300px", // 设置最大高度
            overflow: "hidden", // 隐藏溢出的内容
            textOverflow: "ellipsis", // 添加省略号
          }}
        >
          <Typography sx={{ fontSize: 14 }} color="text.secondary" gutterBottom>
            {postItem.userName} {bull} {postItem.createdAt}
          </Typography>
          <Button
            onClick={() => handleDetail(postItem.id)}
            sx={{ color: "black" }}
          >
            <p id={`pre-${post.id}`} />
          </Button>
        </CardContent>
        <CardActions sx={{ justifyContent: "space-between" }}>
          <ReactionItem toId={post.id} />
        </CardActions>
      </Sheet>
    </>
  );
}
