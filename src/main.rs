use clap::Parser;
use single_server_queueing_system::{
    helpers::{argument_parser::Args, logger::Logger},
    routine::SimulationRoutine,
    state::simulation_state::SimulationState,
};

fn main() {
    Logger::init(log::LevelFilter::Info);
    let args = Args::parse();
    log::info!(
        "Starting Single-Server Queue Simulation with Parameters: \n \n{}\n",
        args
    );
    let mut sim_state: SimulationState = SimulationRoutine::init(args);
    log::info!("Running Simulation ...");
    while sim_state.get_number_delayed() < sim_state.num_delays_required {
        // Determine next event
        let next_event = SimulationRoutine::timings(&mut sim_state);
        // Handle Event
        SimulationRoutine::event(&mut sim_state, next_event);
        log::debug!("System Status: \n \n{}\n", sim_state);
    }
    SimulationRoutine::report_gen(&sim_state);
}
