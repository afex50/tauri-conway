const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

let testMsgEl;
let testInputEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

await invoke("test_normal");
let efe = await invoke("test_param",{asd:"qsda"});
console.log(typeof(efe));
async function test_command() {
  testMsgEl.textContent = await invoke("testcommand",{lol: testInputEl.value});
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

  testMsgEl = document.querySelector("#test-msg");
  testInputEl = document.querySelector("#test-input");


  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  document.querySelector("#test-form").addEventListener("submit", (e) => {
    e.preventDefault();
    test_command();
  })
});

