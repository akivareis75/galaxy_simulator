use nalgebra::Vector3;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};

pub const G: f64 = 1.0;

#[derive(Deserialize, Debug, Clone)]
pub struct SimulationConfig {
    pub input_file: String,
    pub output_dir: String,
    pub softening_epsilon: f64,
    pub eta: f64,
    pub dt_max: f64,
    pub theta: f64,
    pub total_steps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    pub id: usize,
    pub mass: f64,
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub acceleration: Vector3<f64>,
}

#[derive(Clone)]
pub struct BoundingBox {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl BoundingBox {
    pub fn size(&self) -> f64 {
        (self.max.x - self.min.x).max(self.max.y - self.min.y).max(self.max.z - self.min.z)
    }
}

#[derive(Clone)]
pub struct OctreeNode {
    pub center_of_mass: Vector3<f64>,
    pub total_mass: f64,
    pub bounds: BoundingBox,
    pub is_leaf: bool,
    pub particle_id: Option<usize>,
    pub children: Option<[usize; 8]>, 
}

pub struct BarnesHutTree {
    pub nodes: Vec<OctreeNode>,
}

impl BarnesHutTree {
    pub fn build(particles: &[Particle]) -> Self {
        if particles.is_empty() {
            return BarnesHutTree { nodes: vec![] };
        }

        let mut min = particles[0].position;
        let mut max = particles[0].position;
        
        for p in particles {
            min.x = min.x.min(p.position.x);
            min.y = min.y.min(p.position.y);
            min.z = min.z.min(p.position.z);
            max.x = max.x.max(p.position.x);
            max.y = max.y.max(p.position.y);
            max.z = max.z.max(p.position.z);
        }
        
        max += Vector3::new(1e-7, 1e-7, 1e-7);
        let root_bounds = BoundingBox { min, max };

        let mut nodes = Vec::with_capacity(particles.len() * 2);
        let all_indices: Vec<usize> = (0..particles.len()).collect();

        Self::build_node(&mut nodes, particles, &all_indices, root_bounds, 0);

        BarnesHutTree { nodes }
    }

    fn build_node(
        nodes: &mut Vec<OctreeNode>, 
        particles: &[Particle], 
        p_indices: &[usize], 
        bounds: BoundingBox,
        depth: usize
    ) -> Option<usize> {
        if p_indices.is_empty() {
            return None;
        }

        let node_idx = nodes.len();
        
        nodes.push(OctreeNode {
            center_of_mass: Vector3::zeros(),
            total_mass: 0.0,
            bounds: bounds.clone(),
            is_leaf: true,
            particle_id: None,
            children: None,
        });

        if p_indices.len() == 1 || depth > 50 {
            let mut total_m = 0.0;
            let mut com = Vector3::zeros();
            for &idx in p_indices {
                total_m += particles[idx].mass;
                com += particles[idx].position * particles[idx].mass;
            }
            if total_m > 0.0 { com /= total_m; }

            let node = &mut nodes[node_idx];
            node.total_mass = total_m;
            node.center_of_mass = com;
            node.is_leaf = true;
            node.particle_id = if p_indices.len() == 1 { Some(p_indices[0]) } else { None };
        } else {
            let mid = (bounds.min + bounds.max) / 2.0;
            let mut octants: [Vec<usize>; 8] = Default::default();

            for &idx in p_indices {
                let pos = particles[idx].position;
                let mut oct_idx = 0;
                if pos.x > mid.x { oct_idx |= 1; }
                if pos.y > mid.y { oct_idx |= 2; }
                if pos.z > mid.z { oct_idx |= 4; }
                octants[oct_idx].push(idx);
            }

            // CORREÇÃO AQUI: Inicializa os filhos vazios com o valor máximo possível de usize
            let mut children_indices = [usize::MAX; 8];
            let mut has_children = false;
            let mut total_m = 0.0;
            let mut com = Vector3::zeros();

            for i in 0..8 {
                if !octants[i].is_empty() {
                    let child_bounds = Self::get_octant_bounds(&bounds, i, &mid);
                    if let Some(child_idx) = Self::build_node(nodes, particles, &octants[i], child_bounds, depth + 1) {
                        children_indices[i] = child_idx;
                        has_children = true;
                        
                        let child = &nodes[child_idx];
                        total_m += child.total_mass;
                        com += child.center_of_mass * child.total_mass;
                    }
                }
            }

            let node = &mut nodes[node_idx];
            node.is_leaf = false;
            node.children = if has_children { Some(children_indices) } else { None };
            node.total_mass = total_m;
            if total_m > 0.0 {
                node.center_of_mass = com / total_m;
            }
        }

        Some(node_idx)
    }

    fn get_octant_bounds(parent: &BoundingBox, octant: usize, mid: &Vector3<f64>) -> BoundingBox {
        let min_x = if (octant & 1) == 0 { parent.min.x } else { mid.x };
        let max_x = if (octant & 1) == 0 { mid.x } else { parent.max.x };
        
        let min_y = if (octant & 2) == 0 { parent.min.y } else { mid.y };
        let max_y = if (octant & 2) == 0 { mid.y } else { parent.max.y };
        
        let min_z = if (octant & 4) == 0 { parent.min.z } else { mid.z };
        let max_z = if (octant & 4) == 0 { mid.z } else { parent.max.z };
        
        BoundingBox {
            min: Vector3::new(min_x, min_y, min_z),
            max: Vector3::new(max_x, max_y, max_z),
        }
    }

    pub fn compute_force(&self, target: &Particle, node_idx: usize, theta: f64, epsilon: f64) -> Vector3<f64> {
        if self.nodes.is_empty() { return Vector3::zeros(); }
        
        let node = &self.nodes[node_idx];
        let mut force = Vector3::zeros();
        
        let r_vec = node.center_of_mass - target.position;
        let d_sq = r_vec.norm_squared();
        let d = d_sq.sqrt();
        
        if d == 0.0 { return force; }

        let s = node.bounds.size();
        
        if s / d < theta || node.is_leaf {
            let epsilon_sq = epsilon * epsilon;
            let denominator = (d_sq + epsilon_sq) * (d_sq + epsilon_sq).sqrt();
            let force_mag = G * target.mass * node.total_mass / denominator;
            force = r_vec * force_mag;
        } else if let Some(children) = node.children {
            for &child_idx in &children {
                // CORREÇÃO AQUI: Só processa o filho se ele não for uma representação de vazio
                if child_idx != usize::MAX {
                    force += self.compute_force(target, child_idx, theta, epsilon);
                }
            }
        }
        
        force
    }
}

pub struct SimulationSpace {
    pub particles: Vec<Particle>,
    pub dt: f64,
}

impl SimulationSpace {
    pub fn new(particles: Vec<Particle>, initial_dt: f64) -> Self {
        SimulationSpace { particles, dt: initial_dt }
    }

    pub fn compute_accelerations_and_max_a(&mut self, config: &SimulationConfig) -> f64 {
        let tree = BarnesHutTree::build(&self.particles);
        let n = self.particles.len();
        
        let new_accelerations: Vec<Vector3<f64>> = self.particles.par_iter().map(|p| {
            let f = tree.compute_force(p, 0, config.theta, config.softening_epsilon);
            f / p.mass 
        }).collect();

        let mut max_a_sq = 0.0f64;
        for i in 0..n {
            self.particles[i].acceleration = new_accelerations[i];
            let a_sq = new_accelerations[i].norm_squared();
            if a_sq > max_a_sq {
                max_a_sq = a_sq;
            }
        }

        max_a_sq.sqrt()
    }

    pub fn step_adaptive(&mut self, config: &SimulationConfig) {
        let dt_half = self.dt / 2.0;

        self.particles.par_iter_mut().for_each(|p| {
            p.velocity += p.acceleration * dt_half;
        });

        self.particles.par_iter_mut().for_each(|p| {
            p.position += p.velocity * self.dt;
        });

        let max_a = self.compute_accelerations_and_max_a(config);

        self.particles.par_iter_mut().for_each(|p| {
            p.velocity += p.acceleration * dt_half;
        });

        if max_a > 0.0 {
            let cfl_dt = config.eta * (config.softening_epsilon / max_a).sqrt();
            self.dt = cfl_dt.min(config.dt_max);
        }
    }
}