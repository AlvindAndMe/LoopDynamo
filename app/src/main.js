import { invoke } from "@tauri-apps/api/core";

let greetInputEl;
let greetMsgEl;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", {
    name: greetInputEl.value
  });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

  document
    .querySelector("#greet-form")
    .addEventListener("submit", (e) => {
      e.preventDefault();
      greet();
    });

  invoke("run_flow", { json: "{}" })
    .then(result => console.log("C++ says:", result))
    .catch(err => console.error(err));
});
