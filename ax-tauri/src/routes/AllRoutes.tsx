import {BrowserRouter, Route, Routes} from "react-router-dom";
import LoginPage from "../pages/LoginPage.tsx";
import IndexPage from "../pages/IndexPage.tsx";
import CommonRoutes from "./CommonRoutes.tsx";

function AllRoutes() {
  return (
    <>
      <BrowserRouter>
        <Routes>
          <Route path={"/"} element={<IndexPage />} />
          <Route path={"/login"} element={<LoginPage />} />
          <Route path={"/common/*"} element={<CommonRoutes />} />
        </Routes>
      </BrowserRouter>
    </>
  );
}

export default AllRoutes;
