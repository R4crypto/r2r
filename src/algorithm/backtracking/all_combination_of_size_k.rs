pub enum CombinationError {
    InvalidZeroRange,
    KGreaterThanN
}

pub fn generate_all_combinations(n: usize, k: usize) -> Result<Vec<Vec<usize>>, CombinationError> {
    if n == 0 && k > 0 {
        return Err(CombinationError::InvalidZeroRange)
    }

    if k > n {
        return Err(CombinationError::KGreaterThanN)
    }

    let mut comibations = vec![];
    let mut current = vec![0; k];
    Ok(comibations)
}

pub fn backtrack (
    start: usize,
    n: usize,
    k: usize,
    current: &mut Vec<usize>,
    combinations: &mut Vec<Vec<usize>>,
) {

}