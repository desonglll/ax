import {Post} from "../../models/post.ts";
import Box from "@mui/material/Box";
import {Button, Card, CardActions, CardContent, Typography} from "@mui/material";

const bull = (
    <Box
        component="span"
        sx={{display: 'inline-block', mx: '2px', transform: 'scale(0.8)'}}
    >
        •
    </Box>
);


export default function PostListItem({post}: { post: Post }) {

    return (
        <>
            <Card>
                <CardContent>
                    <Typography sx={{fontSize: 14}} color="text.secondary" gutterBottom>
                        {post.userName} {bull} {post.createdAt}
                    </Typography>
                    <Typography variant="body1">
                        {/*<Box component="span" sx={dropCapStyle}>*/}
                        {/*    {post.content.charAt(0)}*/}
                        {/*</Box>*/}
                        {/*{post.content.slice(1)}*/}
                        {post.content}
                    </Typography>
                </CardContent>
                <CardActions>
                    <Button size="small">Detail</Button>
                </CardActions>
            </Card>
        </>
    )

}