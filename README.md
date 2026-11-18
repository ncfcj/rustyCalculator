# rustyCalculator

Calculadora desktop com **Tauri 2**: UI web (Vite + JS) + lógica em **Rust**.

Feita para aprender o padrão mais comum de apps desktop com Rust no mercado.

## Arquitetura

```text
Frontend (WebView)          Backend (Rust)
─────────────────          ────────────────
botões / display    ──invoke──▶  #[tauri::command]
                         │
                         ▼
                   Calculator (estado)
```

| Pasta | Papel |
|-------|--------|
| `src/` | HTML/CSS/JS (Vite) |
| `src-tauri/src/calculator.rs` | Contas — **sem UI** |
| `src-tauri/src/lib.rs` | Commands Tauri + `Mutex<Calculator>` |
| `src-tauri/capabilities/` | Permissões IPC (menor privilégio) |

## Pré-requisitos

1. [Rust](https://rustup.rs/) (stable)
2. [Node.js](https://nodejs.org/) (npm)
3. Windows: [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (já vem na maioria dos Windows 10/11)
4. Windows: **MSVC** Build Tools + Windows SDK (obrigatório para Tauri; MinGW falha no link)

No PowerShell, abra o ambiente MSVC antes de compilar (ou use o “x64 Native Tools Command Prompt”):

```powershell
& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=amd64
```

## Rodar

```bash
npm install
npm run tauri dev
```

Build de produção:

```bash
npm run tauri build
```

Testes da lógica Rust:

```bash
cd src-tauri
cargo test
```

## Estudar na ordem

1. `src-tauri/src/calculator.rs` — domínio puro
2. `src-tauri/src/lib.rs` — bridge `invoke` ↔ Rust
3. `src/main.js` — cada botão chama um command
4. `src-tauri/capabilities/default.json` — o que o frontend pode chamar

## Graphify

```bash
graphify query "onde estão os commands Tauri?"
graphify update .
```
