import { useState } from "react";
import { Card, Form, Input, Button, Typography, App } from "antd";
import { UserOutlined, LockOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
import RouteEndpoint from "@/config/endpoints/route_endpoint";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";
import { useAuth } from "@/contexts/AuthContext";

function SignIn() {
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();
  const { message } = App.useApp();
  const { loggedIn, refresh } = useAuth();

  const onFinish = async (values: { userName: string; password: string }) => {
    setLoading(true);
    try {
      const result = await getData(AxiosEndpoint.Login, "POST", values);
      if (result?.code === 200) {
        message.success("Signed in successfully");
        refresh();
        navigate(RouteEndpoint.Index);
      } else {
        message.error(result?.message || "Invalid credentials");
      }
    } catch {
      message.error("Sign in failed");
    } finally {
      setLoading(false);
    }
  };

  if (loggedIn) {
    navigate(RouteEndpoint.Index);
    return null;
  }

  return (
    <Card style={{ maxWidth: 400, margin: "60px auto" }}>
      <Typography.Title level={3} style={{ textAlign: "center", marginBottom: 24 }}>
        Sign In
      </Typography.Title>
      <Form name="signin" onFinish={onFinish} autoComplete="off">
        <Form.Item name="userName" rules={[{ required: true, message: "Please enter username" }]}>
          <Input prefix={<UserOutlined />} placeholder="Username" size="large" />
        </Form.Item>
        <Form.Item name="password" rules={[{ required: true, message: "Please enter password" }]}>
          <Input.Password prefix={<LockOutlined />} placeholder="Password" size="large" />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" loading={loading} block size="large">
            Sign In
          </Button>
        </Form.Item>
      </Form>
    </Card>
  );
}

export default SignIn;
