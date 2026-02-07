import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg('Searching...')
    const url = `https://api.fda.gov/drug/ndc.json?search=generic_name:${name}&limit=100`;
    setGreetMsg(await invoke("fetch_data_from_rust", { url }));
  }

  return (
    <main className="container">
      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/an.png" className="logo vite" alt="aushad-nation" />
        </a>
      </div>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Eg. Paracetamol"
        />
        <button type="submit">Search</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
