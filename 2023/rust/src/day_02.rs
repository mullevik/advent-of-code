use std::vec;

#[derive(PartialEq, Debug)]
struct Throw {
    red: i32,
    green: i32,
    blue: i32,
}
impl Throw {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    name: i32,
    throws: Vec<Throw>,
}

fn parse_game_name(game_name: &str) -> i32 {
    game_name.split(" ").last().unwrap().parse::<i32>().unwrap()
}

fn parse_game_content(content: &str) -> Vec<Throw> {
    content.split(";").map(parse_game_throw).collect()
}

fn parse_game_throw(throw: &str) -> Throw {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for data in throw.split(",") {
        match data.trim().split_once(" ").unwrap() {
            (x, "red") => red += x.parse::<i32>().unwrap(),
            (x, "green") => green += x.parse::<i32>().unwrap(),
            (x, "blue") => blue += x.parse::<i32>().unwrap(),
            _ => panic!("Unexpected game throw"),
        }
    }
    Throw { red: red, green: green, blue: blue }
}

fn parse_line(line: &str) -> Game {
    match line.split_once(":").unwrap() {
        (game_name, content) => Game{name: parse_game_name(game_name), throws: parse_game_content(content)},
        _ => panic!("Unexpected game line"),
    }
}



fn extract_min_cubes(game: &Game) -> Throw {
    let red_max = game.throws.iter().map(|g| g.red).max().unwrap();
    let green_max = game.throws.iter().map(|g| g.green).max().unwrap();
    let blue_max = game.throws.iter().map(|g| g.blue).max().unwrap();

    Throw { red: red_max, green: green_max, blue: blue_max }
}

fn is_possible(game: &Game) -> bool {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let min_cubes = extract_min_cubes(game);
    min_cubes.red <= MAX_RED && min_cubes.green <= MAX_GREEN && min_cubes.blue <= MAX_BLUE
}



pub fn first_part(input: &str) -> i32 {
    input.split("\n").map(|line| parse_line(line)).filter(is_possible).map(|g| g.name).sum()
}

pub fn second_part(input: &str) -> i32 {
    input.split("\n").map(|line| parse_line(line)).map(|g| extract_min_cubes(&g).power()).sum()
}


#[cfg(test)]
mod tests {
    use crate::day_02::{first_part, second_part, parse_line, Game, Throw, is_possible};
    
    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/02_example_1.txt")), 8);
    }

    #[test]
    fn test_parse_line() {
        let game = parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            game, 
            Game{
                name: 1, 
                throws: vec![
                    Throw{red: 4, blue: 3, green: 0},
                    Throw{red: 1, blue: 6, green: 2},
                    Throw{red: 0, blue: 0, green: 2},
                ]
            });
        assert_eq!(is_possible(&game), true);
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/02.secret")), 2164);
        assert_eq!(second_part(include_str!("inputs/02.secret")), 69929);
    }
}