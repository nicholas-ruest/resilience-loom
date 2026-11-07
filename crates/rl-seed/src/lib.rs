use rl_agent::synthetic_registry;
use rl_types::Agent;
pub fn deterministic_agents() -> Vec<Agent> {
    synthetic_registry()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable() {
        assert_eq!(deterministic_agents()[0].id, "a-research")
    }
}
