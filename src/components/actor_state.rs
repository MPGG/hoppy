#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorState {
    Grounded,
    Airborne,
    Dead,
}

impl Default for ActorState {
    fn default() -> Self {
        Self::Dead
    }
}
