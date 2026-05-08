import { AxiosEndpoint } from "@/config/endpoints/axios_endpoint";
import getData from "@/utils/data_fetch";

const loginCheck = async () => {
  try {
    const data = await getData(AxiosEndpoint.LoginCheck);
    return data?.code === 200;
  } catch {
    return false;
  }
};

export default loginCheck;
