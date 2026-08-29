use crate::physics::Particle;
use std::fs;
use std::path::Path;

pub fn save_snapshot(particles: &[Particle], output_dir: &str, step: usize) {
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).expect("Falha ao criar diretório de snapshots");
    }

    let file_name = format!("{}/snapshot_{:06}.json", output_dir, step);
    
    let json_data = serde_json::to_string_pretty(particles)
        .expect("Falha ao serializar as partículas para JSON");

    fs::write(&file_name, json_data)
        .unwrap_or_else(|_| panic!("Falha ao escrever o arquivo: {}", file_name));
}