use crate::solver::Solver;

pub mod solver;
pub mod trie;

fn main() -> anyhow::Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let solver = Solver::new("dictionary-sowpods.txt".to_string())?;
    let elapsed = now.elapsed();
    println!("{} words in {:?}", solver.dictionary.size(), elapsed);
    let now = Instant::now();
    let words = solver.find("]QUIRINGXPE".to_string());
    let elapsed = now.elapsed();
    println!("{} words found in {:?}", words.len(), elapsed);
    Ok(())
}
