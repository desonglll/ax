import {Route, Routes} from "react-router-dom";
import LoginPage from "../pages/LoginPage.tsx";
import TestPage from "../pages/TestPage.tsx";
import PostRoutes from "./PostRoutes.tsx";
import IndexPage from "../pages/IndexPage.tsx";
import {BrowserRouter} from "react-router-dom";
import {UserRoutes} from "./UserRoutes.tsx";

function AllRoutes() {


    return (
        <>
            <BrowserRouter>
                <Routes>
                    <Route path={"/"} element={<IndexPage/>}/>
                    <Route path={"/login"} element={<LoginPage/>}/>
                    <Route path={"/index"} element={<IndexPage/>}/>
                    <Route path={"/test"} element={<TestPage/>}/>
                    <Route path={"/post/*"} element={<PostRoutes/>}/>
                    <Route path={"/user/*"} element={<UserRoutes/>}/>
                </Routes>
            </BrowserRouter>
        </>
    )
}

export default AllRoutes;