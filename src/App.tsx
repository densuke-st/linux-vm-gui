import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [dockerState, setDockerState] = useState(""); // Docker環境の状態を持つ
  const [vmState, setVmState] = useState(""); // VMの状態を持つ

  async function getDockerState(){
    await setDockerState("ここでDockerの状態を表示");
  }

  async function getVmState(){
    await setVmState("ここでVMの状態を表示");
  }


  useEffect(() => {
    // Dockerの状態を取得、更新

    setInterval(() => {
      getDockerState()
    }, 2000)
    setInterval(() => {
      getVmState()
    }, 2000)
  })

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>授業用LinuxVM管理</h1>
        <p>Docker: {dockerState}</p>
        <p>VM状態: {vmState}</p>
    </div>
  );
}

export default App;
