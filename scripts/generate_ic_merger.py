import json
import os
from galpy.potential import HernquistPotential
from galpy.df import isotropicHernquistdf


def generate_galaxy(n_particles, total_mass, offset_pos, offset_vel, start_id):
    scale_radius = 1.0
    hp = HernquistPotential(amp=2.0*total_mass, a=scale_radius)
    df = isotropicHernquistdf(pot=hp)

    orbits = df.sample(n=n_particles)
    x, y, z = orbits.x(), orbits.y(), orbits.z()
    vx, vy, vz = orbits.vx(), orbits.vy(), orbits.vz()

    particle_mass = total_mass / n_particles
    particles = []

    for i in range(n_particles):
        particles.append({
            "id": start_id + i,
            "mass": particle_mass,
            # Aplica o deslocamento no espaço (posição)
            "position": [float(x[i]) + offset_pos[0], float(y[i]) + offset_pos[1], float(z[i]) + offset_pos[2]],
            # Aplica a velocidade de aproximação
            "velocity": [float(vx[i]) + offset_vel[0], float(vy[i]) + offset_vel[1], float(vz[i]) + offset_vel[2]],
            "acceleration": [0.0, 0.0, 0.0]
        })
    return particles


def generate_merger(n_each=10000):
    print(
        f"Gerando colisão (Merger) com 2 galáxias de {n_each} partículas cada...")

    # Galáxia 1 (Principal): Massa 1.0, centralizada na origem e parada
    galaxy1 = generate_galaxy(
        n_particles=n_each,
        total_mass=1.0,
        offset_pos=[0.0, 0.0, 0.0],
        offset_vel=[0.0, 0.0, 0.0],
        start_id=1
    )

    # Galáxia 2 (Satélite): Massa 0.25 (Minor Merger q=1/4), deslocada em X e com velocidade em Y
    galaxy2 = generate_galaxy(
        n_particles=n_each,
        total_mass=0.25,
        # Começa a uma distância de 8 unidades em X
        offset_pos=[8.0, 2.0, 0.0],
        offset_vel=[-0.5, 0.2, 0.0],  # Indo em direção à origem (X negativo)
        start_id=n_each + 1
    )

    # Junta as duas galáxias no mesmo vetor de partículas
    all_particles = galaxy1 + galaxy2

    output_dir = "/Users/akivareis/tmp/galaxy_simulator/data/input"
    os.makedirs(output_dir, exist_ok=True)

    output_path = os.path.join(output_dir, "merger_20k_ic.json")
    with open(output_path, "w") as f:
        json.dump(all_particles, f, indent=4)

    print(f"Sucesso! Cenário de Camada 3 salvo em: {output_path}")


if __name__ == "__main__":
    generate_merger(10000)
