/**
 * Frontend da calculadora.
 *
 * Cada botão chama um comando Rust via `invoke`.
 * O estado das contas vive no backend (Mutex<Calculator>).
 */
import { invoke } from "@tauri-apps/api/core";

const displayEl = document.getElementById("display");
const padEl = document.getElementById("pad");

/** @type {{ label: string, action: () => Promise<void>, className?: string }[]} */
const keys = [
  { label: "C", className: "fn", action: () => invoke("clear_all") },
  { label: "±", className: "fn", action: () => invoke("toggle_sign") },
  { label: "⌫", className: "fn", action: () => invoke("backspace") },
  { label: "÷", className: "op", action: () => invoke("set_operation", { op: "divide" }) },
  { label: "7", action: () => invoke("input_digit", { digit: "7" }) },
  { label: "8", action: () => invoke("input_digit", { digit: "8" }) },
  { label: "9", action: () => invoke("input_digit", { digit: "9" }) },
  { label: "×", className: "op", action: () => invoke("set_operation", { op: "multiply" }) },
  { label: "4", action: () => invoke("input_digit", { digit: "4" }) },
  { label: "5", action: () => invoke("input_digit", { digit: "5" }) },
  { label: "6", action: () => invoke("input_digit", { digit: "6" }) },
  { label: "−", className: "op", action: () => invoke("set_operation", { op: "subtract" }) },
  { label: "1", action: () => invoke("input_digit", { digit: "1" }) },
  { label: "2", action: () => invoke("input_digit", { digit: "2" }) },
  { label: "3", action: () => invoke("input_digit", { digit: "3" }) },
  { label: "+", className: "op", action: () => invoke("set_operation", { op: "add" }) },
  { label: "0", className: "zero", action: () => invoke("input_digit", { digit: "0" }) },
  { label: ".", action: () => invoke("input_decimal") },
  { label: "=", className: "equals", action: () => invoke("evaluate") },
];

function renderDisplay(state) {
  const pending = state.pending ? `${state.pending} ` : "";
  displayEl.textContent = `${pending}${state.display}`;
}

async function refresh() {
  const state = await invoke("get_display");
  renderDisplay(state);
}

async function runAction(action) {
  try {
    const state = await action();
    renderDisplay(state);
  } catch (err) {
    console.error(err);
    displayEl.textContent = String(err);
  }
}

for (const key of keys) {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = `key${key.className ? ` ${key.className}` : ""}`;
  btn.textContent = key.label;
  btn.addEventListener("click", () => runAction(key.action));
  padEl.appendChild(btn);
}

window.addEventListener("keydown", (event) => {
  const { key } = event;
  if (key >= "0" && key <= "9") {
    event.preventDefault();
    runAction(() => invoke("input_digit", { digit: key }));
  } else if (key === "." || key === ",") {
    event.preventDefault();
    runAction(() => invoke("input_decimal"));
  } else if (key === "+") {
    event.preventDefault();
    runAction(() => invoke("set_operation", { op: "add" }));
  } else if (key === "-") {
    event.preventDefault();
    runAction(() => invoke("set_operation", { op: "subtract" }));
  } else if (key === "*") {
    event.preventDefault();
    runAction(() => invoke("set_operation", { op: "multiply" }));
  } else if (key === "/") {
    event.preventDefault();
    runAction(() => invoke("set_operation", { op: "divide" }));
  } else if (key === "Enter" || key === "=") {
    event.preventDefault();
    runAction(() => invoke("evaluate"));
  } else if (key === "Backspace") {
    event.preventDefault();
    runAction(() => invoke("backspace"));
  } else if (key === "Escape") {
    event.preventDefault();
    runAction(() => invoke("clear_all"));
  }
});

refresh().catch(console.error);
