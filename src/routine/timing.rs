use std::cmp::Ordering;

use crate::state::{event_type::EventType, simulation_state::SimulationState};

use super::SimulationRoutine;

impl SimulationRoutine {
    pub fn timings(sim_state: &mut SimulationState) -> EventType {
        let _res = sim_state
            .system_state
            .event_list
            .iter()
            .min_by(|(k1, v1), (k2, v2)| {
                let (k_order, v_order) = (k1.partial_cmp(k2), v1.partial_cmp(v2));
                match (k_order, v_order) {
                    // Key is impossible to be equal
                    (Some(Ordering::Equal), _) => unreachable!(),

                    // If the time is equal, use the key order
                    (Some(key_ord), Some(Ordering::Equal)) => key_ord,

                    // Other than that, use the time order
                    (Some(_), Some(time_ord)) => time_ord,

                    // Impossible for other
                    (_, _) => unreachable!(),
                }
            });

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
