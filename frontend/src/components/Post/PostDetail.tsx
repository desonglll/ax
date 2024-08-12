import {useParams} from "react-router-dom";
import {Button, Fade, Typography} from "@mui/material";
import {useEffect, useState} from "react";
import getData from "../../utils/data_fetch.ts";
import {Post} from "../../models/post.ts";
import Box from "@mui/material/Box";
import ThumbUpAltIcon from "@mui/icons-material/ThumbUpAlt";
import ThumbUpOffAltIcon from "@mui/icons-material/ThumbUpOffAlt";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDown";
import ThumbDownOffAltIcon from "@mui/icons-material/ThumbDownOffAlt";
import CommentIcon from "@mui/icons-material/Comment";
import axios from "axios";

export function PostDetail() {
    const {id}: { id: number } = useParams(); // 获取路径参数 id
    const [post, setPost] = useState<Post>()

    const [like, setLike] = useState<boolean>(false)
    const [dislike, setDislike] = useState<boolean>(false)
    const [loading, setLoading] = useState(true)
    const handleReaction = (status: boolean, setStatus: (boolean) => void, reaction_name: string) => {
        const data = {
            post_id: Number(id),
            reaction_name: reaction_name
        }
        if (status) {
            axios.post("reaction/delete", data).then((r) => {
                setStatus(false)
            }).catch(error => console.error("Error updating reaction:", error));


        } else {
            axios.post("reaction/insert", data).then(() => {
                setStatus(true)
            }).catch(error => console.error("Error updating reaction:", error));

        }
    }
    const handleLike = () => {
        if (like) {
            handleReaction(like, setLike, "like")
        } else {
            if (dislike) {
                handleReaction(dislike, setDislike, "dislike")
            }
            handleReaction(like, setLike, "like")
        }
    }
    const handleDislike = () => {
        if (dislike) {
            handleReaction(dislike, setDislike, "dislike")
        } else {
            if (like) {
                handleReaction(like, setLike, "like")
            }
            handleReaction(dislike, setDislike, "dislike")
        }
    }
    useEffect(() => {
        getData(`reaction/post/${id}`).then((resp) => {
            const resp_reactions = resp.data.body.data;
            resp_reactions.map((reaction_item) => {
                if (reaction_item.reaction_name === "like") {
                    setLike(true)
                } else if (reaction_item.reaction_name === "dislike") {
                    setDislike(true)
                }
            })
        })
        getData(`post/detail/${id}`).then((resp) => {
            if (resp.data.code === "Success") {
                setPost(resp.data.body.data)
            }
        }).finally(() => {
            setLoading(false)
        })
    }, []);

    return (
        <>
            <Fade in={!loading}>
                <Box sx={{marginTop: '50px', margin: '0px', padding: '10px'}}>

                    <Typography
                        sx={{
                            whiteSpace: 'pre-line',
                            textAlign: "left",
                            fontFamily: "Noto Sans SC, Noto Sans TC",
                            marginTop: '20px',
                            marginBottom: '80px'
                        }}
                    >
                        {post?.content}
                    </Typography>
                    <Box sx={{
                        display: 'flex',
                        marginBottom: '50px',
                        justifyContent: 'flex-end'
                    }}>
                        <Button size="small" onClick={() => handleLike()}>{like ? (
                            <ThumbUpAltIcon/>) : (
                            <ThumbUpOffAltIcon/>)}</Button>
                        <Button size="small"
                                onClick={() => handleDislike()}>{dislike ? (
                            <ThumbDownAltIcon/>) : (<ThumbDownOffAltIcon/>)}</Button>
                        <Button size="small"><CommentIcon/></Button>
                    </Box>
                </Box>
            </Fade>
        </>
    );
}