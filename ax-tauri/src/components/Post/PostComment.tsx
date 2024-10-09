import {Textarea} from "@mui/joy";
import Box from "@mui/material/Box";
import Button from "@mui/joy/Button";
import type React from "react";
import {useEffect, useState} from "react";
import axios from "axios";
import type {Comment} from "../../models/post";
import getData from "../../utils/data_fetch";
import {Fade, List, ListItem} from "@mui/material";
import PostCommentItem from "./PostCommentItem";

export function PostComment({ reply_to }: { reply_to: number }) {
  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  useEffect(() => {
    getData(`comment/reply-to-post/${reply_to}`)
      .then((resp) => {
        if (resp.data.code === "Success") {
          setComments(resp.data.body.data);
        }
      })
      .finally(() => {
        setLoading(false);
      });
  }, [reply_to]);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formData = new FormData(e.currentTarget);
    const data = {
      replyTo: Number(reply_to),
      content: formData.get("content"),
      replyToType: "post",
    };
    console.log(data);
    axios.post("comment/insert", data).then((resp) => {
      console.log(resp.data);
    });
  };
  return (
    <>
      <Box sx={{ width: "80%" }}>
        <form onSubmit={handleSubmit}>
          <Textarea
            minRows={4}
            size="md"
            variant="outlined"
            placeholder={"Leave your comment!"}
            name={"content"}
            required
          />
          <Box
            sx={{
              display: "flex",
              marginTop: "10px",
              justifyContent: "flex-end",
            }}
          >
            <Button type={"submit"}>Submit</Button>
          </Box>
        </form>
        <Fade in={!loading}>
          <List>
            {comments.map((comment) => (
              <ListItem key={comment.id}>
                <PostCommentItem comment={comment} />
              </ListItem>
            ))}
          </List>
        </Fade>
      </Box>
    </>
  );
}
