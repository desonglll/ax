import axios from "axios";

const api = axios.create({
  baseURL: "/api",
  withCredentials: true,
});

const getData = async (
  endpoint: string,
  method: string = "GET",
  data?: unknown,
  params?: Record<string, string | number>,
) => {
  try {
    const response = await api({
      url: endpoint,
      method,
      data,
      params,
    });
    return response.data;
  } catch (error) {
    console.error("Error fetching data:", error);
    throw error;
  }
};

export default getData;
