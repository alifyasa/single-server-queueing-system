use std::fmt::Display;

use crate::helpers::argument_parser::Args;

use super::system_state::SystemState;

pub struct SimulationState {
    pub system_state: SystemState,
    pub pdf_interarrival: rand_distr::Exp<f64>,
    pub pdf_service: rand_distr::Exp<f64>,
    /// Number of delays before termination
    pub num_delays_required: Option<usize>,
    pub max_simulation_time: Option<f64>,
    pub max_sim_time_event_reached: bool,
}

impl SimulationState {
    pub fn new(args: Args) -> SimulationState {
        return SimulationState {
            system_state: SystemState::new(),
            pdf_interarrival: match rand_distr::Exp::new(1.0 / args.mean_interarrival) {
                Ok(pdf) => pdf,
                Err(..) => {
                    log::error!("Failed generating inter-arrival PDF");
                    std::process::exit(-1);
                }
            },
            pdf_service: match rand_distr::Exp::new(1.0 / args.mean_service) {
                Ok(pdf) => pdf,
                Err(..) => {
                    log::error!("Failed generating service PDF");
                    std::process::exit(-1);
                }
            },
            num_delays_required: args.term_args.num_delays_required,
            max_simulation_time: args.term_args.max_sim_time,
            max_sim_time_event_reached: false,
        };
    }

    fn get_number_delayed(&self) -> usize {
        return self.system_state.statistical_counter.number_delayed;
    }

    pub fn is_not_termination(&self) -> bool {
        let (num_delay, max_time) = (self.num_delays_required, self.max_simulation_time);
        match (num_delay, max_time) {
            (Some(num_delay), _) => self.get_number_delayed() < num_delay,
            (_, Some(..)) => !self.max_sim_time_event_reached,
            (_, _) => unreachable!(),
        }
    }
}

impl Display for SimulationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.system_state);
    }
}
