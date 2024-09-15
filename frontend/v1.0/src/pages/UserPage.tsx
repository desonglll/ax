import { useEffect } from "react";
import Profile from "../components/User/Profile";
import loginCheck from "../utils/login_check";
import { useNavigate } from "react-router-dom";
import Endpoint from "../routes/common/end_point";

function UserPage() {
  const navigate = useNavigate();
  useEffect(() => {
    loginCheck().then((result: boolean) => {
      if (!result) {
        navigate(Endpoint.SignIn);
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
