#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
/// Types of an Event in the simulation.
/// DEPARTURE < END < ARRIVAL. This fact is used in timing
pub enum EventType {
    /// Server finished serving a costumer
    DEPARTURE,
    /// Service Ends
    END,
    /// Arrival of a new customer
    ARRIVAL,
}
