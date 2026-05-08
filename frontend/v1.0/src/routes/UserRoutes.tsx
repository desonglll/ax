import { Routes, Route } from "react-router-dom";
import UserPage from "@/pages/UserPage";

function UserRoutes() {
  return (
    <Routes>
      <Route index element={<UserPage />} />
    </Routes>
  );
}

export default UserRoutes;
