import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import AccountCircle from "@mui/icons-material/AccountCircle";
import MenuItem from "@mui/material/MenuItem";
import Menu from "@mui/material/Menu";
import axios from "axios";
import { useEffect, useState } from "react";
import loginCheck from "../../utils/login_check.ts";
import { useNavigate } from "react-router-dom";
import { Avatar, Fade } from "@mui/material";
import type { User } from "../../models/user.ts";
import getData from "../../utils/data_fetch.ts";
import Endpoint from "../../routes/common/end_point.ts";
import { AxiosEndpoint } from "../../libs/axios_endpoint.ts";

export default function AppMenu({
  drawerOpen,
  setDrawerOpen,
}: {
  drawerOpen: boolean;
  setDrawerOpen: (arg0: boolean) => void;
}) {
  const [auth, setAuth] = useState(true);
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const [loading, setLoading] = useState(true);
  const [profile, setProfile] = useState<Partial<User>>();
  const navigate = useNavigate();

  const handleMenu = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };
  const handleLogout = () => {
    axios
      .post(`${AxiosEndpoint.LogOut}`)
      .then((r) => {
        if (r.data.code === "Success") {
          setAuth(false);
          setAnchorEl(null);
        }
      })
      .finally(() => {
        navigate(Endpoint.SignIn);
      });
  };
  const handleLogin = () => {
    navigate(Endpoint.SignIn);
  };
  useEffect(() => {
    loginCheck()
      .then((result) => {
        if (!result) {
          setAuth(false);
          navigate(Endpoint.SignIn);
        }
      })
      .then(() => {
        getData(AxiosEndpoint.Profile).then((resp) => {
          if (resp.data.code === 200) {
            setProfile(resp.data.body.data);
          }
        });
      })
      .finally(() => {
        setLoading(false);
      });
  }, [navigate]);

  return (
    <>
      {loading ? (
        <></>
      ) : (
        <Fade in={!loading}>
          <AppBar position="fixed">
            <Toolbar>
              <IconButton
                size="large"
                edge="start"
                color="inherit"
                aria-label="menu"
                sx={{ mr: 2 }}
                onClick={() => {
                  setDrawerOpen(!drawerOpen);
                }}
              >
                <MenuIcon />
              </IconButton>
              <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                Posts
              </Typography>

              <div>
                <IconButton
                  size="large"
                  aria-label="account of current user"
                  aria-controls="menu-appbar"
                  aria-haspopup="true"
                  onClick={handleMenu}
                  color="inherit"
                >
                  {profile?.profilePicture ? (
                    <Avatar
                      src={`${axios.defaults.baseURL}/${AxiosEndpoint.StreamFile}/${profile.profilePicture}`}
                    />
                  ) : (
                    <AccountCircle />
                  )}
                </IconButton>
                <Menu
                  id="menu-appbar"
                  anchorEl={anchorEl}
                  anchorOrigin={{
                    vertical: "top",
                    horizontal: "right",
                  }}
                  keepMounted
                  transformOrigin={{
                    vertical: "top",
                    horizontal: "right",
                  }}
                  open={Boolean(anchorEl)}
                  onClose={handleClose}
                >
                  {auth ? (
                    <div>
                      <MenuItem
                        onClick={() => {
                          navigate(`${Endpoint.Profile}`);
                        }}
                      >
                        Profile
                      </MenuItem>
                      <MenuItem onClick={handleLogout}>Logout</MenuItem>
                    </div>
                  ) : (
                    <MenuItem onClick={handleLogin}>Login</MenuItem>
                  )}
                </Menu>
              </div>
            </Toolbar>
          </AppBar>
        </Fade>
      )}
    </>
  );
}
