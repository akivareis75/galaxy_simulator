import json
import os
import glob
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation


def generate_animation():
    data_dir = "/Users/akivareis/tmp/galaxy_simulator/data/output"
    # Adicionando validação para garantir que os arquivos sejam lidos na ordem correta
    files = sorted(glob.glob(os.path.join(data_dir, "snapshot_*.json")))

    if not files:
        print("[ERRO] Nenhum snapshot encontrado em data/output/.")
        return

    print(f"[{len(files)} snapshots encontrados] Preparando a renderização visual...")

    fig, ax = plt.subplots(figsize=(8, 8), facecolor='black')
    ax.set_facecolor('black')

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
            with open(file, 'r') as f:
                data = json.load(f)

            pos = np.array([p['position'] for p in data['particles']])
            scatter.set_offsets(pos[:, :2])
            title.set_text(f"Minor Merger - Tempo: {data['time']:.3f} Gyr")
        except Exception as e:
            print(f"Erro ao processar o frame {frame}: {e}")

        if frame % 10 == 0:
            print(f"Renderizando frame {frame}/{len(files)}...")

        return scatter, title

    # blit=False força o Mac a redesenhar a tela inteira, evitando erros de tela preta/branca
    ani = animation.FuncAnimation(
        fig, update, frames=len(files), interval=50, blit=False)

    out_path = os.path.join(data_dir, "colisao_galaxias.gif")

    try:
        print("Montando o arquivo GIF... (Aguarde, isso leva alguns minutos)")
        ani.save(out_path, writer='pillow', fps=20)
        print(f"\nSucesso! Simulação visual salva em: {out_path}")
    except Exception as e:
        print(
            f"\n[ERRO FATAL] Falha ao tentar salvar o GIF. Detalhes do erro: {e}")


if __name__ == "__main__":
    generate_animation()
