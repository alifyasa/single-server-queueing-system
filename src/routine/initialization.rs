use crate::{
    helpers::argument_parser::Arguments,
    state::{event_type::EventType, simulation_state::SimulationState},
};

use super::SimulationRoutine;

impl SimulationRoutine {
    pub fn init(args: Arguments) -> SimulationState {
        log::info!("Initializing Simulation");
        let mut sim_state = SimulationState::new(args);
        sim_state
            .system_state
            .event_list
            .insert(EventType::DEPARTURE, f64::MAX);
        sim_state
            .system_state
            .event_list
            .insert(EventType::ARRIVAL, Self::get_next_arrival(&sim_state));
        return sim_state;
    }
}
