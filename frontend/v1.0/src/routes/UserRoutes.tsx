import { Route, Routes } from "react-router-dom";
import { AxSkeleton } from "../components/AxSkeleton.tsx";
import UserPage from "../pages/UserPage.tsx";

export function UserRoutes() {
  return (
    <>
      <AxSkeleton>
        <Routes>
          <Route path={"profile"} element={<UserPage />} />
        </Routes>
      </AxSkeleton>
    </>
  );
}
