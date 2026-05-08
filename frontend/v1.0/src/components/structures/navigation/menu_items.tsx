import {
  HomeOutlined,
  UserOutlined,
  LoginOutlined,
} from "@ant-design/icons";
import RouteEndpoint from "@/config/endpoints/route_endpoint";

const menuItems = [
  { key: RouteEndpoint.Index, icon: <HomeOutlined />, label: "Home" },
  { key: RouteEndpoint.User, icon: <UserOutlined />, label: "Profile" },
  { key: RouteEndpoint.SignIn, icon: <LoginOutlined />, label: "Sign In" },
];

export default menuItems;
