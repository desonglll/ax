import { useEffect, useState } from "react";
import { Card, Descriptions, Avatar, Spin, Button, App } from "antd";
import { UserOutlined, LogoutOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router-dom";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import RouteEndpoint from "@/config/endpoints/route_endpoint";
import getData from "@/utils/data_fetch";
import type { User } from "@/models/user";

function Profile() {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();
  const { message } = App.useApp();

  useEffect(() => {
    const fetchProfile = async () => {
      try {
        const res = await getData(AxiosEndpoint.Profile);
        const rawData = res?.body?.data ?? null;
        if (rawData) {
          const item = Array.isArray(rawData) ? rawData[0] : rawData;
          if (item) setUser(item as User);
        }
      } finally {
        setLoading(false);
      }
    };
    fetchProfile();
  }, []);

  const handleLogout = async () => {
    try {
      await getData(AxiosEndpoint.LogOut, "POST");
      message.success("Logged out");
      navigate(RouteEndpoint.SignIn);
    } catch {
      message.error("Logout failed");
    }
  };

  if (loading) {
    return <Spin style={{ display: "block", margin: "80px auto" }} />;
  }

  if (!user) {
    return <div>Unable to load profile</div>;
  }

  return (
    <Card>
      <div style={{ textAlign: "center", marginBottom: 24 }}>
        <Avatar size={80} icon={<UserOutlined />} />
      </div>
      <Descriptions column={1} bordered>
        <Descriptions.Item label="ID">{user.id}</Descriptions.Item>
        <Descriptions.Item label="Username">{user.userName}</Descriptions.Item>
        <Descriptions.Item label="Email">{user.email}</Descriptions.Item>
        <Descriptions.Item label="Created">{user.createdAt}</Descriptions.Item>
      </Descriptions>
      <div style={{ textAlign: "center", marginTop: 24 }}>
        <Button danger icon={<LogoutOutlined />} onClick={handleLogout}>
          Sign Out
        </Button>
      </div>
    </Card>
  );
}

export default Profile;
