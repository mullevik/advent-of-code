use core::{fmt, panic};
use rayon::prelude::*;
use std::{thread, usize};

pub fn first_part(input: &str) -> i64 {
    let (data, image) = parse(input);

    let enhanced_image = enhance(image, &data, 2);
    enhanced_image.data.iter().filter(|&&x| x).count() as i64
}

pub fn second_part(input: &str) -> i64 {
    let (data, image) = parse(input);

    let enhanced_image = enhance(image, &data, 50);
    enhanced_image.data.iter().filter(|&&x| x).count() as i64
}

fn enhance(image: Image, data: &[bool], n_times: i32) -> Image {
    let mut enhanced_image = image;
    for i in 0..n_times {
        let is_odd = i % 2 == 1;
        enhanced_image =
            convolve_par_iter_mut(&enhanced_image, data, is_odd && *data.first().unwrap());
    }
    enhanced_image
}

fn char_to_bool(c: &char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("invalid char"),
    }
}

struct Image {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Image {
    fn empty(width: usize, height: usize) -> Self {
        Self {
            data: vec![false; width * height],
            width,
            height,
        }
    }

    fn at(&self, x: i32, y: i32) -> Option<bool> {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            Some(self.data[(y as usize * self.width) + x as usize])
        } else {
            None
        }
    }
}

impl From<Vec<Vec<bool>>> for Image {
    fn from(value: Vec<Vec<bool>>) -> Self {
        let w = value.first().unwrap().len();
        let h = value.len();
        Self {
            data: value.into_iter().flatten().collect(),
            width: w,
            height: h,
        }
    }
}
impl fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height) {
            for x in (0..self.width) {
                write!(
                    f,
                    "{}",
                    match self.at(x as i32, y as i32) {
                        Some(true) => "#",
                        _ => ".",
                    }
                )?;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn convolve(image: &Image, data: &[bool], out_of_bounds_value: bool) -> Image {
    let mut output_image = Image::empty(image.width + 2, image.height + 2);

    for y in 0..output_image.height {
        for x in 0..output_image.width {
            output_image.data[(y * output_image.width) + x] =
                data[window_value(image, (x as i32) - 1, (y as i32) - 1, out_of_bounds_value)];
        }
    }

    output_image
}

fn convolve_par_iter_mut(image: &Image, data: &[bool], out_of_bounds_value: bool) -> Image {
    let mut output_image = Image::empty(image.width + 2, image.height + 2);

    output_image
        .data
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, output)| {
            *output = data[window_value(
                image,
                (i % output_image.height) as i32 - 1,
                (i / output_image.height) as i32 - 1,
                out_of_bounds_value,
            )];
        });

    output_image
}

fn convolve_par_chunks_mut(image: &Image, data: &[bool], out_of_bounds_value: bool) -> Image {
    let mut output_image = Image::empty(image.width + 2, image.height + 2);
    let num_cores = thread::available_parallelism().unwrap();
    let chunk_size = output_image.data.len() / num_cores;
    output_image
        .data
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(i, output)| {
            output.iter_mut().enumerate().for_each(|(j, o)| {
                *o = data[window_value(
                    image,
                    ((i * chunk_size + j) % output_image.height) as i32 - 1,
                    ((i * chunk_size + j) / output_image.height) as i32 - 1,
                    out_of_bounds_value,
                )];
            })
        });
    output_image
}

fn window_value(image: &Image, target_x: i32, target_y: i32, out_of_bounds_value: bool) -> usize {
    let mut index = 0;

    let mut i = 0;
    for y in (target_y - 1)..(target_y + 2) {
        for x in (target_x - 1)..(target_x + 2) {
            if image.at(x, y).unwrap_or(out_of_bounds_value) {
                index += 1 << (9 - (i + 1));
            }
            i += 1;
        }
    }
    index
}

fn parse(input: &str) -> (Vec<bool>, Image) {
    let first_line = input.lines().next().unwrap();

    let scanner_data = first_line.chars().map(|c| char_to_bool(&c)).collect();

    let image = input
        .lines()
        .skip(2)
        .map(|line| line.chars().map(|c| char_to_bool(&c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (scanner_data, image.into())
}

#[cfg(test)]
mod tests_day_20 {
    use crate::day_20::{convolve, first_part, second_part, window_value, Image};

    use super::parse;

    const EXAMPLE_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\n#..#.\n#....\n##..#\n..#..\n..###";

    #[test]
    fn test_parse() {
        let (data, image) = parse(EXAMPLE_INPUT);

        assert_eq!(data.len(), 512);
        assert_eq!(image.width, 5);
        assert_eq!(image.height, 5);
    }

    #[test]
    fn test_convolve() {
        let mut image = Image::empty(2, 2);
        image.data[0] = true;

        let data: Vec<bool> = (0..512).map(|i| i == 1).collect();
        assert_eq!(window_value(&image, 0, 0, false), 16);
        let convolved_image = convolve(&image, &data, false);

        assert_eq!(convolved_image.width, 4);
        assert_eq!(convolved_image.height, 4);
        assert_eq!(
            convolved_image.data,
            vec![
                true, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false
            ]
        );
    }

    #[test]
    fn test_convolve_example() {
        let (data, image) = parse(EXAMPLE_INPUT);
        assert_eq!(window_value(&image, 2, 2, false), 34);
    }
    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(EXAMPLE_INPUT), 35);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/20.in")), 5680);
    }
    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(EXAMPLE_INPUT), 3351);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/20.in")), 19766);
    }
}
