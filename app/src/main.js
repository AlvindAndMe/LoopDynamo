import { invoke } from "@tauri-apps/api/core";

window.addEventListener("DOMContentLoaded", () => {

  invoke("ld_run_flow_cmd", { json: "{}" })
    .then(result => console.log("C++ says:", result))
    .catch(err => console.error(err));
});
