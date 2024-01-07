use std::collections::{BinaryHeap, HashMap};



fn parse(text: &str) -> Vec<Vec<i32>> {
    text
    .split("\n")
    .filter(|l| !l.is_empty())
    .map(|l| {
        l
        .chars()
        .map(
            |c| 
            i32::from_str_radix(c.to_string().as_str(), 10).unwrap()
        )
        .collect()
    }).collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: (i32, i32),
    cost: i32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_four_adjacent(p: (i32, i32), w: usize, h: usize) -> Vec<(i32, i32)> {
    vec![
        (p.0 + 1, p.1 + 0),
        (p.0 - 1, p.1 + 0),
        (p.0 + 0, p.1 + 1),
        (p.0 + 0, p.1 - 1),
    ]
    .iter()
    .filter(
        |(x, y)|
        &0 <= x 
        && (*x as usize) < w 
        && &0 <= y 
        && (*y as usize) < h
    )
    .cloned()
    .collect()
}

type Costs = HashMap<(i32, i32), i32>;
type Predecessors = HashMap<(i32, i32), Option<(i32, i32)>>;

fn get_self_and_predecessors(predecessors: &Predecessors, current: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut acc: Vec<(i32, i32)> = vec![];

    let mut possible_cursor: Option<(i32, i32)> = Some(current.clone());
    while let Some(cursor) = possible_cursor {
        acc.push(cursor);
        possible_cursor = predecessors.get(&cursor).unwrap().clone();
    }
    acc
}

fn dijkstra(heats: &Vec<Vec<i32>>) -> (Costs, Predecessors) {

    let (w, h) = (heats.iter().next().unwrap().len(), heats.len());

    let mut costs: HashMap<(i32, i32), i32> = heats
    .iter().enumerate().map(
        |(y, row)| row.iter().enumerate().map(
            |(x, heat)| ((x as i32, y as i32), i32::MAX)
        ).collect::<Vec<((i32, i32), i32)>>()
    ).flatten().collect();
    costs.insert((0, 0), 0);

    let mut predecessors: HashMap<(i32, i32), Option<(i32, i32)>> = heats
    .iter().enumerate().map(
        |(y, row)| row.iter().enumerate().map(
            |(x, heat)| ((x as i32, y as i32), None)
        )
        .collect::<Vec<((i32, i32), Option<(i32, i32)>)>>()
    ).flatten().collect();

    let mut open_heap: BinaryHeap<State> = BinaryHeap::new();
    open_heap.push(State{position: (0, 0), cost: 0});

    while let Some(state) = open_heap.pop() {
        let position = state.position;
        let cost = state.cost;
        // if &cost > costs.get(&position).unwrap() {
        //     continue;
        // }

        let n_history = 4;
        let self_and_predecessors = get_self_and_predecessors(&predecessors, &position);
        let few_predecessors = self_and_predecessors.iter().take(n_history).collect::<Vec<_>>();

        for adj in get_four_adjacent(position, w, h).iter() {

            let mut x_positions = few_predecessors.iter().map(|(x, y)| x).collect::<Vec<_>>();
            x_positions.push(&adj.0);
            let mut y_positions = few_predecessors.iter().map(|(x, y)| y).collect::<Vec<_>>();
            y_positions.push(&adj.1);
            if 
            x_positions.len() > n_history && (
                x_positions.iter().min() == x_positions.iter().max() 
                || y_positions.iter().min() == y_positions.iter().max()
            ) {
                continue;
            }
            
            let adj_heat = heats[adj.1 as usize][adj.0 as usize];
            let adj_cost = costs.get(adj).unwrap();

            let new_cost = cost + adj_heat;

            if &new_cost < adj_cost {
                predecessors.insert(adj.clone(), Some(position));
                costs.insert(adj.clone(), new_cost);
                open_heap.push(State { position: adj.clone(), cost: new_cost});
            }
        }
    }
    (costs, predecessors)
}

fn show(predecessors: &Predecessors, heats: &Vec<Vec<i32>>) {
    let (w, h) = (heats.iter().next().unwrap().len(), heats.len());

    let end = &(w as i32 - 1, h as i32 - 1);

    let history = get_self_and_predecessors(predecessors, &end);

    let mut mp = vec![vec!["."; w]; h];

    for (x, y) in history.iter() {
        mp[*y as usize][*x as usize] = "#";
    }
    for (y, row) in mp.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            print!("{cell}");
        }
        println!();
    }
}

pub fn first_part(input: &str) -> i32 {
    let heats = parse(input);
    let (w, h) = (heats.iter().next().unwrap().len(), heats.len());

    let (costs, predecessors) = dijkstra(&heats);
    show(&predecessors, &heats);

    dbg!(costs.get(&(0, 0)));
    dbg!(costs.get(&(1, 0)));
    dbg!(costs.get(&(0, 1)));
    
    dbg!(costs.get(&(w as i32 - 1, h as i32 - 1)));


    dbg!(predecessors.get(&(0, 0)));
    dbg!(predecessors.get(&(1, 0)));
    dbg!(predecessors.get(&(0, 1)));

    dbg!(predecessors.get(&(w as i32 - 1, h as i32 - 1)));

    unimplemented!()
}

pub fn second_part(input: &str) -> i32 {
    return -1;
}

#[cfg(test)]
mod tests {
    use crate::day_17::{first_part, second_part};

    use super::{get_self_and_predecessors, Predecessors};
    
    #[test]
    fn test_stuff() {
        let pred: Predecessors = vec![
            ((0, 0), None), 
            ((1, 1), Some((0, 0))),
        ].iter().cloned().collect::<Predecessors>();
        let x = get_self_and_predecessors(&pred, &(1, 1));
        assert_eq!(x, vec![(1, 1), (0, 0)]);
    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/17_example_1.txt")), 102);
    }
    
    // #[test]
    // fn test_parts() {
    //     assert_eq!(first_part(include_str!("inputs/17.secret")), 0);
    //     assert_eq!(second_part(include_str!("inputs/17.secret")), 0);
    // }
}