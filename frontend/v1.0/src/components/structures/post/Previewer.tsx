import { Typography } from "antd";

interface PreviewerProps {
  content: string;
}

function Previewer({ content }: PreviewerProps) {
  return (
    <div style={{ whiteSpace: "pre-wrap", padding: 16 }}>
      {content || <Typography.Text type="secondary">Nothing to preview</Typography.Text>}
    </div>
  );
}

export default Previewer;
