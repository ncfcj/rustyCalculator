//! Backend Tauri: commands IPC + estado da calculadora.
//!
//! O frontend chama `invoke("input_digit", …)`; aqui atualizamos
//! `Calculator` e devolvemos o snapshot do display.

mod calculator;

use std::sync::Mutex;

use calculator::{Calculator, Operation};
use serde::Serialize;
use tauri::State;

/// Snapshot enviado ao frontend após cada ação.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayState {
    pub display: String,
    pub pending: Option<String>,
}

impl DisplayState {
    fn from_calc(calc: &Calculator) -> Self {
        Self {
            display: calc.display().to_string(),
            pending: calc.pending_op().map(|op| op.symbol().to_string()),
        }
    }
}

fn with_calc<F>(state: &State<'_, Mutex<Calculator>>, f: F) -> Result<DisplayState, String>
where
    F: FnOnce(&mut Calculator),
{
    let mut calc = state.lock().map_err(|_| "Estado bloqueado".to_string())?;
    f(&mut calc);
    Ok(DisplayState::from_calc(&calc))
}

#[tauri::command]
fn get_display(state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    let calc = state.lock().map_err(|_| "Estado bloqueado".to_string())?;
    Ok(DisplayState::from_calc(&calc))
}

#[tauri::command]
fn input_digit(digit: String, state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    let ch = digit
        .chars()
        .next()
        .ok_or_else(|| "Dígito vazio".to_string())?;
    if !ch.is_ascii_digit() {
        return Err(format!("Não é dígito: {digit}"));
    }
    with_calc(&state, |c| c.input_digit(ch))
}

#[tauri::command]
fn input_decimal(state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    with_calc(&state, |c| c.input_decimal())
}

#[tauri::command]
fn set_operation(op: String, state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    let operation = Operation::from_str_op(&op)?;
    with_calc(&state, |c| c.set_operation(operation))
}

#[tauri::command]
fn evaluate(state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    with_calc(&state, |c| c.evaluate())
}

#[tauri::command]
fn clear_all(state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    with_calc(&state, |c| c.clear_all())
}

#[tauri::command]
fn backspace(state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    with_calc(&state, |c| c.backspace())
}

#[tauri::command]
fn toggle_sign(state: State<'_, Mutex<Calculator>>) -> Result<DisplayState, String> {
    with_calc(&state, |c| c.toggle_sign())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Calculator::new()))
        .invoke_handler(tauri::generate_handler![
            get_display,
            input_digit,
            input_decimal,
            set_operation,
            evaluate,
            clear_all,
            backspace,
            toggle_sign
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
