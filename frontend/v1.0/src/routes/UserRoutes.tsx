import {Route, Routes} from "react-router-dom";
import {AxSkeleton} from "../components/common/skeleton/AxSkeleton.tsx";
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
