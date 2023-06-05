use crate::state::{event_type::EventType, simulation_state::SimulationState};

use super::SimulationRoutine;

impl SimulationRoutine {
    pub fn timings(sim_state: &mut SimulationState) -> EventType {
        let _res = sim_state
            .system_state
            .event_list
            .iter()
            .min_by(|(_, v1), (_, v2)| v1.total_cmp(v2));

        let (key, val) = match _res {
            Some(min) => min,
            None => {
                log::error!("Failed getting next event");
                std::process::exit(-1);
            }
        };

        sim_state.system_state.simulation_clock = val.clone();
        return key.clone();
    }
}
