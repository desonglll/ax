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
import {useNavigate} from "react-router-dom";

function ReactionItem({toId, toType, likeCount, dislikeCount, isUserLike, isUserDislike}: {
    toId: number;
    toType: string,
    likeCount: number | null,
    dislikeCount: number | null,
    isUserLike: boolean,
    isUserDislike: boolean
}) {
    const [like, setLike] = useState<boolean>(isUserLike);
    const [dislike, setDislike] = useState<boolean>(isUserDislike);
    const [reactionItem, setReactionItem] = useState<Reaction>();
    let [reactionTable, setReactionTable] = useState<ReactionTable>({
        like: 0,
        dislike: 0,
    });
    const [loading, setLoading] = useState(true)
    const navigate = useNavigate();
    useEffect(() => {
        // Check if authentication

        if (likeCount !== null && dislikeCount !== null) {
            setReactionTable({like: likeCount, dislike: dislikeCount})
        } else {
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
        setLoading(!loading)
    }, []);
    const handleLike = () => {
        if (like) {
            axios.get(`${AxiosEndpoint.GetReaction}?toId=${toId}&toType=${toType}&reactionName=Like`).then((resp) => {
                console.log(resp.data)
                axios
                    .delete(
                        `${AxiosEndpoint.DeleteReaction}?reactionId=${resp.data.body.data[0].id}`
                    )
                    .then(() => {
                        setLike(!like);
                        setReactionTable({
                            like: reactionTable.like - 1,
                            dislike: reactionTable.dislike,
                        });
                    });
            })

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
            {!loading && (
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
            )}
        </>
    );
}

export default ReactionItem;
