import { Outlet } from "react-router-dom";
import { Breadcrumb } from "antd";
import { useLocation, Link } from "react-router-dom";
import { HomeOutlined } from "@ant-design/icons";

const breadcrumbNameMap: Record<string, string> = {
  "/posts": "Posts",
  "/user": "Profile",
  "/signin": "Sign In",
};

function CommonRoutes() {
  const location = useLocation();
  const pathSnippets = location.pathname.split("/").filter((i) => i);

  const breadcrumbItems = [
    {
      title: (
        <Link to="/">
          <HomeOutlined /> Home
        </Link>
      ),
    },
    ...pathSnippets.map((_, index) => {
      const url = `/${pathSnippets.slice(0, index + 1).join("/")}`;
      return {
        title: <Link to={url}>{breadcrumbNameMap[url] || url}</Link>,
      };
    }),
  ];

  return (
    <>
      <Breadcrumb items={breadcrumbItems} style={{ marginBottom: 16 }} />
      <Outlet />
    </>
  );
}

export default CommonRoutes;
