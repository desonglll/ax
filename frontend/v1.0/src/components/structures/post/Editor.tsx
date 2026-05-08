import { Input } from "antd";

interface EditorProps {
  value: string;
  onChange: (value: string) => void;
}

function Editor({ value, onChange }: EditorProps) {
  return (
    <Input.TextArea
      value={value}
      onChange={(e) => onChange(e.target.value)}
      placeholder="Write your content..."
      rows={10}
      style={{ fontSize: 15 }}
    />
  );
}

export default Editor;
