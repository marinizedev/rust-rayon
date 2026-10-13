# ⚡ rust-rayon

Repositório complementar ao [`estudy-rust`](https://github.com/marinizedev/estudy-rust), criado na disciplina **Implementação de Estruturas de Dados** do curso de **Análise e Desenvolvimento de Sistemas (ADS)** na UniFECAF.

Aqui o foco sai das estruturas de dados clássicas e vai para três temas práticos: **usar bibliotecas externas (crates)**, **processamento paralelo com Rayon** e **depuração (debug) de programas Rust**. Também inclui uma **comparação direta com Python** para entender as diferenças de abordagem entre as duas linguagens.

---

## 🎯 Objetivo

- Aprender a **instalar e usar bibliotecas externas** em Rust com o Cargo.
- Ver na prática como o **paralelismo** com a crate [Rayon](https://crates.io/crates/rayon) acelera o processamento de grandes volumes de dados.
- Entender a importância de **escolher o tipo numérico correto** em Rust e comparar com o comportamento do Python.
- Aprender a **depurar** código Rust com o `rust-gdb`, usando breakpoints.

Todo o desenvolvimento foi feito no **GitHub Codespaces**.

---

## 📂 Estrutura do repositório

```
rust-rayon/
├── Cargo.toml              # Workspace: reúne os projetos Rust
├── Cargo.lock              # Versões exatas das dependências
├── .gitignore              # Ignora a pasta target/
├── README.md
├── rustrayon/              # Projeto com a crate Rayon
│   ├── Cargo.toml
│   └── src/main.rs
├── comparationpython.py    # Versão equivalente em Python para comparação
└── calc/                   # Projeto simples para praticar debug
    ├── Cargo.toml
    └── src/main.rs
```

Cada projeto Rust tem seu próprio `Cargo.toml` e sua pasta `src/`. O `Cargo.toml` da raiz transforma o repositório em um **workspace** do Cargo: os projetos compartilham o mesmo `Cargo.lock` e a mesma pasta `target/` (gerada na compilação e ignorada pelo Git). Por isso, o executável do `calc` fica em `target/debug/calc`, na raiz do repositório.

---

## 📚 Conteúdo

### 1. `rustrayon/` — Processamento paralelo com Rayon

Programa que trabalha com os números de 1 até **10.000.000** e executa duas tarefas **em paralelo**:

1. **Soma dos quadrados** de todos os números, usando `par_iter()`, `map` e `sum`.
2. **Filtro dos números pares**, coletados em um novo vetor com `par_iter()`, `filter`, `cloned` e `collect`.

O Rayon transforma iteradores comuns em iteradores paralelos, distribuindo o trabalho entre os núcleos do processador com mudanças mínimas no código (basta trocar `iter()` por `par_iter()`).

**Instalando a biblioteca externa:**

```bash
cargo add rayon
```

Isso adiciona a dependência ao `Cargo.toml`. Também é possível escrevê-la manualmente na seção `[dependencies]`.

**Ponto importante: escolha do tipo numérico**

A aula começou com o intervalo de 1 até 1.000.000, e o resultado da soma dos quadrados cabia perfeitamente em um `i64`. Ao aumentar para 10.000.000, o resultado ultrapassou o limite do `i64` (*overflow*), e foi preciso usar o `u128`, que comporta números muito maiores.

> 💡 Em Rust, o programador define o tipo e é responsável por garantir que ele comporta o resultado. Isso traz **controle, previsibilidade e segurança**.

### 2. `comparationpython.py` — Comparação com Python

Mesma lógica do programa em Rust, escrita em Python de forma **sequencial**: soma dos quadrados e filtro dos pares para os números de 1 até 10.000.000.

| Aspecto | Rust | Python |
|---------|------|--------|
| Tipos numéricos | Definidos manualmente (`i32`, `i64`, `u128`...) | Inteiros de tamanho ilimitado, tratados automaticamente |
| Paralelismo | Simples com Rayon (`par_iter`) | Versão sequencial neste exemplo |
| Execução | Código compilado, mais rápido | Código interpretado, mais lento em cálculos pesados |
| Preocupação com *overflow* | Responsabilidade do programador | Resolvida "por trás" pela linguagem |

A conclusão da aula: o Python é mais cômodo, e o Rust exige mais atenção aos detalhes, mas entrega **mais robustez e desempenho**.

### 3. `calc/` — Prática de debug

Um programa propositalmente simples (soma de duas variáveis e impressão do resultado), usado para treinar o fluxo de depuração com o **GDB** adaptado para Rust.

**Passo a passo:**

```bash
# 1. Na raiz do repositório, compile o calc em modo debug
cargo build -p calc

# 2. Abra o depurador sobre o executável
rust-gdb target/debug/calc
```

> Como o repositório é um workspace, o `target/` fica na raiz. Se você estiver dentro da pasta `calc/`, o caminho do executável passa a ser `../target/debug/calc`.

Dentro do `rust-gdb`:

| Comando | O que faz |
|---------|-----------|
| `break main.rs:5` | Cria um *breakpoint* na linha desejada (troque o número pela linha) |
| `run` | Executa o programa até o primeiro breakpoint |
| `next` | Avança para a próxima linha |
| `print variavel` | Mostra o valor de uma variável |
| `continue` | Continua a execução até o próximo breakpoint |
| `quit` | Sai do depurador |

---

## ⚙️ Preparação do ambiente

Antes de rodar os projetos, é preciso preparar o ambiente. Mesmo no GitHub Codespaces o Rust **não vem instalado**.

### 1. Instalar o Rust

1. Acesse a documentação oficial: [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). Lá fica o comando de instalação atualizado.
2. Execute o comando de instalação do `rustup` no terminal (no Linux/Codespaces):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Carregue o Cargo no terminal atual (ou feche e abra um novo terminal):

```bash
source "$HOME/.cargo/env"
```

4. Confirme se deu certo:

```bash
rustc --version
cargo --version
```

### 2. Garantir o depurador (para o projeto `calc`)

O `rust-gdb` é um atalho que usa o **GDB** por baixo. Se o comando não for encontrado, instale o GDB:

```bash
sudo apt update && sudo apt install -y gdb
```

### 3. Python (para `comparationpython.py`)

Confirme se o Python 3 está disponível:

```bash
python3 --version
```

---

## ▶️ Como executar

Com o ambiente preparado, rode:

```bash
# Rust com Rayon (use --release para ver o ganho real de desempenho)
cd rustrayon
cargo run --release

# Versão Python
python3 comparationpython.py

# Projeto de debug (a partir da raiz do repositório)
cargo build -p calc
rust-gdb target/debug/calc
```

Por ser um workspace, os projetos Rust também podem ser executados a partir da raiz, com `-p`:

```bash
cargo run -p rustrayon --release
cargo run -p calc
cargo build --workspace   # compila todos os projetos de uma vez
```

---

## 🛠️ Tecnologias

- Rust, Cargo e a crate Rayon
- Python 3
- rust-gdb
- GitHub Codespaces

---

## 👩‍💻 Autora

**Marinize Santana**
Estudante de Análise e Desenvolvimento de Sistemas (UniFECAF)

- GitHub: [github.com/marinizedev](https://github.com/marinizedev)
- LinkedIn: [linkedin.com/in/marinize-santana-47bb2b372](https://linkedin.com/in/marinize-santana-47bb2b372)
