import { useEffect } from "react";
import Vditor from "vditor";
import "vditor/dist/index.css";

function Editor() {
  useEffect(() => {
    const vditor = new Vditor("vditor", {
      typewriterMode: true,
      after: () => {
        console.log(vditor.getValue());
      },
      ctrlEnter: (value) => {
        console.log("hello", value);
      },
      input(value) {
        console.log(value);
        console.log("input");
      },
      blur(value) {
        console.log(value);
        console.log("blur");
      },
    });
  }, []);

  return (
    <>
      <div id="vditor" className="vditor" />
    </>
  );
}

export default Editor;
