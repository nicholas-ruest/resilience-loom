use rl_errors::{MeshError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetPlan {
    pub allocations: Vec<u32>,
    pub total: u32,
    pub largest_share: u32,
}

pub fn plan(tasks: usize, total: u32) -> Result<BudgetPlan> {
    if tasks == 0 || total < tasks as u32 {
        return Err(MeshError::Invalid("insufficient budget".into()));
    }
    let base = total / tasks as u32;
    let remainder = total % tasks as u32;
    let allocations = (0..tasks)
        .map(|index| base + u32::from(index < remainder as usize))
        .collect::<Vec<_>>();
    let largest_share = allocations.iter().copied().max().unwrap_or(0);
    Ok(BudgetPlan {
        allocations,
        total,
        largest_share,
    })
}

pub fn allocate(tasks: usize, total: u32) -> Result<Vec<u32>> {
    Ok(plan(tasks, total)?.allocations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_total_and_distributes_remainder() {
        let result = plan(3, 92).unwrap();
        assert_eq!(result.allocations, vec![31, 31, 30]);
        assert_eq!(result.allocations.iter().sum::<u32>(), result.total);
        assert_eq!(result.largest_share, 31);
    }

    #[test]
    fn rejects_empty_or_underfunded_work() {
        assert!(plan(0, 10).is_err());
        assert!(plan(4, 3).is_err());
    }
}
