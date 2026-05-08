import { Layout } from "antd";
import { useNavigate } from "react-router-dom";
import AllRoutes from "@/routes/AllRoutes";
import RouteEndpoint from "@/config/endpoints/route_endpoint";
import TopMenu from "@/components/structures/navigation/menu/TopMenu";

const { Header, Content, Footer } = Layout;

function App() {
  const navigate = useNavigate();

  return (
    <Layout style={{ minHeight: "100vh" }}>
      <Header
        style={{
          display: "flex",
          alignItems: "center",
          background: "#fff",
          borderBottom: "1px solid #f0f0f0",
          padding: "0 24px",
        }}
      >
        <div
          style={{
            fontWeight: 700,
            fontSize: 20,
            marginRight: 40,
            cursor: "pointer",
            color: "#1677ff",
          }}
          onClick={() => navigate(RouteEndpoint.Index)}
        >
          AX
        </div>
        <TopMenu />
      </Header>
      <Content style={{ padding: "24px", maxWidth: 960, margin: "0 auto", width: "100%" }}>
        <AllRoutes />
      </Content>
      <Footer style={{ textAlign: "center", color: "#999" }}>
        AX Frontend ©{new Date().getFullYear()}
      </Footer>
    </Layout>
  );
}

export default App;
