import { useState } from "react";
import { Card, Input, Button, App } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";

interface ReleasePostProps {
  onDone: () => void;
}

function ReleasePost({ onDone }: ReleasePostProps) {
  const [content, setContent] = useState("");
  const [loading, setLoading] = useState(false);
  const { message } = App.useApp();

  const handleSubmit = async () => {
    if (!content.trim()) {
      message.warning("Please enter content");
      return;
    }
    setLoading(true);
    try {
      await getData(AxiosEndpoint.CreatePost, "POST", { content });
      message.success("Post published");
      onDone();
    } catch {
      message.error("Failed to publish");
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card>
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: 16 }}>
        <Button icon={<ArrowLeftOutlined />} onClick={onDone}>
          Cancel
        </Button>
        <Button type="primary" loading={loading} onClick={handleSubmit}>
          Publish
        </Button>
      </div>
      <Input.TextArea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder="Write your post..."
        rows={8}
        style={{ fontSize: 16 }}
      />
    </Card>
  );
}

export default ReleasePost;
