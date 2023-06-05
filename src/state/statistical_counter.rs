pub struct StatisticalCounter {
    /// Number of delayed that has occured, increased by one whenever a server accept a new
    /// customer
    pub number_delayed: usize,
    /// Sum of delay of all customers
    pub total_delay: f64,
    /// The area under the curve of q(t) (queue length over time).
    pub area_under_q_t: f64,
    /// The area under the curve of b(t) (service availability over time).
    pub area_under_b_t: f64,
}

impl StatisticalCounter {
    pub fn new() -> StatisticalCounter {
        return StatisticalCounter {
            number_delayed: 0,
            total_delay: 0.0,
            area_under_q_t: 0.0,
            area_under_b_t: 0.0,
        };
    }
}
