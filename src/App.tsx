import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [result, setResult] = createSignal("");
  const [hand, setHand] = createSignal("");

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  async function greet(hand: string) {
    setHand(`You played ${hand}`);
    setResult(await invoke("game", { hand }) + " -- from Rust!");
  }

  return (
    <div class="container">
      <h1>Welcome!</h1>

      <div class="row">
        <button onClick={() => greet("Rock")}>
          <img src="https://twemoji.maxcdn.com/svg/270a.svg" class="logo vite" alt="Vite logo" />
        </button>
        <button onClick={() => greet("Paper")}>
          <img src="https://twemoji.maxcdn.com/svg/270b.svg" class="logo tauri" alt="Tauri logo" />
        </button>
        <button onClick={() => greet("Scissors")}>
          <img src="https://twemoji.maxcdn.com/svg/270c.svg" class="logo solid" alt="Solid logo" />
        </button>
      </div>

      <p>{hand}</p>

      <p>{result}</p>
    </div>
  );
}

export default App;
