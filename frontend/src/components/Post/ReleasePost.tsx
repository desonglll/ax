import Box from "@mui/material/Box";
import axios from "axios";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Vditor from "vditor";
import "vditor/dist/index.css";

export function ReleasePost() {
  const navigate = useNavigate();
  useEffect(() => {
    const vditor = new Vditor("vditor", {
      typewriterMode: true,
      after: () => {
        console.log(vditor.getValue());
      },
      ctrlEnter: (value) => {
        console.log("hello", value);
        const data = {
          content: value,
        };
        axios.post("post/insert", data).then((resp) => {
          if (resp.data.code === "Success") {
            console.log("Success");
            navigate(-1);
          }
        });
      },
    });
  }, [navigate]);

  return (
    <>
      <Box
        sx={{
          width: "100%",
          display: "flex",
          alignItems: "center",
          flexDirection: "column",
        }}
      >
        <Box sx={{ width: "80%", marginTop: "60px" }}>
          <div id="vditor" className="vditor" />
        </Box>
      </Box>
    </>
  );
}
