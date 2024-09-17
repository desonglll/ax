import { Route, Routes } from "react-router-dom";
import SignInPage from "../pages/SignInPage.tsx";
import IndexPage from "../pages/IndexPage.tsx";
import { BrowserRouter } from "react-router-dom";
import PostRoutes from "./PostRoutes.tsx";
import { UserRoutes } from "./UserRoutes.tsx";
import RouteEndpoint from "../config/endpoints/route_endpoint.ts";

function AllRoutes() {
  return (
    <>
      <BrowserRouter>
        <Routes>
          <Route path={"/"} element={<IndexPage />} />
          <Route path={"/index"} element={<IndexPage />} />
          <Route path={RouteEndpoint.SignIn} element={<SignInPage />} />
          <Route path={RouteEndpoint.Post} element={<PostRoutes />} />
          <Route path={RouteEndpoint.User} element={<UserRoutes />} />
        </Routes>
      </BrowserRouter>
    </>
  );
}

export default AllRoutes;
