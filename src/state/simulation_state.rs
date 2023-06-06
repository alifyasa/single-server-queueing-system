use std::fmt::Display;

use crate::helpers::argument_parser::Args;

use super::system_state::SystemState;

pub struct SimulationState {
    pub system_state: SystemState,
    pub pdf_interarrival: rand_distr::Exp<f64>,
    pub pdf_service: rand_distr::Exp<f64>,
    /// Number of delays before termination
    pub num_delays_required: usize,
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
            num_delays_required: args.num_delays_required,
        };
    }

    pub fn get_number_delayed(&self) -> usize {
        return self.system_state.statistical_counter.number_delayed;
    }
}

impl Display for SimulationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.system_state);
    }
}
