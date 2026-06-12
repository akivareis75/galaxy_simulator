// Testes de integração numérica (Órbita de Kepler)
use galaxy_simulator::physics::{Particle, SimulationSpace};
use galaxy_simulator::analytics::evaluate_conservation;
use nalgebra::Vector3;

#[test]
fn test_leapfrog_symplectic_conservation() {
    // Inicializa dois corpos manualmente para um teste controlado de longo termo
    let body1 = Particle {
        id: 1,
        mass: 1.0,
        position: Vector3::new(0.0, 0.0, 0.0),
        velocity: Vector3::new(0.0, -0.01, 0.0),
        acceleration: Vector3::zeros(),
    };

    let body2 = Particle {
        id: 2,
        mass: 0.001,
        position: Vector3::new(1.0, 0.0, 0.0),
        velocity: Vector3::new(0.0, 1.0, 0.0),
        acceleration: Vector3::zeros(),
    };

    let epsilon = 0.001; // Softening pequeno para teste orbital puro
    let dt = 0.01;
    let mut space = SimulationSpace::new(vec![body1, body2], epsilon, dt);

    // Avalia a energia inicial (E_0)
    let initial_metrics = evaluate_conservation(&space.particles, space.epsilon);
    let e_0 = initial_metrics.total_energy;

    // Executa 10.000 passos dinâmicos (equivalente a muitas órbitas)
    for _ in 0..10000 {
        space.step();
    }

    // Avalia a energia final (E_f) após a simulação longa
    let final_metrics = evaluate_conservation(&space.particles, space.epsilon);
    let e_f = final_metrics.total_energy;

    // Calcula o desvio percentual de energia: |(E_f - E_0) / E_0|
    let energy_drift = ((e_f - e_0) / e_0).abs();

    println!("Energia Inicial: {}, Energia Final: {}", e_0, e_f);
    println!("Drift de Energia medido: {}", energy_drift);

    // Integradores simpléticos não acumulam erro linearmente.
    // O desvio relativo deve se manter estritamente baixo (ex: < 0.5%)
    assert!(
        energy_drift < 0.005,
        "O integrador Leapfrog falhou: O drift de energia ({}) foi muito alto!",
        energy_drift
    );
}