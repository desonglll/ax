import {useParams} from "react-router-dom";
import {Fade} from "@mui/material";

export function PostDetail() {
    const {id}: { id: number } = useParams(); // 获取路径参数 id

    return (
        <>
            <Fade in={true}>
                <p>
                    Post Detail {id}
                </p>
            </Fade>
        </>
    );
}