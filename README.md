# Single-Server Queueing System Simulation

A simulation of a single-server queueing system, written in **Rust**.

## Assumptions
 - Customer's arrival assumed distributed exponentially.
 - Server's service time assumed distributed exponentially.

# How to Run

Run using [cargo](https://doc.rust-lang.org/cargo/):

```shell
cargo run --release [mean_interarrival] [mean_service] [num_delays_required]
```

Where:
 - `mean_interarrival` is the mean of time between customer's arrival.
 - `mean_service` is the mean of time the server can serve a customer.
 - `num_delays_required`. Total number of delays. The program terminates when the `num_delays_required`-th customer is served.

 ## Example

 ```console
foo@bar:~$ cargo run --release 1 0.5 100000
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/single-server-queueing-system 1 0.5 100000`
 INFO  - Starting Single-Server Queue Simulation with Parameters:

     Inter-Arrival Mean : 1
     Service Mean       : 0.5
     Number of Delays   : 100000

 INFO  - Initializing Simulation
 INFO  - Running Simulation ...
 INFO  - Simulation Report:

     Mean Delay              : 0.4940762448876413
     Mean Customer in Queue  : 0.49428167323087124
     Mean Server Utilization : 0.4975755885947495
     Simulation Time         : 99958.43901274203
 ```
