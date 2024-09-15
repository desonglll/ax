import { Route, Routes } from "react-router-dom";
import Profile from "../components/User/Profile.tsx";
import { AxSkeleton } from "../components/AxSkeleton.tsx";

export function UserRoutes() {
  return (
    <>
      <AxSkeleton>
        <Routes>
          <Route path={"profile"} element={<Profile />} />
        </Routes>
      </AxSkeleton>
    </>
  );
}
