import Login from "../components/Login.tsx";
import Box from "@mui/material/Box";
import Typography from '@mui/joy/Typography';

function LoginPage() {

    return (
        <>
            <Box sx={{display: 'flex', flexDirection: 'column', alignItems: 'center'}}>
                <h2 style={{marginTop: '50px', marginBottom: '50px'}}>🚪Login Page</h2>
                <Box sx={{display: 'flex', flexDirection: 'column', alignItems: 'center', marginBottom: '50px'}}>
                    <Typography level="h4">
                        痛苦的人总是携带傲慢与偏见
                    </Typography>
                    <Typography level="h4">
                        幸福的人总是多一份宽容与慈悲
                    </Typography>
                </Box>
                <Login/>
            </Box>
        </>
    )
}

export default LoginPage