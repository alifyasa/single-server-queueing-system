# Single-Server Queueing System Simulation

A simulation of a single-server queueing system, written in **Rust**.

## Assumptions
 - Customer's arrival assumed distributed exponentially.
 - Server's service time assumed distributed exponentially.

# How to Run

Run using [cargo](https://doc.rust-lang.org/cargo/):

```console
foo@bar:~$ cargo run -- --help
   Compiling single-server-queueing-system v0.2.0 (/home/alifyasa/Git/single-server-queueing-system)
    Finished dev [unoptimized + debuginfo] target(s) in 1.47s
     Running `target/debug/single-server-queueing-system --help`
Usage: single-server-queueing-system --mean-interarrival <MEAN_INTERARRIVAL> --mean-service <MEAN_SERVICE> <--num-delays-required <NUM_DELAYS_REQUIRED>|--max-sim-time <MAX_SIM_TIME>>

Options:
  -i, --mean-interarrival <MEAN_INTERARRIVAL>
          Mean of time between customer's arrival. Distributed exponentially
  -s, --mean-service <MEAN_SERVICE>
          Mean of time the server can serve a customer. Distributed exponentially
  -n, --num-delays-required <NUM_DELAYS_REQUIRED>
          Number of delays before termination
  -t, --max-sim-time <MAX_SIM_TIME>
          Simulation time before termination
  -h, --help
          Print help
  -V, --version
          Print version
```

or compile it before running:

```console
foo@bar:~$ cargo build --release
   Compiling single-server-queueing-system v0.2.0 (/home/foo/)
    Finished release [optimized] target(s) in 1.50s
foo@bar:~$ ./target/release/single-server-queueing-system --help
Usage: single-server-queueing-system --mean-interarrival <MEAN_INTERARRIVAL> --mean-service <MEAN_SERVICE> <--num-delays-required <NUM_DELAYS_REQUIRED>|--max-sim-time <MAX_SIM_TIME>>

Options:
  ...
```

 ## Example

 ```console
foo@bar:~$ cargo run --release -i 1 -s 0.5 -n 100000
 INFO  - Starting Single-Server Queue Simulation with Parameters:

     Inter-Arrival Mean  : 1
     Service Mean        : 0.5
     Number of Delays    : 100000

 INFO  - Initializing Simulation
 INFO  - Running Simulation ...
 INFO  - Simulation Report:

     Mean Delay              : 0.5042315428779208
     Mean Customer in Queue  : 0.5030363491573772
     Mean Server Utilization : 0.502340035976073
     Simulation Time         : 100239.17378594127

 ```

 ```console
foo@bar:~$ cargo run --release -i 1 -s 0.5 -t 100000
 INFO  - Starting Single-Server Queue Simulation with Parameters:

     Inter-Arrival Mean  : 1
     Service Mean        : 0.5
     Max Simulation Time : 100000

 INFO  - Initializing Simulation
 INFO  - Running Simulation ...
 INFO  - Simulation Report:

     Mean Delay              : 0.5037678899980591
     Mean Customer in Queue  : 0.504830840245955
     Mean Server Utilization : 0.5027925359629589
     Simulation Time         : 100000

 ```
