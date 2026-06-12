use crate::physics::{Particle, G};
use nalgebra::Vector3;

pub struct DiagnosticMetrics {
    pub kinetic_energy: f64,
    pub potential_energy: f64,
    pub total_energy: f64,
    pub angular_momentum: Vector3<f64>,
}

// Calcula e retorna o estado de conservação atual do sistema N-Corpos
pub fn evaluate_conservation(particles: &[Particle], epsilon: f64) -> DiagnosticMetrics {
    let mut kinetic = 0.0;
    let mut potential = 0.0;
    let mut total_l = Vector3::zeros();
    let n = particles.len();

    for i in 0..n {
        let p_i = &particles[i];
        
        // 1. Energia Cinética: K = 0.5 * m * v^2
        kinetic += 0.5 * p_i.mass * p_i.velocity.norm_squared();

        // 2. Momento Angular: L = r x p = m * (r x v)
        let angular_momentum_i = p_i.mass * p_i.position.cross(&p_i.velocity);
        total_l += angular_momentum_i;

        // 3. Energia Potencial Gravitacional (Interação de Pares Modificada)
        for j in (i + 1)..n {
            let p_j = &particles[j];
            let r = (p_i.position - p_j.position).norm();
            
            // U = -G * m_i * m_j / sqrt(r^2 + eps^2)
            let u_ij = -G * p_i.mass * p_j.mass / (r.powi(2) + epsilon.powi(2)).sqrt();
            potential += u_ij;
        }
    }

    DiagnosticMetrics {
        kinetic_energy: kinetic,
        potential_energy: potential,
        total_energy: kinetic + potential,
        angular_momentum: total_l,
    }
}