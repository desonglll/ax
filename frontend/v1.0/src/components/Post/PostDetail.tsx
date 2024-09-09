import { useNavigate, useParams } from "react-router-dom";
import { Button, Fade } from "@mui/material";
import { useEffect, useState } from "react";
import getData from "../../utils/data_fetch.ts";
import type { Post } from "../../models/post.ts";
import Box from "@mui/material/Box";
import ThumbUpAltIcon from "@mui/icons-material/ThumbUpAlt";
import ThumbUpOffAltIcon from "@mui/icons-material/ThumbUpOffAlt";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDown";
import ThumbDownOffAltIcon from "@mui/icons-material/ThumbDownOffAlt";
import axios from "axios";
import { PostComment } from "./PostComment.tsx";
import Vditor from "vditor";
import BackButton from "../Navigation/BackButton.tsx";
import type { ApiResponse } from "../../models/api_response.ts";
import type { File } from "../../models/file.ts";
import Endpoint from "../../routes/common/end_point.ts";

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
  const [editing, setEditing] = useState(false);
  const navigate = useNavigate();

  const handleReaction = async (
    status: boolean,
    setStatus: (arg0: boolean) => void,
    reaction_name: string
  ) => {
    const data = {
      postId: Number(id),
      reactionName: reaction_name,
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
        navigate(Endpoint.SignIn);
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
          const vditor = new Vditor("vditor", {
            typewriterMode: true,
            after: () => {
              vditor.setValue(resp.data.body.data.content);
            },
            ctrlEnter: (value) => {
              console.log("hello", value);
              const updated_request = {
                id: Number(id),
                content: value,
              };
              axios.post("post/update", updated_request).then((resp) => {
                if (resp.data.code === "Success") {
                  console.log(resp.data);
                }
              });
            },
            blur(value: string) {
              console.log("hello", value);
              const updated_request = {
                id: Number(id),
                content: value,
              };
              axios.post("post/update", updated_request).then((resp) => {
                if (resp.data.code === "Success") {
                  console.log(resp.data);
                }
              });
            },
            upload: {
              accept: "image/*,.mp3, .wav, .rar",
              url: `${axios.defaults.baseURL}/upload`,
              filename(name) {
                return name
                  .replace(/[^(a-zA-Z0-9\u4e00-\u9fa5\.)]/g, "")
                  .replace(/[\?\\/:|<>\*\[\]\(\)\$%\{\}@~]/g, "")
                  .replace("/\\s/g", "");
              },
              withCredentials: true,
              format(files, responseText) {
                const response: ApiResponse<File> = JSON.parse(responseText);
                console.log(response);
                console.log(files);

                const result = {
                  msg: "",
                  code: 0,
                  data: {
                    errFiles: [] as string[],
                    succMap: {} as { [key: string]: string }, // 这里定义键为字符串，值也为字符串的对象
                  },
                };

                if (response.code === "Success") {
                  const data = response.body.data;
                  data.map((file) => {
                    result.data.succMap[
                      file.name
                    ] = `${axios.defaults.baseURL}/stream/${file.id}`;
                  });
                } else {
                  // 如果上传不成功，可以在 msg 中返回服务器的错误信息
                  result.msg = response.message || "Upload failed!";
                }

                return JSON.stringify(result);
              },
            },
          });
          Vditor.preview(
            document.getElementById("pre") as HTMLDivElement,
            resp.data.body.data.content,
            {
              cdn: "https://unpkg.com/vditor@3.10.5",
              mode: "light",
            }
          );
        }
      })
      .then(() => {})
      .finally(() => {
        setLoading(false);
      });
  }, [id, navigate]);
  // Vditor.preview(
  //   document.getElementById("pre") as HTMLDivElement,
  //   post.content,
  //   {
  //     cdn: "https://unpkg.com/vditor@3.10.5",
  //     mode: "light",
  //   }
  // );

  return (
    <>
      {true && (
        <Fade in={!loading}>
          <Box
            sx={{
              marginTop: "50px",
              margin: "0px",
              marginBottom: "200px",
              padding: "10px",
            }}
          >
            <BackButton />
            <div hidden={localStorage.getItem("user_name") !== post.userName}>
              <Button
                onClick={() => {
                  setEditing(!editing);
                }}
              >
                Edit
              </Button>
            </div>
            <div id="vditor" hidden={!editing} />

            <div hidden={editing}>
              <div id="pre" />
            </div>

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
              <PostComment reply_to={Number(id)} />
            </Box>
          </Box>
        </Fade>
      )}
    </>
  );
}
