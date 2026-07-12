import json
import os
import glob
import gc
import numpy as np
import matplotlib.pyplot as plt

def process_snapshots_low_memory():
    data_dir = "/Users/akivareis/tmp/galaxy_simulator/data/output"
    files = sorted(glob.glob(os.path.join(data_dir, "snapshot_*.json")))
    
    if not files:
        print("Nenhum snapshot encontrado. Verifique o diretório.")
        return

    times, K_list, U_list, E_list = [], [], [], []
    G = 1.0
    
    print(f"Iniciando leitura otimizada de {len(files)} arquivos...")
    print("A memória RAM será limpa (Garbage Collection) a cada iteração.")
    
    for file in files:
        # 1. Carrega apenas um arquivo por vez
        with open(file, 'r') as f:
            data = json.load(f)
        
        t = data['time']
        particles = data['particles']
        N = len(particles)
        
        # Reconstrói o epsilon dinâmico exatamente igual ao Rust
        r_half_mass = 1.0 * (1.0 + np.sqrt(2.0))
        epsilon = (N / 1000.0)**(-0.5) * r_half_mass
        eps_sq = epsilon**2
        
        # 2. Extrai arrays nativos do numpy
        mass = np.array([p['mass'] for p in particles])
        pos = np.array([p['position'] for p in particles])
        vel = np.array([p['velocity'] for p in particles])
        
        # 3. DESTROI O JSON PESADO DA RAM AGORA
        del data
        del particles
        gc.collect()
        
        # Cálculo da Energia Cinética
        K = 0.5 * np.sum(mass * np.sum(vel**2, axis=1))
        
        # Cálculo da Energia Potencial OTIMIZADO (sem matriz 20k x 20k)
        U = 0.0
        for i in range(N - 1):
            dx = pos[i+1:] - pos[i]
            r = np.sqrt(np.sum(dx**2, axis=1) + eps_sq)
            U -= G * mass[i] * np.sum(mass[i+1:] / r)
        
        E = K + U
        times.append(t)
        K_list.append(K)
        U_list.append(U)
        E_list.append(E)
        
        print(f"Lido: {os.path.basename(file)} | E_total: {E:.6f} | RAM Limpa")
        
        # 4. Destroi as variáveis locais do numpy antes do próximo loop
        del mass, pos, vel, dx, r
        gc.collect()

    # --- RENDERIZAÇÃO DO GRÁFICO ---
    E_0 = E_list[0]
    relative_error = [(e - E_0) / abs(E_0) for e in E_list]

    fig, ax = plt.subplots(1, 2, figsize=(14, 5))

    # Gráfico 1: Termodinâmica (Teorema do Virial)
    ax[0].plot(times, K_list, label='Energia Cinética (K)', color='blue')
    ax[0].plot(times, U_list, label='Energia Potencial (U)', color='red')
    ax[0].plot(times, E_list, label='Energia Total (E)', color='black', linewidth=2)
    ax[0].set_title(f"Conservação de Energia (N={N} partículas)")
    ax[0].set_xlabel("Tempo Simulado")
    ax[0].set_ylabel("Energia")
    ax[0].grid(True, linestyle='--', alpha=0.6)
    ax[0].legend()

    # Gráfico 2: Erro Relativo (Diagnóstico de Estabilidade)
    ax[1].plot(times, relative_error, color='purple', linewidth=2)
    ax[1].set_title("Flutuação Simplética (ΔE / |E₀|)")
    ax[1].set_xlabel("Tempo Simulado")
    ax[1].set_ylabel("Erro Relativo")
    ax[1].grid(True, linestyle='--', alpha=0.6)

    plt.tight_layout()
    plt.savefig(os.path.join(data_dir, "plot_conservacao_20k.png"))
    print("\nGráfico salvo em: /Users/akivareis/tmp/galaxy_simulator/data/output/plot_conservacao_20k.png")
    plt.show()

if __name__ == "__main__":
    process_snapshots_low_memory()