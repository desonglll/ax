import {Textarea} from "@mui/joy";
import Box from "@mui/material/Box";
import React from "react";
import Button from "@mui/joy/Button";
import axios from "axios";

export function ReleasePost() {

    const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const formData = new FormData(e.currentTarget)
        const content = formData.get("content")
        const data = {
            content: content
        }
        axios.post("post/insert", data).then((resp) => {
            if (resp.data.code === "Success") {
                console.log("Success")
            }
        })
    }

    return (
        <>
            <Box sx={{
                width: '100%',
                display: 'flex',
                alignItems: 'center',
                flexDirection: 'column'
            }}>
                <Box sx={{width: '80%', marginTop: '60px'}}>
                    <form onSubmit={onSubmit}>
                        <Textarea
                            minRows={6}
                            size="md"
                            variant="outlined"
                            placeholder={"Say something!"}
                            name={"content"}
                            required
                        />
                        <Box sx={{display: 'flex', marginTop: '10px', justifyContent: 'flex-end'}}>
                            <Button type={"submit"}>Submit</Button>
                        </Box>
                    </form>
                </Box>
            </Box>
        </>
    );
}