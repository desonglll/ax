import { Menu, App } from "antd";
import {
  HomeOutlined,
  UserOutlined,
  LoginOutlined,
  LogoutOutlined,
} from "@ant-design/icons";
import { useNavigate, useLocation } from "react-router-dom";
import RouteEndpoint from "@/config/endpoints/route_endpoint";
import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import { useAuth } from "@/contexts/AuthContext";
import getData from "@/utils/data_fetch";

function TopMenu() {
  const navigate = useNavigate();
  const location = useLocation();
  const { message } = App.useApp();
  const { loggedIn, refresh } = useAuth();

  const handleLogout = async () => {
    try {
      await getData(AxiosEndpoint.LogOut, "POST");
      message.success("Logged out");
      refresh();
      navigate(RouteEndpoint.SignIn);
    } catch {
      message.error("Logout failed");
    }
  };

  const authItem = loggedIn
    ? { key: "logout", icon: <LogoutOutlined />, label: "Sign Out", onClick: handleLogout }
    : { key: RouteEndpoint.SignIn, icon: <LoginOutlined />, label: "Sign In" };

  const items = [
    { key: RouteEndpoint.Index, icon: <HomeOutlined />, label: "Home" },
    ...(loggedIn ? [{ key: RouteEndpoint.User, icon: <UserOutlined />, label: "Profile" }] : []),
    authItem,
  ];

  return (
    <Menu
      mode="horizontal"
      selectedKeys={[location.pathname]}
      items={items}
      onClick={({ key }) => {
        if (key !== "logout") navigate(key);
      }}
      style={{ flex: 1, border: "none" }}
    />
  );
}

export default TopMenu;
