#[derive(Debug)]
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

    let mut combinations = vec![];
    let mut current = vec![0; k];

    backtrack(0, n, k, 0, &mut current, &mut combinations);

    Ok(combinations)
}

pub fn backtrack (
    start: usize,
    n: usize,
    k: usize,
    index: usize,
    current: &mut Vec<usize>,
    combinations: &mut Vec<Vec<usize>>,
) {

    if index == k {
        combinations.push(current.clone());
        return;
    }

    for num in start..=(n - k + index) {
        current[index] = num;
        backtrack(num + 1, n, k, index + 1, current, combinations);
    }
}