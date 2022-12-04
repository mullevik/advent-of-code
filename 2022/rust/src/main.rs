

fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;
    
    let calories = include_str!("../inputs/01_1")
        .lines()
        .map(|line| line.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let mut elven_sums = calories.split(|cal| cal.is_none())
    .map(|g| g.iter()
        .map(|cal| cal.unwrap())
        .sum::<u32>()).collect::<Vec<_>>();
    elven_sums.sort();
    elven_sums.reverse();
    let sum_of_top_three_elfs: u32 = elven_sums.iter().take(3).sum();
    // println!("calories = {calories:?}");
    println!("most calories = {sum_of_top_three_elfs}");
    return Ok(())
}