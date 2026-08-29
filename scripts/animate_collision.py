import json
import os
import glob
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation


def load_config(config_path="config.json"):
    if not os.path.exists(config_path):
        raise FileNotFoundError(
            f"Arquivo config não encontrado: {config_path}")
    with open(config_path, "r") as f:
        return json.load(f)


def generate_animation():
    # 1. Busca o diretório de saída parametrizado no config.json
    try:
        config = load_config()
        data_dir = config.get("output_dir", "./snapshots")
    except FileNotFoundError:
        print(
            "[ERRO] Arquivo config.json não encontrado. Execute a partir da raiz do projeto.")
        return

    files = sorted(glob.glob(os.path.join(data_dir, "snapshot_*.json")))

    if not files:
        print(f"[ERRO] Nenhum snapshot encontrado em: {data_dir}")
        return

    print(f"[{len(files)} snapshots encontrados] Preparando a renderização visual...")

    fig, ax = plt.subplots(figsize=(8, 8), facecolor='black')
    ax.set_facecolor('black')

    # Ajuste o limite espacial da visualização se a galáxia expandir muito
    lim = 15.0
    ax.set_xlim(-lim, lim)
    ax.set_ylim(-lim, lim)
    ax.set_aspect('equal')
    ax.axis('off')

    title = ax.text(0.5, 0.95, "", color='white',
                    transform=ax.transAxes, ha="center", fontsize=12)
    scatter = ax.scatter([], [], s=0.2, color='cyan', alpha=0.5)

    def update(frame):
        file = files[frame]
        try:
            # 2. Extrai o número do passo diretamente do nome do arquivo
            filename = os.path.basename(file)
            step_str = filename.replace("snapshot_", "").replace(".json", "")
            step = int(step_str)

            # 3. Lê o array plano de partículas exportado pelo Rust
            with open(file, 'r') as f:
                particles = json.load(f)

            pos = np.array([p['position'] for p in particles])
            scatter.set_offsets(pos[:, :2])
            title.set_text(f"Minor Merger - Passo: {step}")
        except Exception as e:
            print(f"Erro ao processar o frame {frame}: {e}")

        if frame % 10 == 0:
            print(f"Renderizando frame {frame}/{len(files)}...")

        return scatter, title

    ani = animation.FuncAnimation(
        fig, update, frames=len(files), interval=50, blit=False)

    out_path = os.path.join(data_dir, "colisao_galaxias.gif")

    try:
        print("Montando o arquivo GIF... (Aguarde, isso leva alguns minutos)")
        ani.save(out_path, writer='pillow', fps=20)
        print(f"\nSucesso! Simulação visual salva em: {out_path}")
    except Exception as e:
        print(f"\n[ERRO FATAL] Falha ao tentar salvar o GIF. Detalhes: {e}")


if __name__ == "__main__":
    generate_animation()
