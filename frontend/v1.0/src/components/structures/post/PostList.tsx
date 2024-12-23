import Box from "@mui/material/Box";
import {CircularProgress, Fab, Fade, List, ListItem,} from "@mui/material";
import PostListItem from "./PostListItem.tsx";
import type {Post} from "../../../models/post.ts";
import {useEffect, useState} from "react";
import getData from "../../../utils/data_fetch.ts";
import axios, {type AxiosResponse} from "axios";
import type {ApiResponse} from "../../../models/api_response.ts";
import {useNavigate} from "react-router-dom";
import AddIcon from "@mui/icons-material/Add";
import {Pagination} from "antd";
import RouteEndpoint from "../../../config/endpoints/route_endpoint.ts";
import {AxiosEndpoint} from "../../../config/endpoints/axios_endpoint.ts";
import {Reaction} from "../../../models/reaction.ts";

export default function PostList() {
    const [isLoading, setIsLoading] = useState(true);
    const [posts, setPosts] = useState<Post[]>([]);
    const [reactions, setReactions] = useState<Reaction[]>([]);
    const [currentPage, setCurrentPage] = useState<number>(() => {
        // 从 localStorage 获取当前页码
        const savedPage = localStorage.getItem("currentPage");
        return savedPage ? Number.parseInt(savedPage, 10) : 1;
    });
    const [pageSize, setPageSize] = useState<number>(10); // 默认每页显示10个
    const navigate = useNavigate();
    const [response, setResponse] = useState<Partial<ApiResponse<Post>>>();

    useEffect(() => {
        const endpoint = `${AxiosEndpoint.PostList}?limit=${pageSize}&offset=${
            pageSize * (currentPage - 1)
        }`;
        getData(endpoint).then((response: AxiosResponse<ApiResponse<Post>>) => {
            if (response.data.code == 401) {
                console.log("Please SignIn!");
                navigate(RouteEndpoint.SignIn);
            } else {
                setResponse(response.data);
                if (response.data.body) {
                    setPosts(response.data.body.data);
                }
                setIsLoading(false);
            }
        });
        getData(`${AxiosEndpoint.GetReaction}`).then((resp) => {
            setReactions(resp.data.body.data)
            console.log(resp.data)
        })
    }, [navigate, currentPage, pageSize]);

    const handleChangePagination = (page: number, newPageSize: number) => {
        setCurrentPage(page);
        setPageSize(newPageSize);
        // 将当前页码保存到 localStorage
        localStorage.setItem("currentPage", page.toString());

        const pagination = {
            offset: newPageSize * (page - 1),
            limit: newPageSize,
        };

        try {
            axios
                .get(
                    `${AxiosEndpoint.PostList}?limit=${pagination.limit}&offset=${pagination.offset}`
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
                <div>
                    <Box
                        sx={{
                            display: "flex",
                            alignItems: "center",
                            justifyContent: "center",
                            marginTop: "10px",
                        }}
                    >
                        <CircularProgress/>
                    </Box>
                </div>
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
                            <List sx={{width: "100%"}}>
                                {/* eslint-disable-next-line @typescript-eslint/no-unused-vars */}
                                {posts.map((post, _index) => (
                                    // <Grow
                                    //     key={post.id}
                                    //     in={!isLoading}
                                    //     style={{transformOrigin: "0 0 0"}}
                                    //     timeout={(index + 1) * 500} // 每个组件的延迟时间增加1000ms
                                    // >
                                    <ListItem key={post.id} sx={{justifyContent: "center"}}>
                                        <PostListItem post={post} reactions={reactions}/>
                                    </ListItem>
                                    // </Grow>
                                ))}
                            </List>

                            <Pagination
                                current={currentPage}
                                total={response?.body?.pagination?.count}
                                pageSize={pageSize}
                                showSizeChanger
                                showQuickJumper
                                showTotal={(total) => `Total ${total} items`}
                                style={{marginTop: "40px", marginBottom: "40px"}}
                                onChange={handleChangePagination}
                            />
                        </Box>
                    </Fade>
                    <Fab
                        color="primary"
                        aria-label="add"
                        sx={{position: "fixed", bottom: 70, right: 16}}
                        onClick={() => {
                            navigate(`${RouteEndpoint.PostNew}`);
                        }}
                    >
                        <AddIcon/>
                    </Fab>
                </Box>
            )}
        </>
    );
}
