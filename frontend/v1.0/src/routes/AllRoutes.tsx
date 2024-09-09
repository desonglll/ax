import { Route, Routes } from "react-router-dom";
import SignInPage from "../pages/SignInPage.tsx";
import IndexPage from "../pages/IndexPage.tsx";
import { BrowserRouter } from "react-router-dom";
import PostRoutes from "./PostRoutes.tsx";
import { UserRoutes } from "./UserRoutes.tsx";
import Endpoint from "./common/end_point.ts";

function AllRoutes() {
  return (
    <>
      <BrowserRouter>
        <Routes>
          <Route path={"/"} element={<IndexPage />} />
          <Route path={"/index"} element={<IndexPage />} />
          <Route path={Endpoint.SignIn} element={<SignInPage />} />
          <Route path={Endpoint.Post} element={<PostRoutes />} />
          <Route path={Endpoint.User} element={<UserRoutes />} />
        </Routes>
      </BrowserRouter>
    </>
  );
}

export default AllRoutes;
