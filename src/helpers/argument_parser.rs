use std::{fmt::Display, str::FromStr};

#[allow(dead_code)]
pub struct Arguments {
    pub mean_interarrival: f64,
    pub mean_service: f64,
    pub num_delays_required: usize,
}

#[allow(dead_code)]
impl Arguments {
    pub fn parse() -> Arguments {
        let args: Vec<String> = std::env::args().skip(1).collect();
        match args.len() {
            3 => {
                return Arguments {
                    mean_interarrival: Self::parse_variable(&args[0]),
                    mean_service: Self::parse_variable(&args[1]),
                    num_delays_required: Self::parse_variable(&args[2]),
                }
            }
            _ => {
                log::error!("Incorrect number of argument");
                eprintln!(" Usage:");
                eprintln!("     single-server-queueing-system [mean_interarrival] [mean_service] [num_delays_required]");
                std::process::exit(-1);
            }
        }
    }

    fn parse_variable<T: FromStr>(s: &String) -> T {
        match s.parse() {
            Ok(var) => return var,
            _ => {
                log::error!(
                    "Failed parsing argument {} as {}",
                    s,
                    std::any::type_name::<T>()
                );
                std::process::exit(-1);
            }
        }
    }
}

impl Display for Arguments {
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
