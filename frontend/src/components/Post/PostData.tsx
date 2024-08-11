import Box from '@mui/material/Box';
import {Collapse, Fade, List, ListItem} from "@mui/material";
import PostListItem from "./PostListItem.tsx";
import {Post} from "../../models/post.ts";
import {useEffect, useState} from "react";
import getData from "../../utils/data_fetch.ts";
import {AxiosResponse} from "axios";
import {ApiResponse} from "../../models/api_response.ts";
import {useNavigate} from "react-router-dom";

export default function PostData() {

    const [isLoading, setIsLoading] = useState(true)
    const [posts, setPosts] = useState<Post[]>([])
    const navigate = useNavigate()


    useEffect(() => {

        const endpoint = "post/list-all"
        getData(endpoint).then((response: AxiosResponse<ApiResponse<Post>>) => {
            if (response.data.code === "Unauthorized") {
                console.log("Please Login!")
                navigate("/login")
            } else {
                setPosts(response.data.body.data)
                setIsLoading(false)
            }
        })

    }, []);

    return (
        <>
            {isLoading ? (
                <p></p>
            ) : (
                <Box>
                    <Fade in={!isLoading}>
                        <List>
                            {posts.map((post) => (
                                <ListItem key={post.id}>
                                    <PostListItem post={post}/>
                                </ListItem>
                            ))}
                        </List>
                    </Fade>
                </Box>
            )}
        </>
    )
}
