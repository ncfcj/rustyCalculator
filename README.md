# rustyCalculator

Calculadora desktop simples em **Rust**, feita para aprender a linguagem e GUI nativa no Windows.

Interface com **egui + eframe** — código da app 100% Rust (sem HTML/JS).

## Pré-requisitos

1. [Rust](https://rustup.rs/) (stable)
2. No Windows: [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) com workload **Desktop development with C++** (linker `link.exe`)

## Rodar

```bash
cargo run
```

Testes da lógica (sem abrir a janela):

```bash
cargo test
```

## Estrutura (para estudar)

| Arquivo | Papel |
|---------|--------|
| `src/main.rs` | Abre a janela (`eframe`) |
| `src/app.rs` | Botões, display e teclado (`egui`) |
| `src/calculator.rs` | Contas e estado — **sem UI** |

Ideia central: a UI só chama métodos de `Calculator`; a matemática fica testável à parte.

## GUI em Rust no Windows — é 100% Rust?

Sim, para o código que você escreve. Opções comuns:

| Stack | UI em Rust? | Notas |
|-------|-------------|--------|
| **egui / eframe** (este projeto) | Sim | Immediate mode, ótimo para aprender |
| **iced** | Sim | Arquitetura Elm (mensagem → update → view) |
| **Slint** | Quase | DSL `.slint` + lógica em Rust |
| **Tauri** | Não na UI | Webview (HTML/CSS/JS) + Rust no backend |
| **windows-rs** | Bindings | Win32/WinUI nativo; curva mais íngreme |

egui desenha com OpenGL/wgpu por baixo; isso é detalhe da plataforma. Sua calculadora, estado e eventos ficam em Rust.

## Próximos passos de estudo

1. Ler `calculator.rs` e os testes
2. Mudar o layout ou cores em `app.rs`
3. Adicionar `%` ou histórico de operações
4. Experimentar `iced` numa branch para comparar estilos de UI

## Graphify

Este repo usa [graphify](https://github.com/graphify-ai/graphify) para mapa do código (`graphify-out/`). Antes de explorar com grep, prefira:

```bash
graphify query "onde está a lógica de divisão?"
graphify update .
```
