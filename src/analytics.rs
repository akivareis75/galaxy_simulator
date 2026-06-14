use crate::physics::{Particle, G};
use nalgebra::Vector3;
use rayon::prelude::*;

pub struct DiagnosticMetrics {
    pub kinetic_energy: f64,
    pub potential_energy: f64,
    pub total_energy: f64,
    pub angular_momentum: Vector3<f64>,
}

pub fn evaluate_conservation(particles: &[Particle], epsilon: f64) -> DiagnosticMetrics {
    let epsilon_sq = epsilon.powi(2);

    // Mapeamento e redução paralela para Energia Cinética e Momento Angular
    let (kinetic, total_l) = particles.par_iter().map(|p| {
        let k = 0.5 * p.mass * p.velocity.norm_squared();
        let l = p.mass * p.position.cross(&p.velocity);
        (k, l)
    }).reduce(|| (0.0, Vector3::zeros()), |a, b| (a.0 + b.0, a.1 + b.1));

    // Cálculo da Energia Potencial Gravitacional em paralelo
    let potential: f64 = particles.par_iter().enumerate().map(|(i, p_i)| {
        let mut u_i = 0.0;
        for p_j in particles.iter().skip(i + 1) {
            let r_sq = (p_i.position - p_j.position).norm_squared();
            u_i += -G * p_i.mass * p_j.mass / (r_sq + epsilon_sq).sqrt();
        }
        u_i
    }).sum();

    DiagnosticMetrics {
        kinetic_energy: kinetic,
        potential_energy: potential,
        total_energy: kinetic + potential,
        angular_momentum: total_l,
    }
}