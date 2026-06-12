"""
Script to read Rust outputs and plot energy and momentum diagnostics.
"""
import json
import glob
import os
import matplotlib.pyplot as plt
import numpy as np


def plot_simulation_diagnostics():
    output_dir = "/Users/akivareis/tmp/galaxy_simulator/data/output"
    # Search for all snapshot files saved by Rust
    pattern = os.path.join(output_dir, "snapshot_*.json")
    snapshot_files = sorted(glob.glob(pattern))

    if not snapshot_files:
        print(f"No snapshot files found in {output_dir}. Run the Rust simulation first.")
        return

    times = []
    kinetic_energy = []
    potential_energy = []
    total_energy = []
    total_momentum_mag = []
    com_positions = []

    print("Parsing and processing snapshot files...")
    # Parse and extract metrics
    for file_path in snapshot_files:
        with open(file_path, "r") as f:
            data = json.load(f)

            particles = data["particles"]
            if not particles:
                continue

            # CORREÇÃO: Removida a duplicata externa. O tempo só é adicionado aqui,
            # garantindo sincronia exata (1:1) com os calculos de energia abaixo.
            times.append(data["time"])
            
            # Extract properties into numpy arrays for vectorized operations
            masses = np.array([p["mass"] for p in particles])
            positions = np.array([p["position"] for p in particles])
            velocities = np.array([p["velocity"] for p in particles])

            # Kinetic energy: K = 1/2 * m * v^2
            k = 0.5 * np.sum(masses * np.sum(velocities**2, axis=1))

            # Potential energy: U = - G * m_i * m_j / r_ij
            # Using broadcasting to compute all pairwise distances
            diff = positions[:, np.newaxis, :] - positions[np.newaxis, :, :]
            dist_sq = np.sum(diff**2, axis=-1)

            # Add softening parameter to distance
            dist = np.sqrt(dist_sq + 0.01**2)

            # Mass products m_i * m_j
            mass_prod = masses[:, np.newaxis] * masses[np.newaxis, :]
            # Exclude self-interaction
            np.fill_diagonal(mass_prod, 0)

            # Sum all potential pairs (divided by 2 since each pair is counted twice)
            u = -0.5 * np.sum(mass_prod / dist)

            kinetic_energy.append(k)
            potential_energy.append(u)
            total_energy.append(k + u)

            # Center of mass (CoM)
            com = np.sum(masses[:, np.newaxis] * positions,
                         axis=0) / np.sum(masses)
            com_positions.append(com)

            # Total momentum
            momentum = np.sum(masses[:, np.newaxis] * velocities, axis=0)
            total_momentum_mag.append(np.linalg.norm(momentum))

    if not times:
        print("No valid particle data found in the snapshots.")
        return

    com_positions = np.array(com_positions)

    # Generation of Conservation Plots
    plt.figure(figsize=(15, 10))

    # Subplot 1: Individual Energies
    plt.subplot(2, 2, 1)
    plt.plot(times, kinetic_energy, label="Kinetic (K)", color="orange")
    plt.plot(times, potential_energy, label="Potential (U)", color="blue")
    plt.xlabel("Simulation Time")
    plt.ylabel("Energy")
    plt.title("Evolution of Energy Components")
    plt.legend()
    plt.grid(True)

    # Subplot 2: Total Energy Conservation (Symplectic sanity check)
    plt.subplot(2, 2, 2)
    e_0 = total_energy[0]
    if e_0 != 0:
        relative_error = [(e - e_0) / abs(e_0) for e in total_energy]
    else:
        relative_error = [0 for e in total_energy]

    plt.plot(times, relative_error, color="red", label="Relative Error")
    plt.xlabel("Simulation Time")
    plt.ylabel(r"Relative Energy Error ($\Delta E / |E_0|$)")
    plt.title("Leapfrog Integrator Fluctuation")
    plt.legend()
    plt.grid(True)

    # Subplot 3: Total Momentum
    plt.subplot(2, 2, 3)
    plt.plot(times, total_momentum_mag, color="purple")
    plt.xlabel("Simulation Time")
    plt.ylabel("Magnitude of Total Momentum")
    plt.title("Total Momentum Conservation")
    plt.grid(True)

    # Subplot 4: Center of Mass Drift
    plt.subplot(2, 2, 4)
    # Calculate drift distance from initial CoM
    if len(com_positions) > 0:
        com_drift = np.linalg.norm(com_positions - com_positions[0], axis=1)
        plt.plot(times, com_drift, color="green")
    plt.xlabel("Simulation Time")
    plt.ylabel("CoM Drift Distance")
    plt.title("Center of Mass Drift")
    plt.grid(True)

    plt.tight_layout()
    plot_path = os.path.join(output_dir, "conservation_diagnostics.png")
    plt.savefig(plot_path, dpi=150)
    print(f"Diagnostic plot successfully saved to: {plot_path}")


if __name__ == "__main__":
    plot_simulation_diagnostics()