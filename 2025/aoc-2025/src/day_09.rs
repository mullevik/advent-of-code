use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::{commons::Vec2, vec2};

pub fn p1(input: &str) -> i64 {
    let points = parse(input);
    let mut mx = 0;
    for i in 0..points.len() {
        for j in 0..points.len() {
            let a = points[i];
            let b = points[j];

            let a_ = area((a, b));
            if a_ > mx {
                mx = a_
            }
        }
    }
    mx
}
pub fn p2(input: &str) -> i64 {
    let points = parse(input);

    let mut x_vertices = points
        .iter()
        .map(|p| p.x)
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    x_vertices.sort();

    let mut y_vertices = points
        .iter()
        .map(|p| p.y)
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    y_vertices.sort();

    let lines = build_lines(&points);

    let mut centroids = vec![];
    for row in 0..y_vertices.len() - 1 {
        for col in 0..x_vertices.len() - 1 {
            let a = Vec2::new(x_vertices[row], y_vertices[col]);
            let b = Vec2::new(x_vertices[row + 1], y_vertices[col + 1]);

            let c = centroid((a, b));
            centroids.push((c, is_inside(c, &lines)));
        }
    }
    // println!("centroids {:?}", centroids);

    let mut mx = 0;
    for i in 0..points.len() {
        for j in 0..points.len() {
            let bbox = (points[i], points[j]);
            let bbox_area = area(bbox);
            let overlapping_centroids = centroids
                .iter()
                .filter(|c| contains(bbox, c.0))
                .collect::<Vec<_>>();

            if overlapping_centroids.iter().all(|c| c.1) {
                println!("valid rect is: {:?} ({})", bbox, bbox_area);
                if bbox_area > mx {
                    mx = bbox_area
                }
            }
        }
    }
    mx
}

fn area(bbox: (Vec2<i32>, Vec2<i32>)) -> i64 {
    (bbox.0.x.abs_diff(bbox.1.x) as i64 + 1) * (bbox.0.y.abs_diff(bbox.1.y) as i64 + 1)
}

fn build_lines(points: &[Vec2<i32>]) -> Vec<(Vec2<i32>, Vec2<i32>)> {
    let mut lines = vec![];

    for i in 0..points.len() - 1 {
        lines.push((points[i], points[i + 1]));
    }
    lines.push((points[points.len() - 1], points[0]));
    lines
}

fn is_inside(p: Vec2<f32>, lines: &[(Vec2<i32>, Vec2<i32>)]) -> bool {
    let cnt = lines
        .iter()
        .filter(|&l| has_intersect(*l, p.x) && (l.0.y as f32) <= p.y)
        .count();
    cnt % 2 == 1
}

fn parse(input: &str) -> Vec<Vec2<i32>> {
    input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .map(|l: &str| {
            let (a, b) = l.split_once(",").unwrap();
            Vec2::new(a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<_>>()
}

fn contains(bbox: (Vec2<i32>, Vec2<i32>), p: Vec2<f32>) -> bool {
    let min_bbox_x = min(bbox.0.x, bbox.1.x);
    let max_bbox_x = max(bbox.0.x, bbox.1.x);
    let min_bbox_y = min(bbox.0.y, bbox.1.y);
    let max_bbox_y = max(bbox.0.y, bbox.1.y);

    p.x >= min_bbox_x as f32
        && p.x <= max_bbox_x as f32
        && p.y >= min_bbox_y as f32
        && p.y <= max_bbox_y as f32
}

fn centroid(bbox: (Vec2<i32>, Vec2<i32>)) -> Vec2<f32> {
    let min_bbox_x = min(bbox.0.x, bbox.1.x);
    let max_bbox_x = max(bbox.0.x, bbox.1.x);
    let min_bbox_y = min(bbox.0.y, bbox.1.y);
    let max_bbox_y = max(bbox.0.y, bbox.1.y);

    vec2!(
        min_bbox_x as f32 + ((max_bbox_x - min_bbox_x) as f32 / 2.),
        min_bbox_y as f32 + ((max_bbox_y - min_bbox_y) as f32 / 2.)
    )
}

fn has_intersect(line: (Vec2<i32>, Vec2<i32>), beam_x: f32) -> bool {
    if line.0.x == line.1.x {
        // vertical
        false
    } else {
        // horizontal
        let min_line_x = min(line.0.x, line.1.x);
        let max_line_x = max(line.0.x, line.1.x);
        beam_x >= min_line_x as f32 && beam_x <= max_line_x as f32
    }
}

mod tests {
    use std::fs;

    use crate::{
        commons::Vec2,
        day_09::{centroid, contains, has_intersect, is_inside, p1, p2},
        vec2,
    };

    #[test]
    fn test_p1() {
        assert_eq!(p1(&fs::read_to_string("inputs/09.example").unwrap()), 50);
    }
    #[test]
    fn test_p2() {
        assert_eq!(p2(&fs::read_to_string("inputs/09.example").unwrap()), 24);
        assert_eq!(p2(&fs::read_to_string("inputs/09.custom").unwrap()), 153);
    }

    #[test]
    fn test_utils() {
        assert_eq!(
            centroid((Vec2::new(1, 1), Vec2::new(4, 4))),
            vec2!(1. + 3. / 2., 1. + 3. / 2.),
        );
        assert!(contains((Vec2::new(1, 1), Vec2::new(4, 4)), vec2!(2., 2.)));

        assert!(has_intersect((Vec2::new(0, 0), Vec2::new(100, 0)), 30.));
        assert!(!has_intersect((Vec2::new(0, 0), Vec2::new(100, 0)), 120.));
        assert!(has_intersect((vec2!(0, 0), vec2!(1, 0)), 0.5));
        assert!(!has_intersect((vec2!(0, 0), vec2!(1, 0)), 1.1));
        assert!(is_inside(
            vec2!(30., 10.),
            &vec![(Vec2::new(0, 0), Vec2::new(100, 0))]
        ));
        assert!(!is_inside(
            vec2!(30., 10.),
            &vec![
                (Vec2::new(0, 0), Vec2::new(100, 0)),
                (Vec2::new(0, 5), Vec2::new(100, 5))
            ]
        ));
    }
}
