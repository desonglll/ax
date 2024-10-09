import {useEffect} from "react";
import loginCheck from "../utils/login_check";
import {useNavigate} from "react-router-dom";
import RouteEndpoint from "../config/endpoints/route_endpoint.ts";
import PostList from "../components/structures/post/PostList.tsx";

function PostPage() {
  const navigate = useNavigate();
  useEffect(() => {
    loginCheck().then((result) => {
      if (!result) {
        navigate(RouteEndpoint.SignIn);
      }
    });
  }, [navigate]);
  return (
    <>
      <PostList />
    </>
  );
}

export default PostPage;
