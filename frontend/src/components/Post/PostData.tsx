import Box from "@mui/material/Box";
import { Fab, Fade, List, ListItem } from "@mui/material";
import PostListItem from "./PostListItem.tsx";
import type { Post } from "../../models/post.ts";
import { useEffect, useState } from "react";
import getData from "../../utils/data_fetch.ts";
import axios, { type AxiosResponse } from "axios";
import type { ApiResponse } from "../../models/api_response.ts";
import { useNavigate } from "react-router-dom";
import AddIcon from "@mui/icons-material/Add";
import { Pagination } from "antd";

export default function PostData() {
  const [isLoading, setIsLoading] = useState(true);
  const [posts, setPosts] = useState<Post[]>([]);
  const navigate = useNavigate();
  const [response, setResponse] = useState<ApiResponse<Post>>();

  useEffect(() => {
    const endpoint = "post/list-all";
    getData(endpoint).then((response: AxiosResponse<ApiResponse<Post>>) => {
      if (response.data.code === "Unauthorized") {
        console.log("Please Login!");
        navigate("/login");
      } else {
        setResponse(response.data);
        setPosts(response.data.body.data);
        setIsLoading(false);
      }
    });
  }, [navigate]);

  const handleChangePagination = (page: number, pageSize: number) => {
    console.log(page, pageSize);
    const pagination = {
      offset: pageSize * (page - 1),
      limit: pageSize,
    };

    try {
      axios
        .get(
          `post/list-all?limit=${pagination.limit}&offset=${pagination.offset}`
        )
        .then((resp) => {
          if (resp.data.code === "Success") {
            setPosts(resp.data.body.data);
          }
        });
    } catch (e) {
      console.log(e);
    }
  };

  return (
    <>
      {isLoading ? (
        <p>Loading</p>
      ) : (
        <Box>
          <Fade in={!isLoading}>
            <Box
              sx={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
              }}
            >
              <List sx={{ width: "100%" }}>
                {posts.map((post) => (
                  <ListItem key={post.id} sx={{ justifyContent: "center" }}>
                    <PostListItem post={post} />
                  </ListItem>
                ))}
              </List>
              <Pagination
                total={response?.body.pagination.count}
                showSizeChanger
                showQuickJumper
                showTotal={(total) => `Total ${total} items`}
                style={{ marginTop: "40px", marginBottom: "40px" }}
                onChange={handleChangePagination}
              />
            </Box>
          </Fade>
          <Fab
            color="primary"
            aria-label="add"
            sx={{ position: "fixed", bottom: 70, right: 16 }}
            onClick={() => {
              navigate("/post/new");
            }}
          >
            <AddIcon />
          </Fab>
        </Box>
      )}
    </>
  );
}
