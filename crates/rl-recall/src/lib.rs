use rl_embedding::embed;
use rl_errors::Result;
use rl_ruvector::RecoveryPlanIndex;
use rl_types::Analogue;
pub fn recall(index: &RecoveryPlanIndex, text: &str) -> Result<Vec<Analogue>> {
    index.search(&embed(text)?, 3)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_index() {
        assert!(recall(&RecoveryPlanIndex::new().unwrap(), "objective")
            .unwrap()
            .is_empty());
    }
    #[test]
    fn rejects_empty_recall_query() {
        assert!(recall(&RecoveryPlanIndex::new().unwrap(), "").is_err());
    }
}
