import {useEffect} from "react";
import Profile from "../components/structures/user/Profile";
import loginCheck from "../utils/login_check";
import {useNavigate} from "react-router-dom";
import RouteEndpoint from "../config/endpoints/route_endpoint.ts";

function UserPage() {
  const navigate = useNavigate();
  useEffect(() => {
    loginCheck().then((result: boolean) => {
      if (!result) {
        navigate(RouteEndpoint.SignIn);
      }
    });
  }, [navigate]);
  return (
    <div>
      <Profile />
    </div>
  );
}

export default UserPage;
