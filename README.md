Aqui está um arquivo `README.md` completo, robusto e profissional para o seu repositório. Ele foi estruturado para servir tanto como documentação de engenharia de software quanto como relatório científico, destacando a escolha do Rust, as restrições físicas do integrador e o alinhamento com as diretrizes de pesquisa do grupo.

Para adicioná-lo ao seu projeto, basta criar ou sobrescrever o arquivo `README.md` na raiz do seu diretório (`/Users/akivareis/tmp/galaxy_simulator/README.md`).

---

```markdown
# Simulador de Colisões de Galáxias (N-Corpos)

[cite_start]Este projeto consiste em um simulador dinâmico de N-corpos projetado para modelar a evolução e colisão de galáxias (foco em *Minor Mergers*)[cite: 2, 10, 257]. [cite_start]O núcleo computacional de alta performance e evolução temporal é desenvolvido do zero em **Rust**, enquanto os pipelines de geração de condições iniciais (IC) e análise morfológica estatística quantitativa são acoplados em **Python**[cite: 297, 340, 374, 400].

[cite_start]O projeto está inserido nas atividades do **Grupo de Iniciação Científica em Astronomia** da **UNINTER**, sob a orientação do Prof. Daniel Guimarães Tedesco[cite: 1, 3, 5].

---

## 🌌 Visão Geral do Pipeline Científico

[cite_start]A pesquisa adota uma estratégia de **dois pipelines convergentes** para responder se a dinâmica gravitacional pura de N-corpos isolados é suficiente para explicar as perturbações morfológicas observadas no universo em grande escala[cite: 385, 393]:

1. [cite_start]**Abordagem Cosmológica (TNG):** Mineração e catalogação de galáxias reais que sofreram *minor mergers* ($1/10 < q < 1/4$) na simulação hidroดินâmica *Illustris TNG*[cite: 257, 346, 368].
2. [cite_start]**Abordagem Isolada (Rust):** Execução de uma grade de 36 simulações controladas e puramente gravitacionais com este simulador[cite: 406].

[cite_start]Ambos os caminhos convergem na análise estatística morfológica via parâmetros **CAS** (Concentração, Assimetria e Suavidade) e **Índice de Sérsic**[cite: 370, 372, 385].

---

## 🛠️ Arquitetura do Projeto

[cite_start]O repositório está organizado de forma modular para garantir a separação de escopos e a reprodutibilidade dos dados acadêmicos[cite: 455, 460]:

```text
galaxy_simulator/
├── Cargo.toml             # Metadados e dependências do projeto Rust
├── README.md              # Documentação do projeto (este arquivo)
├── src/                   # Código-fonte principal em Rust
│   ├── main.rs            # Ponto de entrada (Loop e controle de snapshots)
│   ├── lib.rs             # Interface de biblioteca para testes externos
│   ├── physics.rs         # Núcleo numérico (Leapfrog, Plummer e 3ª Lei de Newton)
│   ├── io.rs              # Leitura de Condições Iniciais e escrita de Snapshots
│   └── analytics.rs       # Diagnósticos de conservação física
├── tests/                 # Testes de integração (validação automatizada)
│   └── physics_tests.rs   # Teste de estabilidade orbital (Kepler de longo termo)
└── scripts/               # Scripts de suporte e análise em Python
    ├── generate_ic.py     # Geração de condições iniciais de teste
    └── plot_analytics.py  # Geração de gráficos de conservação de energia/momento

```

---

## 🔬 Fundamentos Físicos e Implementação

### 1. Integrador Simplético Leapfrog (Kick-Drift-Kick)

Diferente de integradores comuns como Euler ou Runge-Kutta 4 (RK4), que dissipam ou injetam energia artificialmente em sistemas conservativos ao longo do tempo, este simulador adota o método **Leapfrog de 2ª ordem**. Ele preserva a estrutura geométrica do espaço de fase, fazendo com que o erro de energia oscile em torno do valor correto sem sofrer deriva (*drift*).

O loop avança a cada passo temporal $\Delta t$ por meio de:

1. 
**Meio-Kick:** $\vec{v}^{n+1/2} = \vec{v}^{n} + \vec{a}^{n} \frac{\Delta t}{2}$ 


2. 
**Drift:** $\vec{r}^{n+1} = \vec{r}^{n} + \vec{v}^{n+1/2} \Delta t$ 


3. 
**Recálculo de Forças:** $\vec{a}^{n+1} = f(\vec{r}^{n+1})$ 


4. 
**Meio-Kick Final:** $\vec{v}^{n+1} = \vec{v}^{n+1/2} + \vec{a}^{n+1} \frac{\Delta t}{2}$ 



### 2. Suavização de Força (Softening de Plummer)

Para evitar instabilidades numéricas e velocidades infinitas quando duas partículas se aproximam demasiadamente ($r \to 0$), a Lei da Gravitação é modificada introduzindo o parâmetro de amaciamento $\epsilon$:

$$\vec{F}_{ij} = -\frac{G m_{i}m_{j}(\vec{r}_{i} - \vec{r}_{j})}{(|\vec{r}_{i} - \vec{r}_{j}|^{2} + \epsilon^{2})^{3/2}}$$



### 3. Otimização via 3ª Lei de Newton

O motor de cálculo de acelerações aproveita o princípio de ação e reação ($\vec{F}_{ij} = -\vec{F}_{ji}$). Ao calcular a força que a partícula $j$ faz em $i$, o vetor simétrico correspondente é imediatamente aplicado a $j$ escalonado por sua massa, cortando o custo computacional do loop clássico $O(N^2)$ pela metade.

---

## 🚀 Como Executar o Pipeline de Validação

### Pré-requisitos

* **Rust:** Através do `cargo` (Edição 2021).
* **Python 3:** Com as bibliotecas `numpy`, `matplotlib` e `json`.

### Passo a Passo

**1. Gerar as Condições Iniciais (IC):**
Cria um arquivo de teste do problema de Kepler com dois corpos massivos em órbita estável.

```bash
python3 scripts/generate_ic.py

```

**2. Compilar e Executar o Simulador em Rust:**
Execute obrigatoriamente a partir da **raiz do projeto** (onde está o `Cargo.toml`). O simulador carregará os dados de entrada, evoluirá o sistema no tempo e despejará snapshots temporais estruturados em JSON no diretório `/tmp`.

```bash
cargo run

```

**3. Processar Gráficos de Conservação e Diagnóstico:**
Lê a série de snapshots gerada pelo Rust e renderiza as curvas de energia cinética, potencial, flutuação simplética da energia total, momento angular e deriva de centro de massa.

```bash
python3 scripts/plot_analytics.py

```

**4. Rodar Testes de Integração Automatizados:**
Dispara o script de validação de longo termo (10.000 passos) para garantir que o erro relativo da energia se mantenha estritamente abaixo da tolerância estipulada de 0.5%.

```bash
cargo test

```

---

## 📈 Metas de Produção e Relatórios

Os arquivos e gráficos gerados por este ambiente servem como comprovação para o **Relatório Trimestral de Duas Páginas** e marcos semestrais exigidos pela instituição.

A estabilidade comprovada nesta Fase 1 (Camada 1) valida o simulador para progredir em direção à **Camada 2 e Camada 3**, onde os arquivos gerados em Rust passarão pelo pipeline automatizado de morfologia estatística quantitativa `statmorph` visando a publicação final na **MNRAS** (Monthly Notices of the Royal Astronomical Society).

```

```
