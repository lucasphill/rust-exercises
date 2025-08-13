
# Plano de Estudos - Rust + Tauri (4 Semanas)

## 📅 Semana 1 – Fundamentos de Rust
🎯 **Objetivo:** dominar o básico da linguagem e do `cargo`.

### Estudo
- Instalar Rust (`rustup`) e configurar ambiente Windows
- Comandos do `cargo` (`new`, `run`, `build`, `check`, `add`)
- Sintaxe básica:
  - Variáveis, mutabilidade, tipos
  - Controle de fluxo (`if`, `match`, `loop`, `for`)
  - Funções
- Ownership, Borrowing e Lifetimes (conceito introdutório)
- Structs e Enums

### Prática
- Exercícios no [Rustlings](https://github.com/rust-lang/rustlings)
- **Projeto:** Conversor de temperatura (Celsius ↔ Fahrenheit)
- **Projeto:** Jogo de adivinhação (usando `rand`)

---

## 📅 Semana 2 – Rust Intermediário
🎯 **Objetivo:** trabalhar com dados, arquivos e crates.

### Estudo
- `Vec`, `HashMap`, iteradores (`map`, `filter`)
- Manipulação de `String`
- Tratamento de erros (`Result`, `Option`, `?`)
- Ler e escrever arquivos (`std::fs`)
- Serialização com `serde` e `serde_json`

### Prática
- **Projeto:** Lista de tarefas (JSON no disco)
- **Projeto:** Contador de palavras em um texto
- Exercícios extras: refatorar usando módulos (`mod`, `pub`, `use`)

---

## 📅 Semana 3 – APIs, CLI e Introdução ao Tauri
🎯 **Objetivo:** integrar Rust com APIs externas e entender Tauri.

### Estudo
- HTTP requests com `reqwest`
- Concorrência (`std::thread`) e assíncronismo (`tokio`)
- Criar CLI com `clap`
- Introdução ao Tauri:
  - Estrutura do projeto (`src-tauri`, frontend)
  - Comunicação frontend ↔ backend (`#[tauri::command]`, `invoke`)

### Prática
- **Projeto Rust:** Conversor de moedas via CLI (busca API e retorna no terminal)
- Criar projeto React + Tauri (`npx create-react-app` + `npx tauri init`)
- Implementar comando Rust que retorna "Hello from Rust" no React

---

## 📅 Semana 4 – Projeto Final com Rust + Tauri
🎯 **Objetivo:** criar um app desktop com Rust backend e React frontend.

### Estudo
- APIs Tauri: `fs`, `dialog`, `window`
- Configuração de build (`tauri.conf.json`)
- Geração de `.exe` e `.msi` para Windows
- Boas práticas de integração Rust + React

### Projeto Final
📌 *App de Anotações Offline*
- Frontend em React com lista de notas
- Backend Rust com:
  - Comandos para adicionar, listar e remover notas
  - Persistência em JSON usando `serde`
- Uso das APIs do Tauri (`dialog` para escolher pasta de salvamento)
- Geração de instalador `.msi`

---

## 💡 Dica Extra  
Se tiver tempo, depois do plano:
- Aprender **WebAssembly** e compilar Rust para rodar no navegador
- Integrar SQLite usando `sqlx` no Rust para dados persistentes
- Adicionar **auto-update** no Tauri
