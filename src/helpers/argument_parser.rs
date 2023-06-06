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

    /// Number of delays before termination
    #[arg(short, long)]
    pub num_delays_required: usize,
}

impl Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            concat!(
                "     Inter-Arrival Mean : {}\n",
                "     Service Mean       : {}\n",
                "     Number of Delays   : {}"
            ),
            self.mean_interarrival, self.mean_service, self.num_delays_required
        );
    }
}
