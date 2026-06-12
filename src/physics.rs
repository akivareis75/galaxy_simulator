// Núcleo numérico: Leapfrog e Softening de Plummer
use nalgebra::Vector3;
use serde::{Serialize, Deserialize};

// Constante Gravitacional G em unidades de simulação (normalizada para 1.0)
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
    pub epsilon: f64, // Softening de Plummer
    pub dt: f64,      // Passo de tempo (Delta t)
}

impl SimulationSpace {
    pub fn new(particles: Vec<Particle>, epsilon: f64, dt: f64) -> Self {
        let mut space = SimulationSpace { particles, epsilon, dt };
        // Calcula as acelerações iniciais (a^0) para preparar o Leapfrog
        space.compute_accelerations();
        space
    }

    // Calcula a aceleração gravitacional de todas as partículas: O(N²)
    pub fn compute_accelerations(&mut self) {
        let n = self.particles.len();
        
        // Inicializa/reseta as acelerações com zero
        for p in &mut self.particles {
            p.acceleration = Vector3::zeros();
        }

        // Interação de pares N-Corpos com softening de Plummer
        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }

                // Vetor distância r_ij = r_i - r_j
                let r_vec = self.particles[i].position - self.particles[j].position;
                let r_squared = r_vec.norm_squared();
                
                // Equação (15) do planejamento: (r^2 + eps^2)^(3/2)
                let denominator = (r_squared + self.epsilon.powi(2)).powf(1.5);
                
                // Contribuição da aceleração: a_i += -G * m_j * r_vec / denominador
                if denominator > 0.0 {
                    let accel_contribution = -G * self.particles[j].mass * r_vec / denominator;
                    self.particles[i].acceleration += accel_contribution;
                }
            }
        }
    }

    // Integrador Simplético Leapfrog: Variante Kick-Drift-Kick (Equações 12, 13 e 14)
    pub fn step(&mut self) {
        // 1. Meio-Kick: v^{n+1/2} = v^n + a^n * (dt / 2)
        for p in &mut self.particles {
            p.velocity += p.acceleration * (self.dt / 2.0);
        }

        // 2. Drift: r^{n+1} = r^n + v^{n+1/2} * dt
        for p in &mut self.particles {
            p.position += p.velocity * self.dt;
        }

        // Atualiza as acelerações para as novas posições r^{n+1} -> a^{n+1}
        self.compute_accelerations();

        // 3. Meio-Kick Final: v^{n+1} = v^{n+1/2} + a^{n+1} * (dt / 2)
        for p in &mut self.particles {
            p.velocity += p.acceleration * (self.dt / 2.0);
        }
    }
}