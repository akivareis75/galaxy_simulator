use std::path::Path;

// CORREÇÃO: Importa os módulos da biblioteca do próprio projeto, 
// em vez de usar "pub mod physics;", etc.
use galaxy_simulator::physics::SimulationSpace;
use galaxy_simulator::io::{load_initial_conditions, save_snapshot};
use galaxy_simulator::analytics::evaluate_conservation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================================");
    println!("Inicializando Simulador Galáctico - Camada 1 & 2");
    println!("==================================================");

    let input_path = Path::new("/tmp/galaxy_simulator/data/input/kepler_ic.json");
    let output_dir = Path::new("/tmp/galaxy_simulator/data/output");

    if !input_path.exists() {
        eprintln!("[ERRO] Arquivo de condições iniciais não encontrado em: {:?}", input_path);
        eprintln!("Por favor, rode o script Python 'generate_ic.py' primeiro.");
        std::process::exit(1);
    }

    println!("[IO] Carregando condições iniciais de: {:?}", input_path);
    let loaded_particles = load_initial_conditions(input_path)?;
    println!("[IO] Sucesso! {} partículas carregadas.", loaded_particles.len());

    let epsilon = 0.01;  
    let dt = 0.001;      
    let total_steps = 1000;
    let save_interval = 10; 

    let mut space = SimulationSpace::new(loaded_particles, epsilon, dt);
    let mut current_time = 0.0;

    println!("\n[PHYSICS] Iniciando loop de evolução dinâmica (Leapfrog)...");
    
    for step in 0..=total_steps {
        if step % save_interval == 0 {
            let metrics = evaluate_conservation(&space.particles, space.epsilon);
            println!(
                "Passo {:4} | Tempo: {:.3} | Energia Total: {:.6} | L_z: {:.6}",
                step, current_time, metrics.total_energy, metrics.angular_momentum.z
            );

            let snapshot_filename = format!("snapshot_{:04}.json", step);
            let snapshot_path = output_dir.join(snapshot_filename);

            save_snapshot(&snapshot_path, step, current_time, &space.particles)?;
        }

        space.step();
        current_time += dt;
    }

    println!("\n[FIM] Simulação concluída com sucesso.");
    Ok(())
}