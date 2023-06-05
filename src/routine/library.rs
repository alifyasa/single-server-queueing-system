use rand_distr::Distribution;

use crate::state::simulation_state::SimulationState;

use super::SimulationRoutine;

impl SimulationRoutine {
    fn exp(pdf: &rand_distr::Exp<f64>) -> f64 {
        return pdf.sample(&mut rand::thread_rng());
    }

    pub fn get_next_arrival(sim_state: &SimulationState) -> f64 {
        return Self::exp(&sim_state.pdf_interarrival);
    }

    pub fn get_next_departure(sim_state: &SimulationState) -> f64 {
        return Self::exp(&sim_state.pdf_service);
    }
}
