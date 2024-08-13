import {Textarea} from "@mui/joy";
import Box from "@mui/material/Box";
import Button from '@mui/joy/Button';
import React from "react";
import axios from "axios";

export function Comment({reply_to}: { reply_to: number }) {
    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const formData = new FormData(e.currentTarget)
        const data = {
            reply_to: Number(reply_to),
            content: formData.get("content")
        }
        console.log(data)
        axios.post("comment/insert", data).then((resp) => {
            console.log(resp.data)
        })

    }
    return (
        <>
            <Box sx={{width: "80%"}}>
                <form onSubmit={handleSubmit}>
                    <Textarea
                        minRows={4}
                        size="md"
                        variant="outlined"
                        placeholder={"Leave your comment!"}
                        name={"content"}
                        required
                    />
                    <Box sx={{display: 'flex', marginTop: '10px', justifyContent: 'flex-end'}}>
                        <Button type={"submit"}>Submit</Button>
                    </Box>
                </form>
            </Box>
        </>
    );
}