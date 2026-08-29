# Simulador de Colisões de Galáxias (N-Corpos)

Este projeto consiste em um simulador dinâmico de N-corpos projetado para modelar a evolução e colisão de galáxias, com foco em *Minor Mergers*. O núcleo computacional de alta performance é desenvolvido em **Rust**, enquanto os pipelines de geração de condições iniciais (IC), animação e análise morfológica são acoplados em **Python**.

O projeto integra as atividades do **Grupo de Iniciação Científica em Astronomia** da **UNINTER**, sob a orientação do Prof. Daniel Guimarães Tedesco.

---

## 🌌 Visão Geral do Pipeline Científico

A pesquisa adota uma estratégia de **dois pipelines convergentes** para avaliar se a dinâmica gravitacional de N-corpos isolados explica as perturbações morfológicas observadas no universo em grande escala:

1. **Abordagem Cosmológica (TNG):** Mineração de galáxias reais que sofreram *minor mergers* ($1/10 < q < 1/4$) na simulação hidrodinâmica *Illustris TNG*.
2. **Abordagem Isolada (Rust):** Execução de uma grade de simulações controladas puramente gravitacionais com o motor de alta performance do repositório[cite: 3].

Ambos convergem na análise morfológica via parâmetros **CAS** (Concentração, Assimetria e Suavidade) e **Índice de Sérsic**[cite: 3].

---

## 🛠️ Arquitetura do Projeto

O repositório está organizado de forma modular, agora unificado por um arquivo de configuração central:

```text
galaxy_simulator/
├── Cargo.toml                 # Metadados e dependências do projeto Rust[cite: 3]
├── config.json                # (Novo) Fonte única de verdade para parâmetros físicos (G, theta, eta)
├── README.md                  # Documentação do projeto (este arquivo)[cite: 3]
├── src/                       # Código-fonte principal em Rust[cite: 3]
│   ├── main.rs                # Ponto de entrada e orquestração do loop adaptativo[cite: 3]
│   ├── physics.rs             # Núcleo numérico (Barnes-Hut Octree, Leapfrog CFL)[cite: 3]
│   └── io.rs                  # Exportação otimizada de snapshots iterativos[cite: 3]
└── scripts/                   # Scripts de suporte e análise em Python[cite: 3]
    ├── generate_ic_merger.py  # Geração de condições iniciais para colisão (Minor Merger)
    ├── plot_analytics.py      # Diagnóstico de conservação de energia com GC otimizado[cite: 3]
    └── animate_collision.py   # Renderização visual (GIF) do espaço tridimensional

🔬 Fundamentos Físicos e Implementação

1. Aproximação Espacial em Árvore (Barnes-Hut $O(N \log N)$)

O loop clássico de força bruta $O(N^2)$ foi substituído por uma Octree de Barnes-Hut. O espaço tridimensional é recursivamente particionado em octantes. Se a razão entre o tamanho de um nó ($s$) e a distância até a partícula ($d$) for menor que o limite de abertura $\theta$, o aglomerado de partículas daquela região é aproximado como um único centro de massa.A árvore é construída utilizando alocação contígua (Arena Allocation) para evitar overhead de ponteiros e proteger contra stack overflows de galáxias densas.2. Passo Temporal Adaptativo (Courant-Friedrichs-Lewy / CFL)O avanço do tempo deixou de ser constante. A cada iteração espacial, o motor captura o pico de aceleração gravitacional do sistema e calcula o próximo passo de tempo ($\Delta t$) com base na tolerância $\eta$. Isso previne saltos espaciais irreais no núcleo denso (perfil de Hernquist) sem comprometer o avanço temporal das partículas nas bordas periféricas.

