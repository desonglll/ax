import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import axios from "axios";


const host = window.location.hostname;
axios.defaults.withCredentials = true;
console.log(host);
axios.defaults.baseURL = `http://${host}:8000/api`;
ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <App/>
    </React.StrictMode>,
)
