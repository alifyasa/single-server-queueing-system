use crate::state::{
    event_type::EventType, server_status::ServerStatus, simulation_state::SimulationState,
};

use super::SimulationRoutine;

impl SimulationRoutine {
    pub fn event(sim_state: &mut SimulationState, event_type: EventType) {
        let time_since_last_event: f64 =
            sim_state.system_state.simulation_clock - sim_state.system_state.time_of_last_event;
        log::debug!("Time since last event: {}", time_since_last_event);

        // Update time of last event
        sim_state.system_state.time_of_last_event = sim_state.system_state.simulation_clock;

        // Update statistics
        sim_state.system_state.statistical_counter.area_under_q_t +=
            time_since_last_event * sim_state.system_state.arrival_time_queue.len() as f64;
        sim_state.system_state.statistical_counter.area_under_b_t +=
            time_since_last_event * sim_state.system_state.get_server_status() as f64;
        log::debug!(
            "Server status: {}",
            sim_state.system_state.get_server_status()
        );

        // Handle Event
        match event_type {
            EventType::ARRIVAL => Self::arrival_routine(sim_state),
            EventType::DEPARTURE => Self::departure_routine(sim_state),
        }
    }

    fn arrival_routine(sim_state: &mut SimulationState) {
        log::debug!("Handling Arrival");
        sim_state.system_state.event_list.insert(
            EventType::ARRIVAL,
            sim_state.system_state.simulation_clock + Self::get_next_arrival(sim_state),
        );
        match sim_state.system_state.server_status {
            ServerStatus::BUSY => {
                sim_state
                    .system_state
                    .arrival_time_queue
                    .push_back(sim_state.system_state.simulation_clock);
            }
            ServerStatus::IDLE => {
                let delay: f64 = 0.0;
                sim_state.system_state.statistical_counter.total_delay += delay;
                sim_state.system_state.statistical_counter.number_delayed += 1;
                sim_state.system_state.server_status = ServerStatus::BUSY;
                sim_state.system_state.event_list.insert(
                    EventType::DEPARTURE,
                    sim_state.system_state.simulation_clock + Self::get_next_departure(sim_state),
                );
            }
        }
    }
    fn departure_routine(sim_state: &mut SimulationState) {
        log::debug!("Handling Departure");
        match sim_state.system_state.arrival_time_queue.pop_front() {
            None => {
                sim_state.system_state.server_status = ServerStatus::IDLE;
                sim_state
                    .system_state
                    .event_list
                    .insert(EventType::DEPARTURE, f64::MAX);
            }
            Some(new_customer_time) => {
                let delay = sim_state.system_state.simulation_clock - new_customer_time;
                sim_state.system_state.statistical_counter.total_delay += delay;
                sim_state.system_state.statistical_counter.number_delayed += 1;
                sim_state.system_state.event_list.insert(
                    EventType::DEPARTURE,
                    sim_state.system_state.simulation_clock + Self::get_next_departure(sim_state),
                );
            }
        }
    }
}
