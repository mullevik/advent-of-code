use std::{collections::{HashMap, VecDeque}, default, iter::FlatMap};

use itertools::Itertools;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High
}

#[derive(Debug)]
enum ModuleKind {
    FlipFlop,
    Broadcast,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    name: String,
    kind: ModuleKind,
    outputs: Vec<String>
}


#[derive(Debug, Clone)]
enum ModuleState {
    FlipFlopState(bool),
    ConjunctionState(HashMap<String, Pulse>),
    None
}

fn parse_line(line: &str) -> (String, Module) {
    let (name, outputs) = line.split_once("->").unwrap();

    let (actual_name, kind): (String, ModuleKind) = {
        if name.starts_with("b") {
            (name.trim().to_string(), ModuleKind::Broadcast)
        } else if name.starts_with("%") {
            (name[1..name.len()].trim().to_string(), ModuleKind::FlipFlop)
        } else if name.starts_with("&") {
            (name[1..name.len()].trim().to_string(), ModuleKind::Conjunction)
        } else {
            dbg!(name);
            panic!("Unexpected name {name:?}");
        }
    };

    let outputs = outputs.split(",").map(|x| x.trim().to_string()).collect();
    
    (actual_name.clone(), Module{name: actual_name.clone(), kind: kind, outputs: outputs})
}

fn parse(text: &str) -> HashMap<String, Module> {
    text.split("\n")
    .filter(|x| !x.is_empty())
    .map(parse_line).collect()
}

fn simulate_broadcast(module: &Module, pulse: Pulse, pulse_queue: &mut VecDeque<(String, Pulse, String)>, states: &mut HashMap<String, ModuleState>) {
    for output in module.outputs.iter() {
        pulse_queue.push_front((module.name.clone(), pulse, output.clone()))
    }
}

fn simulate_flip_flop(module: &Module, pulse: Pulse, pulse_queue: &mut VecDeque<(String, Pulse, String)>, states: &mut HashMap<String, ModuleState>) {
    match pulse {
        Pulse::High => (),
        Pulse::Low => {
            let state = states.get(&module.name).unwrap();
            match state {
                ModuleState::FlipFlopState(x) => {
                    match x {
                        false => {
                            states.insert(module.name.clone(), ModuleState::FlipFlopState(true));
                            for output in module.outputs.iter() {
                                pulse_queue.push_front((module.name.clone(), Pulse::High, output.clone()));
                            }
                        },
                        true => {
                            states.insert(module.name.clone(), ModuleState::FlipFlopState(false));
                            for output in module.outputs.iter() {
                                pulse_queue.push_front((module.name.clone(), Pulse::Low, output.clone()));
                            }

                        }
                    } 
                }
                _ => panic!("Unknown flip flop state {state:?}")
            }
        }
    }
}

fn simulate_conjunction(last_name: String, module: &Module, pulse: Pulse, pulse_queue: &mut VecDeque<(String, Pulse, String)>, states: &mut HashMap<String, ModuleState>) {
    let latest_state = states.get(&module.name).unwrap();
    match latest_state {
        ModuleState::ConjunctionState(_sts) => {
            let new_sts: HashMap<String, Pulse> = _sts
            .iter()
            .map(|(remembered_i, remembered_pulse)| {
                if remembered_i == &last_name {
                    (remembered_i.clone(), pulse)
                } else {
                    (remembered_i.clone(), remembered_pulse.clone())
                }
            })
            .collect();
            states.insert(module.name.clone(), ModuleState::ConjunctionState(new_sts.clone()));

            if new_sts.values().all(|p| matches!(p, Pulse::High)) {
                for output in module.outputs.iter() {
                    pulse_queue.push_front((module.name.clone(), Pulse::Low, output.clone()))
                }
            } else {
                for output in module.outputs.iter() {
                    pulse_queue.push_front((module.name.clone(), Pulse::High, output.clone()))
                }
            }
        },
        _ => panic!("Unknown state {latest_state:?}")
    }

}

fn simulate(modules: &HashMap<String, Module>, states: &mut HashMap<String, ModuleState>) -> Vec<(Pulse, String)> {
    
    let mut pulse_queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
    pulse_queue.push_front(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

    let mut saved_pulses: Vec<(Pulse, String)> = Vec::new();

    while !pulse_queue.is_empty() {
        let (last_name, current_pulse, current_name) = pulse_queue.pop_back().unwrap();
        let x = ["qt".to_string(), "rx".to_string(), "mr".to_string(), "kk".to_string(), "bb".to_string(), "gl".to_string()];
        if x.contains(&last_name) || x.contains(&current_name) {
            println!("{last_name:?} -{current_pulse:?} {current_name:?}");
        }
        saved_pulses.push((current_pulse, current_name.clone()));

        match modules.get(&current_name) {
            Some(m) => {
                match m.kind {
                    ModuleKind::Broadcast => simulate_broadcast(m, current_pulse, &mut pulse_queue, states),
                    ModuleKind::FlipFlop => simulate_flip_flop(m, current_pulse, &mut pulse_queue, states),
                    ModuleKind::Conjunction => simulate_conjunction(last_name, m, current_pulse, &mut pulse_queue, states)
                }
            },
            None => ()
        }
    }

    saved_pulses
}

fn _initialize_conjunction_state(modules: &HashMap<String, Module>, module: &Module) -> ModuleState {

    let state_map = modules
    .iter()
    .map(|(_n, _m)| match _m.outputs.iter().find(|x| x == &&module.name) {
        Some(_) => Some(_n),
        None => None
    })
    .filter_map(|x| x)
    .map(|x| (x.to_owned(), Pulse::Low))
    .collect();

    ModuleState::ConjunctionState(state_map)
}

fn initialize_states(modules: &HashMap<String, Module>) -> HashMap<String, ModuleState> {
    modules.iter().map(
        |(name, module)| {
            match module.kind {
                ModuleKind::Broadcast => (name.to_owned(), ModuleState::None),
                ModuleKind::FlipFlop => (name.to_owned(), ModuleState::FlipFlopState(false)),
                ModuleKind::Conjunction => (name.to_owned(), _initialize_conjunction_state(modules, module)),
            }
        }
    ).collect()
}

pub fn first_part(input: &str) -> i32 {
    let modules = parse(input);
    
    let mut states = initialize_states(&modules);
    let mut pulses = Vec::new();
    for _ in 0..1000 {
        pulses.push(simulate(&modules, &mut states));
    }
    let n_low_pulses = pulses.iter().flatten().filter(|(p, d)| matches!(p, Pulse::Low)).count();
    let n_hight_pulses = pulses.iter().flatten().filter(|(p, d)| matches!(p, Pulse::High)).count();
    (n_low_pulses * n_hight_pulses) as i32
}

pub fn second_part(input: &str) -> i64 {
    let modules = parse(input);
    
    let mut states = initialize_states(&modules);
    let mut n_presses = 0i64;
    loop {
        n_presses += 1;
        let pulses = simulate(&modules, &mut states);
        let res = pulses.iter().find(|(p, d)| (p, d) == (&Pulse::Low, &"rx".to_string()));
        if matches!(res, Some(x)) {
            return n_presses
        } else {
            // println!("{n_presses}")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_20::{first_part, second_part};
    
    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/20_example_1.txt")), 32000000);
        assert_eq!(first_part(include_str!("inputs/20_example_2.txt")), 11687500);
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/20.secret")), 879834312);
        // assert_eq!(second_part(include_str!("inputs/20.secret")), 0);
    }
}