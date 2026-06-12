# Script para gerar condições iniciais via galpy/agama
import json
import os

def generate_kepler_test():
    # Parâmetros para uma órbita estável ideal de dois corpos (m1 >> m2)
    particles = [
        {
            "id": 1,
            "mass": 1.0,
            "position": [0.0, 0.0, 0.0],
            "velocity": [0.0, -0.01, 0.0],
            "acceleration": [0.0, 0.0, 0.0]
        },
        {
            "id": 2,
            "mass": 0.001,
            "position": [1.0, 0.0, 0.0],
            "velocity": [0.0, 1.0, 0.0], # Velocidade orbital circular se G=1 e M=1
            "acceleration": [0.0, 0.0, 0.0]
        }
    ]
    
    output_dir = "/Users/akivareis/tmp/galaxy_simulator/data/input"
    os.makedirs(output_dir, exist_ok=True)
    
    output_path = os.path.join(output_dir, "kepler_ic.json")
    with open(output_path, "w") as f:
        json.dump(particles, f, indent=4)
        
    print(f"Condições iniciais de teste salvas com sucesso em: {output_path}")

if __name__ == "__main__":
    generate_kepler_test()