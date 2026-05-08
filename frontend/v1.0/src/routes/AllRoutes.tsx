import { Routes, Route } from "react-router-dom";
import CommonRoutes from "./CommonRoutes";
import PostRoutes from "./PostRoutes";
import UserRoutes from "./UserRoutes";
import IndexPage from "@/pages/IndexPage";
import SignInPage from "@/pages/SignInPage";

function AllRoutes() {
  return (
    <Routes>
      <Route element={<CommonRoutes />}>
        <Route path="/" element={<IndexPage />} />
        <Route path="/signin" element={<SignInPage />} />
        <Route path="/posts/*" element={<PostRoutes />} />
        <Route path="/user/*" element={<UserRoutes />} />
      </Route>
    </Routes>
  );
}

export default AllRoutes;
