use std::path::Path;
use galaxy_simulator::physics::SimulationSpace;
use galaxy_simulator::io::{load_initial_conditions, save_snapshot};
use galaxy_simulator::analytics::evaluate_conservation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================================================");
    println!(" Simulador Galáctico - Camada 2 (10^4 Partículas)");
    println!("=======================================================");

    let input_path = Path::new("/Users/akivareis/tmp/galaxy_simulator/data/input/hernquist_10k_ic.json");
    let output_dir = Path::new("/Users/akivareis/tmp/galaxy_simulator/data/output");

    if !input_path.exists() {
        eprintln!("[ERRO] Arquivo IC não encontrado: {:?}", input_path);
        std::process::exit(1);
    }

    println!("[IO] Carregando modelo de Hernquist: {:?}", input_path);
    let loaded_particles = load_initial_conditions(input_path)?;
    let n_particles = loaded_particles.len();
    println!("[IO] {} partículas alocadas na memória.", n_particles);

    // CÁLCULO DINÂMICO DO SOFTENING DE PLUMMER
    let a_scale = 1.0;
    let r_half_mass = a_scale * (1.0 + 2.0_f64.sqrt());
    let n_f64 = n_particles as f64;
    
    let epsilon = (n_f64 / 1000.0).powf(-0.5) * r_half_mass;
    
    println!("[PHYSICS] Raio de meia-massa (r_1/2): {:.4}", r_half_mass);
    println!("[PHYSICS] Softening de Plummer (\u{03B5}): {:.6}", epsilon);

    let dt = 0.005; 
    let total_steps = 2000;
    let save_interval = 20;

    let mut space = SimulationSpace::new(loaded_particles, epsilon, dt);
    let mut current_time = 0.0;

    println!("\n[RAYON] Iniciando Leapfrog Paralelo usando {} threads nativas...", rayon::current_num_threads());
    
    for step in 0..=total_steps {
        if step % save_interval == 0 {
            let metrics = evaluate_conservation(&space.particles, space.epsilon);
            println!(
                "Passo {:4} | Tempo: {:.3} | E. Total: {:.6} | L_z: {:.6}",
                step, current_time, metrics.total_energy, metrics.angular_momentum.z
            );

            let snapshot_filename = format!("snapshot_{:04}.json", step);
            let snapshot_path = output_dir.join(snapshot_filename);
            save_snapshot(&snapshot_path, step, current_time, &space.particles)?;
        }

        space.step();
        current_time += dt;
    }

    println!("\n[FIM] Integração de alta densidade concluída.");
    Ok(())
}