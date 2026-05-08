import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Profile from "@/components/structures/user/Profile";
import { useAuth } from "@/contexts/AuthContext";
import RouteEndpoint from "@/config/endpoints/route_endpoint";

function UserPage() {
  const navigate = useNavigate();
  const { loggedIn, loading } = useAuth();

  useEffect(() => {
    if (!loading && !loggedIn) {
      navigate(RouteEndpoint.SignIn);
    }
  }, [loggedIn, loading, navigate]);

  if (loading) return null;

  return loggedIn ? <Profile /> : null;
}

export default UserPage;
