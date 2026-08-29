import json
import os
import numpy as np
import argparse
# Requer instalação: pip install galpy astropy
from galpy.potential import HernquistPotential
from galpy.df import isotropicHernquistdf


def generate_hernquist_equilibrium(n_particles, output_path, seed):
    print(
        f"Gerando modelo de Hernquist com {n_particles} partículas "
        f"(Seed: {seed})..."
    )

    # Trava a semente do NumPy para garantir reprodutibilidade na amostragem do galpy
    np.random.seed(seed)

    # Parâmetros base da galáxia (G=1 em unidades naturais)
    scale_radius = 1.0
    total_mass = 1.0

    # Instancia o potencial no galpy
    # Nota: No galpy, a amplitude de um Hernquist de massa M é 2*M
    hp = HernquistPotential(amp=2.0*total_mass, a=scale_radius)

    # Cria a Função de Distribuição Isotrópica atrelada ao potencial de Hernquist
    df = isotropicHernquistdf(pot=hp)

    print("Calculando o espaço de fase (isso pode levar alguns segundos)...")
    # O galpy amostra o DF e retorna um objeto "Orbit" contendo todas as partículas
    orbits = df.sample(n=n_particles)

    # Extrai coordenadas de fase cartesianas diretamente do objeto Orbit
    x, y, z = orbits.x(), orbits.y(), orbits.z()
    vx, vy, vz = orbits.vx(), orbits.vy(), orbits.vz()

    # Divide a massa total igualmente entre as super-partículas
    particle_mass = total_mass / n_particles

    particles = []
    for i in range(n_particles):
        particles.append({
            "id": i + 1,
            "mass": particle_mass,
            "position": [float(x[i]), float(y[i]), float(z[i])],
            "velocity": [float(vx[i]), float(vy[i]), float(vz[i])],
            "acceleration": [0.0, 0.0, 0.0]
        })

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w") as f:
        json.dump(particles, f, indent=4)

    print(f"Sucesso! Condições iniciais salvas em: {output_path}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Geração de IC (Hernquist) para N-Corpos")
    parser.add_argument("--n", type=int, default=20000,
                        help="Número de partículas na galáxia")
    parser.add_argument("--seed", type=int, default=42,
                        help="Semente do gerador aleatório para "
                        "reprodutibilidade")
    parser.add_argument("--out", type=str, default="/Users/akivareis/tmp"
                        "/galaxy_simulator/data/input/hernquist_ic.json",
                        help="Caminho do arquivo de saída")
    args = parser.parse_args()

    generate_hernquist_equilibrium(args.n, args.out, args.seed)
