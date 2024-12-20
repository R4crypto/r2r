use r2r::algorithm::backtracking::all_combination_of_size_k::generate_all_combinations;

fn main() {
    match generate_all_combinations(5, 3) {
        Ok(combinations) => {
            println!("Combinations for n=5, k=3: {:?}", combinations);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}