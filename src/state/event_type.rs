#[derive(Eq, Hash, PartialEq, Clone, Copy)]
/// Types of an Event in the simulation
pub enum EventType {
    /// Arrival of a new customer
    ARRIVAL,
    /// Server finished serving a costumer
    DEPARTURE,
}
