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
```

---

## 🔬 Fundamentos Físicos e Implementação

1. Aproximação Espacial em Árvore (Barnes-Hut $O(N \log N)$)

  O loop clássico de força bruta $O(N^2)$ foi substituído por uma Octree de Barnes-Hut. O espaço tridimensional é recursivamente particionado em octantes. Se a razão entre o 
tamanho de um nó ($s$) e a distância até a partícula ($d$) for menor que o limite de abertura $\theta$, o aglomerado de partículas daquela região é aproximado como um único centro de massa.
A árvore é construída utilizando alocação contígua (Arena Allocation) para evitar overhead de ponteiros e proteger contra stack overflows de galáxias densas.

2. Passo Temporal Adaptativo (Courant-Friedrichs-Lewy / CFL)

  O avanço do tempo deixou de ser constante. A cada iteração espacial, o motor captura o pico de aceleração gravitacional do sistema e calcula o próximo passo de tempo
($\Delta t$) com base na tolerância $\eta$. Isso previne saltos espaciais irreais no núcleo denso (perfil de Hernquist) sem comprometer o avanço temporal das partículas nas bordas periféricas.

3. Integrador Simplético Leapfrog

  Para garantir que a flutuação simplética da energia não exploda com o $\Delta t$ adaptativo, o integrador preserva a 
  estrutura geométrica dividindo a atualização de velocidades em duas etapas sincronizadas com a nova força:

   * Meio-Kick (avança $v$ com $\Delta t / 2$)
   * Drift (avança $r$ com o passo completo)
   * Recálculo da Árvore Espacial de Forças
   * Meio-Kick Final


4. Suavização de Força (Plummer Softening)

  O parâmetro de amaciamento $\epsilon$ é injetado globalmente para limitar as forças quando partículas se aproximam demasiadamente ($r \to 0$)[cite: 3]:

  
   $$\vec{F}_{ij} = -\frac{G m_{i}m_{j}(\vec{r}_{i} - \vec{r}_{j})}{(|\vec{r}_{i} - \vec{r}_{j}|^{2} + \epsilon^{2})^{3/2}}$$

## 🚀 Como Executar o Pipeline
  
Pré-requisitos: 
  
  * Rust: Através do cargo (Edição 2021) com flag de otimização máxima (--release)[cite: 3].

  * Python 3: Com numpy e matplotlib[cite: 3].

Passo a Passo

1. Gerar Cenário Base:

   Bash

   python3 scripts/generate_ic_merger.py

2. Configurar Simulação:
   
  Ajuste o config.json na raiz apontando o input_file gerado no passo acima e calibrando os limites de aproximação temporal (eta) e espacial (theta).

3. Executar o Motor de N-Corpos:
   
  Sempre rode o comando a partir da raiz do projeto para que o pacote localize o config.json corretamente.
  
  cargo run --release


4. Validar Conservação Física:
   
  Verifique se a configuração limitadora impediu a fuga de energia. O script otimiza a RAM via Garbage Collection nativo a cada iteração de estado.

  python3 scripts/plot_analytics.py


## 📈 Metas de Produção


A estabilidade comprovada nesta nova arquitetura de núcleo de força em malha possibilita avançar aos testes definitivos e à pipeline morfológica statmorph, 
visando a elaboração e publicação final na MNRAS (Monthly Notices of the Royal Astronomical Society)[cite: 3].

    
  

