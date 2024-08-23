import {Route, Routes} from "react-router-dom";
import SignInPage from "../pages/SignInPage.tsx";
import IndexPage from "../pages/IndexPage.tsx";
import {BrowserRouter} from "react-router-dom";
import End_points from "./common/end_points.ts";
import PostRoutes from "./PostRoutes.tsx";
import {UserRoutes} from "./UserRoutes.tsx";

function AllRoutes() {
    return (
        <>
            <BrowserRouter>
                <Routes>
                    <Route path={"/"} element={<IndexPage/>}/>
                    <Route path={"/index"} element={<IndexPage/>}/>
                    <Route path={End_points.SignIn} element={<SignInPage/>}/>
                    <Route path={End_points.Post} element={<PostRoutes/>}/>
                    <Route path={End_points.User} element={<UserRoutes/>}/>
                </Routes>
            </BrowserRouter>
        </>
    );
}

export default AllRoutes;
