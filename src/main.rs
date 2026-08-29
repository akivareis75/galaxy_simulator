use std::fs;

// Importa todas as estruturas do physics, incluindo o SimulationConfig agora unificado
use crate::physics::{Particle, SimulationSpace, SimulationConfig};

pub mod io;
pub mod physics;

fn main() {
    let config_data = fs::read_to_string("config.json").expect("Falha ao ler config.json");
    let config: SimulationConfig = serde_json::from_str(&config_data).expect("Erro no parse JSON");

    println!("Iniciando simulação com N-corpos adaptativo.");
    println!("Lendo entrada: {}", config.input_file);

    let input_data = fs::read_to_string(&config.input_file).expect("Falha ao ler IC");
    let particles: Vec<Particle> = serde_json::from_str(&input_data).expect("Falha no parse das partículas");

    let mut space = SimulationSpace::new(particles, config.dt_max);

    for step in 0..config.total_steps {
        space.step_adaptive(&config);
        
        if step % 100 == 0 {
            println!("Passo {} concluído | dt adaptativo: {:.6}", step, space.dt);
            io::save_snapshot(&space.particles, &config.output_dir, step);
        }
    }
    
    println!("Simulação unificada finalizada com sucesso!");
}