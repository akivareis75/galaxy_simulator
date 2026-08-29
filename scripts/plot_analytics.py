import json
import os
import glob
import gc
import numpy as np
import matplotlib.pyplot as plt


def load_config(config_path="config.json"):
    if not os.path.exists(config_path):
        raise FileNotFoundError(
            f"Arquivo de configuração não encontrado: {config_path}")
    with open(config_path, "r") as f:
        return json.load(f)


def process_snapshots_low_memory():
    # 1. Injeção de parâmetros via configuração unificada
    config = load_config()
    data_dir = config.get("output_dir", "./snapshots")
    epsilon = config.get("softening_epsilon", 0.05)
    eps_sq = epsilon**2

    files = sorted(glob.glob(os.path.join(data_dir, "snapshot_*.json")))

    if not files:
        print(f"Nenhum snapshot encontrado em: {data_dir}")
        return

    steps, K_list, U_list, E_list = [], [], [], []
    G = 1.0

    print(f"Iniciando leitura otimizada de {len(files)} arquivos...")
    print(f"Parâmetros Físicos: G={G}, Epsilon={epsilon}")

    for file in files:
        # Extrai o passo simulado do nome do arquivo (ex: snapshot_000100.json -> 100)
        filename = os.path.basename(file)
        step_str = filename.replace("snapshot_", "").replace(".json", "")
        step = int(step_str)

        # O motor em Rust exporta diretamente a lista de partículas
        with open(file, 'r') as f:
            particles = json.load(f)

        N = len(particles)

        # 2. Extrai arrays nativos do numpy
        mass = np.array([p['mass'] for p in particles])
        pos = np.array([p['position'] for p in particles])
        vel = np.array([p['velocity'] for p in particles])

        # 3. Libera o peso estrutural do JSON da RAM imediatamente
        del particles
        gc.collect()

        # Cálculo da Energia Cinética (K = 1/2 m v^2)
        K = 0.5 * np.sum(mass * np.sum(vel**2, axis=1))

        # Cálculo da Energia Potencial (U) - Loop otimizado sem matriz NxN
        U = 0.0
        for i in range(N - 1):
            dx = pos[i+1:] - pos[i]
            r = np.sqrt(np.sum(dx**2, axis=1) + eps_sq)
            U -= G * mass[i] * np.sum(mass[i+1:] / r)

        E = K + U
        steps.append(step)
        K_list.append(K)
        U_list.append(U)
        E_list.append(E)

        print(f"Lido: {filename} | E_total: {E:.6f}")

        # 4. Destroi buffers locais temporários do numpy
        del mass, pos, vel, dx, r
        gc.collect()

    # --- RENDERIZAÇÃO E DIAGNÓSTICO ---
    E_0 = E_list[0]
    relative_error = [(e - E_0) / abs(E_0) for e in E_list]

    fig, ax = plt.subplots(1, 2, figsize=(14, 5))

    # Gráfico 1: Termodinâmica e Conservação
    ax[0].plot(steps, K_list, label='Energia Cinética (K)', color='blue')
    ax[0].plot(steps, U_list, label='Energia Potencial (U)', color='red')
    ax[0].plot(steps, E_list, label='Energia Total (E)',
               color='black', linewidth=2)
    ax[0].set_title(f"Conservação de Energia (N={N})")
    ax[0].set_xlabel("Passo de Integração")
    ax[0].set_ylabel("Energia")
    ax[0].grid(True, linestyle='--', alpha=0.6)
    ax[0].legend()

    # Gráfico 2: Flutuação do Integrador
    ax[1].plot(steps, relative_error, color='purple', linewidth=2)
    ax[1].set_title("Flutuação da Energia Total (ΔE / |E₀|)")
    ax[1].set_xlabel("Passo de Integração")
    ax[1].set_ylabel("Erro Relativo")
    ax[1].grid(True, linestyle='--', alpha=0.6)

    plt.tight_layout()
    output_plot = os.path.join(data_dir, f"plot_conservacao_{N}.png")
    plt.savefig(output_plot)
    print(f"\nGráfico diagnóstico de física salvo em: {output_plot}")

    # plt.show() # Descomente se desejar abrir a janela interativa no macOS


if __name__ == "__main__":
    process_snapshots_low_memory()
