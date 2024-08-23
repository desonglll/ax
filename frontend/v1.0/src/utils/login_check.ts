import axios from "axios";


const loginCheck = async () => {
    try {
        const response = await axios.get("login-check");
        return response;  // 确保返回响应数据
    } catch (error) {
        console.error("Error fetching data:", error);
        throw error;  // 可以选择抛出错误以便在调用时处理
    }
};

export default loginCheck;
