use crate::{
    helpers::argument_parser::Args,
    state::{event_type::EventType, simulation_state::SimulationState},
};

use super::SimulationRoutine;

impl SimulationRoutine {
    pub fn init(args: Args) -> SimulationState {
        log::info!("Initializing Simulation");
        let mut sim_state = SimulationState::new(args);
        sim_state
            .system_state
            .event_list
            .insert(EventType::DEPARTURE, f64::INFINITY);
        sim_state
            .system_state
            .event_list
            .insert(EventType::ARRIVAL, Self::get_next_arrival(&sim_state));
        match sim_state.max_simulation_time {
            Some(time) => {
                sim_state
                    .system_state
                    .event_list
                    .insert(EventType::END, time);
            }
            None => {}
        }
        return sim_state;
    }
}
