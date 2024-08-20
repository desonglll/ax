import { Button } from "@mui/joy";
import { useNavigate } from "react-router-dom";

function BackButton() {
  const navigate = useNavigate();
  const handleBack = () => {
    navigate(-1);
  };
  return (
    <>
      <Button onClick={() => handleBack()}>Back</Button>
    </>
  );
}

export default BackButton;
