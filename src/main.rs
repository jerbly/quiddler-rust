use crate::{solver::Solver, trie::KeyState};

pub mod solver;
pub mod trie;

fn main() -> anyhow::Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let solver = Solver::new("dictionary-sowpods.txt".to_string())?;
    let d_elapsed = now.elapsed();
    println!("{} words in {:?}", solver.dictionary.size(), d_elapsed);
    let key = "INQUIRING".to_string().chars().collect();
    assert_eq!(solver.dictionary.contains(&key), KeyState::STRING(46));
    Ok(())
}
