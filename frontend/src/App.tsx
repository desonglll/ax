import './App.css'
import React from "react";
import axios from 'axios';

function App() {

    const login = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()

        // 创建 FormData 实例
        const formData = new FormData(e.currentTarget);

        // 获取 user_id 字段的值
        const userId = Number(formData.get('user_id'));

        console.log('user_id:', userId);

        axios.post(`login`, {
            user_id: userId
        })
    }

    return (
        <>

            <form onSubmit={login}>
                <div>{axios.defaults.baseURL}</div>
                <div className={"log"}>
                    <input className={"input"} name={"user_id"} type={"number"}/>
                    <button type={"submit"}>Login</button>
                </div>
            </form>
        </>
    )
}

export default App
