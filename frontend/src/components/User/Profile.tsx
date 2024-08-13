import { Avatar, Box, CircularProgress, Fade, TextField } from "@mui/material";
import type React from "react";
import { useEffect, useState } from "react";
import getData from "../../utils/data_fetch";
import type { UpdateUserRequest, User } from "../../models/user";
import { LoadingOutlined, PlusOutlined } from "@ant-design/icons";
import axios from "axios";
import { Upload } from "antd";
import { Button } from "@mui/joy";

function Profile() {
  //希望 user 的初始状态是一个空对象，并且你不想提供所有字段
  const [user, setUser] = useState<Partial<User>>({});
  const [imageUuid, setImageUuid] = useState("null");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    getData("user/profile")
      .then((resp) => {
        if (resp.data.code === "Success") {
          console.log(resp.data.body.data);
          setUser(resp.data.body.data);
          setImageUuid(resp.data.body.data.profilePicture);
        }
      })
      .then(() => {
        setLoading(false);
      });
  }, []);

  const handleOnSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const formData = new FormData(form);

    // 提取具体的值
    const userName = formData.get("userName") as string;
    const password = formData.get("password") as string;
    const email = formData.get("email") as string;
    const fullName = formData.get("fullName") as string;
    const phone = formData.get("phone") as string;
    const data: UpdateUserRequest = {
      id: Number(user.id),
      userName: userName,
      email: email,
      password: password === "" ? undefined : password,
      isActive: true,
      isAdmin: false,
      fullName: fullName,
      phone: phone,
      profilePicture: imageUuid,
    };
    console.log(data);

    try {
      axios.post("user/update", data).then((resp) => {
        if (resp.data.code === "Success") {
          console.log(resp.data);
        }
      });
    } catch (err) {
      console.log(err);
    }
  };
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const customRequest = (info: any) => {
    console.log(info);
    try {
      const data = {
        file: info.file,
      };
      axios
        .post("upload", data, {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        })
        .then((resp) => {
          console.log(resp.data);
          // Assuming your backend returns the uploaded image URL
          if (resp.data.code === "Success") {
            const uuid = resp.data.body.data[0].id;
            setImageUuid(uuid);
          }
        });
    } catch (e) {
      console.log(e);
    }
  };
  const uploadButton = (
    <button style={{ border: 0, background: "none" }} type="button">
      {loading ? <LoadingOutlined /> : <PlusOutlined />}
      <div style={{ marginTop: 8 }}>Upload</div>
    </button>
  );
  return (
    <>
      {loading ? (
        <div>
          <Box
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              marginTop: "10px",
            }}
          >
            <CircularProgress />
          </Box>
        </div>
      ) : (
        <Fade in={!loading}>
          <Box
            sx={{
              marginTop: "50px",
              width: "100%",
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
            }}
          >
            <Upload
              name="avatar"
              listType="picture-circle"
              className="avatar-uploader"
              showUploadList={false}
              customRequest={customRequest}
            >
              {imageUuid ? (
                <Avatar
                  src={`${axios.defaults.baseURL}/stream/${imageUuid}`}
                  //   alt="avatar"
                  style={{ width: "100%", height: "100%" }}
                />
              ) : (
                uploadButton
              )}
            </Upload>
            <Box
              component={"form"}
              sx={{
                width: "50%",
                display: "flex",
                flexDirection: "column",
              }}
              onSubmit={handleOnSubmit}
            >
              <TextField
                label={"User Name"}
                variant="standard"
                defaultValue={user?.userName}
                name="userName"
              />
              <TextField
                label={"Password"}
                variant="standard"
                name="password"
              />
              <TextField
                label={"Email"}
                variant="standard"
                defaultValue={user?.email}
                name="email"
              />
              <TextField
                label={"Full Name"}
                variant="standard"
                defaultValue={user?.fullName}
                name="fullName"
              />
              <TextField
                label={"Phone"}
                variant="standard"
                defaultValue={user?.phone}
                name="phone"
              />
              <Button type="submit" sx={{ marginTop: "20px" }}>
                Submit
              </Button>
            </Box>
          </Box>
        </Fade>
      )}
    </>
  );
}

export default Profile;
