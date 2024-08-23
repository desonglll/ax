import { Box } from "@mui/joy";
import SignIn from "../components/SignIn.tsx";

function SignInPage() {
  return (
    <>
      <Box
        sx={{
          height: "100vh",
          display: "flex",
          justifyContent: "center", // 水平方向居中
          alignItems: "center",
        }}
      >
        <SignIn />
      </Box>
    </>
  );
}

export default SignInPage;
