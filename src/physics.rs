use nalgebra::Vector3;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};

pub const G: f64 = 1.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    pub id: usize,
    pub mass: f64,
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub acceleration: Vector3<f64>,
}

pub struct SimulationSpace {
    pub particles: Vec<Particle>,
    pub epsilon: f64,
    pub dt: f64,
}

impl SimulationSpace {
    pub fn new(particles: Vec<Particle>, epsilon: f64, dt: f64) -> Self {
        let mut space = SimulationSpace { particles, epsilon, dt };
        space.compute_accelerations();
        space
    }

    pub fn compute_accelerations(&mut self) {
        let epsilon_sq = self.epsilon.powi(2);
        
        // Cria um clone imutável das partículas para leitura simultânea pelas threads
        let particles_snapshot = self.particles.clone();

        // Iterador paralelo (Rayon): Distribui as partículas pelos núcleos do CPU
        self.particles.par_iter_mut().for_each(|p_i| {
            let mut accel = Vector3::zeros();
            
            for p_j in &particles_snapshot {
                if p_i.id == p_j.id { continue; }

                let r_vec = p_i.position - p_j.position;
                let r_squared = r_vec.norm_squared();
                // Código antigo e LENTO:
                // let denominator = (r_squared + epsilon_sq).powf(1.5);

                // Código novo e EXTREMAMENTE RÁPIDO:
                let r_eps_sq = r_squared + epsilon_sq;
                let denominator = r_eps_sq * r_eps_sq.sqrt();

                if denominator > 0.0 {
                    accel += -G * p_j.mass * r_vec / denominator;
                }
            }
            p_i.acceleration = accel;
        });
    }

    pub fn step(&mut self) {
        // Kick 1 (Paralelo)
        self.particles.par_iter_mut().for_each(|p| {
            p.velocity += p.acceleration * (self.dt / 2.0);
        });

        // Drift (Paralelo)
        self.particles.par_iter_mut().for_each(|p| {
            p.position += p.velocity * self.dt;
        });

        self.compute_accelerations();

        // Kick 2 (Paralelo)
        self.particles.par_iter_mut().for_each(|p| {
            p.velocity += p.acceleration * (self.dt / 2.0);
        });
    }
}