import axios from "axios";
import { AxiosEndpoint } from "../libs/axios_endpoint";

const loginCheck = async () => {
  try {
    const resp = await axios.get(AxiosEndpoint.LoginCheck);
    if (resp.data.code === 200) {
      return true;
    }
    return false;
  } catch (error) {
    console.error("Error fetching data:", error);
    throw error; // 可以选择抛出错误以便在调用时处理
  }
};

export default loginCheck;
