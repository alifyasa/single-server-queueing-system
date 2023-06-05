use crate::state::simulation_state::SimulationState;

use super::SimulationRoutine;

impl SimulationRoutine {
    pub fn report_gen(sim_state: &SimulationState) {
        log::info!(
            "Simulation Report: \n \n{}\n",
            sim_state.system_state.get_report()
        );
    }
}
