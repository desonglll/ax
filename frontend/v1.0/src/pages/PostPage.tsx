import { useEffect } from "react";
import loginCheck from "../utils/login_check";
import { useNavigate } from "react-router-dom";
import Endpoint from "../routes/common/end_point";
import PostData from "../components/Post/PostData";

function PostPage() {
  const navigate = useNavigate();
  useEffect(() => {
    loginCheck().then((result) => {
      if (!result) {
        navigate(Endpoint.SignIn);
      }
    });
  }, [navigate]);
  return (
    <>
      <PostData />
    </>
  );
}

export default PostPage;
