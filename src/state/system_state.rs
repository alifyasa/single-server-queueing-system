use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use super::{
    event_type::EventType, server_status::ServerStatus, statistical_counter::StatisticalCounter,
};

pub struct SystemState {
    pub server_status: ServerStatus,
    /// Queue of customer, represented by a vector of arrival time
    pub arrival_time_queue: VecDeque<f64>,
    pub time_of_last_event: f64,
    pub simulation_clock: f64,
    pub event_list: HashMap<EventType, f64>,
    pub statistical_counter: StatisticalCounter,
}

impl SystemState {
    pub fn new() -> SystemState {
        return SystemState {
            server_status: ServerStatus::IDLE,
            arrival_time_queue: VecDeque::new(),
            time_of_last_event: 0.0,
            simulation_clock: 0.0,
            event_list: HashMap::new(),
            statistical_counter: StatisticalCounter::new(),
        };
    }

    pub fn get_server_status(&self) -> usize {
        return match self.server_status {
            ServerStatus::BUSY => 1,
            ServerStatus::IDLE => 0,
        };
    }

    pub fn get_report(&self) -> String {
        return format!(
            concat!(
                "     Mean Delay              : {}\n",
                // "     Total Area Under Q(t)   : {}\n",
                // "     Total Area Under B(t)   : {}\n",
                "     Mean Customer in Queue  : {}\n",
                "     Mean Server Utilization : {}\n",
                "     Simulation Time         : {}"
            ),
            self.statistical_counter.total_delay / self.statistical_counter.number_delayed as f64,
            // self.statistical_counter.area_under_q_t,
            // self.statistical_counter.area_under_b_t,
            self.statistical_counter.area_under_q_t / self.simulation_clock,
            self.statistical_counter.area_under_b_t / self.simulation_clock,
            self.simulation_clock
        );
    }
}

impl Display for SystemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            concat!(
                "     Server Status      : {}\n",
                "     Queue              : {:?}\n",
                "     Time of Last Event : {}\n",
                "     Simulation Clock   : {}\n",
                "     Next Arrival       : {}\n",
                "     Next Departure     : {}",
            ),
            match self.server_status {
                ServerStatus::BUSY => "BUSY",
                ServerStatus::IDLE => "IDLE",
            },
            self.arrival_time_queue,
            self.time_of_last_event,
            self.simulation_clock,
            self.event_list.get(&EventType::ARRIVAL).unwrap(),
            self.event_list.get(&EventType::DEPARTURE).unwrap(),
        );
    }
}
