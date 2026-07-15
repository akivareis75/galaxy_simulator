use std::path::Path;
use std::env;
use galaxy_simulator::physics::SimulationSpace;
use galaxy_simulator::io::{load_initial_conditions, save_snapshot};
use galaxy_simulator::analytics::evaluate_conservation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Lê o primeiro argumento passado na linha de comando
    let args: Vec<String> = env::args().collect();
    let layer = if args.len() > 1 { args[1].as_str() } else { "layer2" };

    let output_dir = Path::new("/Users/akivareis/tmp/galaxy_simulator/data/output");

    match layer {
        "layer2" => {
            println!("=======================================================");
            println!(" Iniciando Camada 2: Galáxia Isolada (10k/20k Partículas)");
            println!("=======================================================");
            let input_path = Path::new("/Users/akivareis/tmp/galaxy_simulator/data/input/hernquist_20k_ic.json");
            run_simulation(input_path, output_dir, 0.005, 2000, 20)?;
        }
        "layer3" => {
            println!("=======================================================");
            println!(" Iniciando Camada 3: Colisão de Galáxias (Minor Merger)");
            println!("=======================================================");
            let input_path = Path::new("/Users/akivareis/tmp/galaxy_simulator/data/input/merger_20k_ic.json");
            // A colisão exige mais passos (ex: 4000) para ver o resultado do impacto a longo prazo
            run_simulation(input_path, output_dir, 0.005, 4000, 40)?;
        }
        _ => {
            eprintln!("[ERRO] Camada desconhecida. Use 'cargo run -- layer2' ou 'cargo run -- layer3'");
            std::process::exit(1);
        }
    }

    Ok(())
}

// Método isolado que encapsula a execução do simulador independente da camada
fn run_simulation(input_path: &Path, output_dir: &Path, dt: f64, total_steps: usize, save_interval: usize) -> Result<(), Box<dyn std::error::Error>> {
    if !input_path.exists() {
        eprintln!("[ERRO] Arquivo IC não encontrado: {:?}", input_path);
        std::process::exit(1);
    }

    println!("[IO] Carregando modelo: {:?}", input_path);
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

    println!("\n[FIM] Integração concluída.");
    Ok(())
}