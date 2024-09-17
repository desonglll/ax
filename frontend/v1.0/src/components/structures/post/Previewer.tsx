import {useEffect} from "react";
import Vditor from "vditor";
import {Post} from "../../../models/post.ts";

export function Previewer({post}: { post: Post }) {
    useEffect(() => {
        Vditor.preview(
            document.getElementById("pre") as HTMLDivElement,
            post.content,
            {
                cdn: "https://unpkg.com/vditor@3.10.5",
                mode:
                    "light",
            }
        ).then(_ => {
        });

    }, []);

    return (
        <>
            <div>
                <div id="pre"/>
            </div>
        </>
    );
}