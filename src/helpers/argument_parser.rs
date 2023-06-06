use clap::Parser;
use std::fmt::Display;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Mean of time between customer's arrival. Distributed exponentially
    #[arg(short = 'i', long)]
    pub mean_interarrival: f64,

    /// Mean of time the server can serve a customer. Distributed exponentially
    #[arg(short = 's', long)]
    pub mean_service: f64,

    // /// Number of delays before termination
    // #[arg(short, long)]
    // pub num_delays_required: usize,
    //
    // /// Simulation time before termination
    // #[arg(short = 't', long)]
    // pub max_sim_time:f64,
    /// Termination Condition
    #[command(flatten)]
    pub term_args: ArgsTermination,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ArgsTermination {
    /// Number of delays before termination
    #[arg(short, long)]
    pub num_delays_required: Option<usize>,

    /// Simulation time before termination
    #[arg(short = 't', long)]
    pub max_sim_time: Option<f64>,
}

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = format!(
            concat!(
                "     Inter-Arrival Mean  : {}\n",
                "     Service Mean        : {}\n",
            ),
            self.mean_interarrival, self.mean_service,
        );
        let (num_delays, max_sim_time) = (
            self.term_args.num_delays_required,
            self.term_args.max_sim_time,
        );
        let ext = match (num_delays, max_sim_time) {
            (Some(numd), None) => format!("     Number of Delays    : {}", numd),
            (None, Some(time)) => format!("     Max Simulation Time : {}", time),
            _ => unreachable!(),
        };
        return write!(f, "{}{}", base, ext);
    }
}
