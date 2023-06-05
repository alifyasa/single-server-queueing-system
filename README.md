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
