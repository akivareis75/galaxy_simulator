// Leitura de Condições Iniciais e escrita de Snapshots
use crate::physics::Particle;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

// Estrutura auxiliar para mapear o formato JSON de entrada e saída
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub step: usize,
    pub time: f64,
    pub particles: Vec<Particle>,
}

// Carrega as Condições Iniciais (IC) geradas externamente
pub fn load_initial_conditions<P: AsRef<Path>>(path: P) -> Result<Vec<Particle>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let particles: Vec<Particle> = serde_json::from_reader(reader)?;
    Ok(particles)
}

// Salva o snapshot atual para posterior análise CAS e Sérsic em Python
pub fn save_snapshot<P: AsRef<Path>>(
    path: P, 
    step: usize, 
    time: f64, 
    particles: &[Particle]
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    
    let snapshot = Snapshot {
        step,
        time,
        particles: particles.to_vec(),
    };

    // Serializa os dados de forma compacta (JSON de uma linha por eficiência)
    serde_json::to_writer(writer, &snapshot)?;
    Ok(())
}

// Função auxiliar para exportar dados em formato CSV simples, caso prefira ler puro em matrizes NumPy
pub fn export_to_csv<P: AsRef<Path>>(path: P, particles: &[Particle]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Header do arquivo
    writeln!(writer, "id,mass,x,y,z,vx,vy,vz")?;

    for p in particles {
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{}",
            p.id, p.mass,
            p.position.x, p.position.y, p.position.z,
            p.velocity.x, p.velocity.y, p.velocity.z
        )?;
    }
    Ok(())
}