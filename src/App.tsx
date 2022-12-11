import { createSignal, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [viewAoc, setViewAoc] = createSignal(false);
  const [viewRps, setViewRps] = createSignal(false);

  const [scoreboard, setScoreboard] = createSignal({
    wins: 0,
    losses: 0,
    draws: 0,
  });
  const [aoc, setAoc] = createSignal(0);
  const [aocTextarea, setAocTextarea] = createSignal("", { equals: false });

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  async function play(hand: string) {
    await invoke("play_game", { hand });
    setScoreboard(await invoke("show_scoreboard"));
  }

  async function reset() {
    await invoke("reset_scoreboard");
    setScoreboard({ wins: 0, losses: 0, draws: 0 });
  }

  async function aocGuide(guide: string) {
    try {
      const results: number = await invoke("aoc", { guide });
      setAoc(results);
    } catch (err) {
      if (typeof err === "string") {
        setAocTextarea(err);
      }
    }
  }

  return (
    <div class="container">
      <h1>Advent of Code 2022 - Day 2</h1>
      <a target="_blank" href="https://adventofcode.com/2022/day/2">
        https://adventofcode.com/2022/day/2
      </a>

      <div class="row">
        <button onClick={() => setViewAoc(!viewAoc())}>Toggle AOC</button>
      </div>
      <Show when={viewAoc()}>{renderAoc()}</Show>

      <h1>Play game</h1>

      <div class="row">
        <button onClick={() => setViewRps(!viewRps())}>Toggle RPS</button>
      </div>
      <Show when={viewRps()}>{renderRps()}</Show>
    </div>
  );

  function renderAoc() {
    return (
      <>
        <p>Input strategy guide:</p>
        <div class="row">
          <textarea
            value={aocTextarea()}
            onChange={(input) => aocGuide(input.currentTarget.value)}
          />
        </div>
        <p>Score: {aoc}</p>
      </>
    );
  }

  function renderRps() {
    return (
      <>
        <div class="row">
          <button onClick={() => play("Rock")}>
            <img
              src="https://twemoji.maxcdn.com/svg/270a.svg"
              class="logo rock"
              alt="Rock logo"
            />
          </button>
          <button onClick={() => play("Paper")}>
            <img
              src="https://twemoji.maxcdn.com/svg/270b.svg"
              class="logo paper"
              alt="Paper logo"
            />
          </button>
          <button onClick={() => play("Scissors")}>
            <img
              src="https://twemoji.maxcdn.com/svg/270c.svg"
              class="logo scissors"
              alt="Scissors logo"
            />
          </button>
        </div>

        <div class="row">
          <button onClick={() => reset()}>Reset</button>
        </div>

        <p>Scoreboard -- from Rust!</p>
        <div class="row">
          <table>
            <thead>
              <tr>
                <th>Won</th>
                <th>Lost</th>
                <th>Draw</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>{scoreboard().wins}</td>
                <td>{scoreboard().losses}</td>
                <td>{scoreboard().draws}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </>
    );
  }
}

export default App;
