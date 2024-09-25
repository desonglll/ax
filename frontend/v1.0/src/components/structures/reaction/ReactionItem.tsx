import {useEffect, useState} from "react";
import {Box, Button} from "@mui/material";
import ThumbUpOffAltIcon from "@mui/icons-material/ThumbUpOffAlt";
import ThumbUpAltIcon from "@mui/icons-material/ThumbUpAlt";
import CommentIcon from "@mui/icons-material/Comment";
import ThumbDownOffAltIcon from "@mui/icons-material/ThumbDownOffAlt";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDown";
import axios from "axios";
import {AxiosEndpoint} from "../../../config/endpoints/axios_endpoint.ts";
import {Reaction, ReactionTable} from "../../../models/reaction.ts";
import loginCheck from "../../../utils/login_check.ts";
import {useNavigate} from "react-router-dom";
import RouteEndpoint from "../../../config/endpoints/route_endpoint.ts";

function ReactionItem({toId, toType, likeCount, dislikeCount}: {
    toId: number;
    toType: string,
    likeCount: number | null,
    dislikeCount: number | null
}) {
    const [like, setLike] = useState<boolean>(false);
    const [dislike, setDislike] = useState<boolean>(false);
    const [reactionItem, setReactionItem] = useState<Reaction>();
    let [reactionTable, setReactionTable] = useState<ReactionTable>({
        like: 0,
        dislike: 0,
    });
    const navigate = useNavigate();
    useEffect(() => {
        // Check if authentication
        loginCheck()
            .then((result) => {
                if (!result) {
                    navigate(RouteEndpoint.SignIn);
                }
            })
            .then(() => {
                // 用来检测登录用户是否为toId点赞
                axios
                    .get(`${AxiosEndpoint.GetReaction}?toId=${toId}&toType=${toType}`)
                    .then((resp) => {
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
            })
            .then(() => {
                if (likeCount !== null && dislikeCount !== null) {
                    setReactionTable({like: likeCount, dislike: dislikeCount})
                } else {
                    console.log("查询like dislike总数")
                    // 用来查询like dislike总数
                    axios
                        .get(
                            `${AxiosEndpoint.GetReactionTable}?toId=${toId}&toType=${toType}`
                        )
                        .then((resp) => {
                            if (resp.data.code == 200) {
                                setReactionTable(resp.data.body.data);
                            }
                        });
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
                    setReactionTable({
                        like: reactionTable.like - 1,
                        dislike: reactionTable.dislike,
                    });
                });
        } else {
            axios
                .post(`${AxiosEndpoint.LikeReaction}?toId=${toId}&toType=${toType}`)
                .then(() => {
                    setDislike(false);
                    setLike(!like);
                    setReactionTable({
                        like: reactionTable.like + 1,
                        dislike: dislike
                            ? reactionTable.dislike - 1
                            : reactionTable.dislike,
                    });
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
                    setReactionTable({
                        like: reactionTable.like,
                        dislike: reactionTable.dislike - 1,
                    });
                });
        } else {
            axios
                .post(`${AxiosEndpoint.DislikeReaction}?toId=${toId}&toType=${toType}`)
                .then((resp) => {
                    console.log(resp.data);

                    setLike(false);
                    setDislike(!dislike);

                    setReactionTable({
                        like: like ? reactionTable.like - 1 : reactionTable.like,
                        dislike: reactionTable.dislike + 1,
                    });
                });
        }
    };
    return (
        <>
            <Box sx={{display: "flex"}}>
                <div>
                    <Button size="small" onClick={() => handleLike()}>
                        {like ? <ThumbUpAltIcon/> : <ThumbUpOffAltIcon/>}
                        {reactionTable?.like}
                    </Button>
                </div>
                <div>
                    <Button size="small" onClick={() => handleDislike()}>
                        {dislike ? <ThumbDownAltIcon/> : <ThumbDownOffAltIcon/>}
                        {reactionTable?.dislike}
                    </Button>
                </div>
                <Button size="small">
                    <CommentIcon/>
                </Button>
            </Box>
        </>
    );
}

export default ReactionItem;
