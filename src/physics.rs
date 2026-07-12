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
        let n = self.particles.len();
        
        // Implementação Matemática da Terceira Lei de Newton (Fij = -Fji)
        // O padrão Map-Reduce do Rayon evita "data races" criando buffers locais por thread.
        let new_accelerations = (0..n)
            .into_par_iter()
            .fold(
                || vec![Vector3::zeros(); n], // Buffer de acelerações local para a thread
                |mut accels, i| {
                    let p_i = &self.particles[i];
                    
                    // Loop Triangular: Garantimos que cada par (i, j) seja calculado apenas uma vez.
                    for j in (i + 1)..n {
                        let p_j = &self.particles[j];

                        // Vetor distância r_i - r_j
                        let r_vec = p_i.position - p_j.position;
                        let r_squared = r_vec.norm_squared();
                        let r_eps_sq = r_squared + epsilon_sq;
                        let denominator = r_eps_sq * r_eps_sq.sqrt();

                        if denominator > 0.0 {
                            // Fator base da Força (sem as massas individuais): G * r_vec / r^3
                            let force_base = G * r_vec / denominator;

                            // Aceleração em i gerada por j (a_i = F_ij / m_i)
                            accels[i] += -force_base * p_j.mass;
                            
                            // Aceleração em j gerada por i (a_j = F_ji / m_j)
                            // Pela 3ª Lei, a força é inversa. Como r_vec foi calculado de j para i,
                            // o sinal aqui é invertido em relação à aceleração de i.
                            accels[j] += force_base * p_i.mass;
                        }
                    }
                    accels
                }
            )
            .reduce(
                || vec![Vector3::zeros(); n],
                |mut accels_a, accels_b| {
                    // Combina os resultados de todas as threads eficientemente
                    for (a, b) in accels_a.iter_mut().zip(accels_b.iter()) {
                        *a += *b;
                    }
                    accels_a
                }
            );

        // Aplica as acelerações acumuladas de volta às partículas originais
        self.particles.par_iter_mut().enumerate().for_each(|(i, p)| {
            p.acceleration = new_accelerations[i];
        });
    }

    pub fn step(&mut self) {
        // Kick 1 (Paralelo) - Integrador Leapfrog
        self.particles.par_iter_mut().for_each(|p| {
            p.velocity += p.acceleration * (self.dt / 2.0);
        });

        // Drift (Paralelo)
        self.particles.par_iter_mut().for_each(|p| {
            p.position += p.velocity * self.dt;
        });

        // Atualiza acelerações baseadas nas novas posições espaciais
        self.compute_accelerations();

        // Kick 2 (Paralelo)
        self.particles.par_iter_mut().for_each(|p| {
            p.velocity += p.acceleration * (self.dt / 2.0);
        });
    }
}