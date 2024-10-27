use std::ops::Range;

use rustc_hash::FxHashSet;

pub fn first_part(input: &str) -> i64 {
    find_model_number_trying_digits(
        &parse(include_str!("../inputs/24.in")),
        &[9, 8, 7, 6, 5, 4, 3, 2, 1],
    )
}
pub fn second_part(input: &str) -> i64 {
    find_model_number_trying_digits(
        &parse(include_str!("../inputs/24.in")),
        &[1, 2, 3, 4, 5, 6, 7, 8, 9],
    )
}

fn find_model_number_trying_digits(operations: &[Operation], digits: &[i32]) -> i64 {
    let modules = split_into_modules(operations);
    let mut banned_states = FxHashSet::default();
    find_model_number(0, 0, 0, &modules, &mut banned_states, digits).unwrap()
}

type Registers = [i32; 4];

const EMPTY_REGISTERS: Registers = [0, 0, 0, 0];
const MAX_ALU_VALUE: i32 = 10_000_000;

#[derive(Debug, Clone, Copy)]
enum Symbol {
    Constant(i32),
    VariableIndex(usize),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Inp(Symbol),
    Add(Symbol, Symbol),
    Mul(Symbol, Symbol),
    Div(Symbol, Symbol),
    Mod(Symbol, Symbol),
    Eql(Symbol, Symbol),
}

struct Module {
    operations: Vec<Operation>,
}

fn execute(operations: &[Operation], inputs: &[i32], starting_registers: &Registers) -> Registers {
    let mut registers: Registers = *starting_registers;

    let mut input_iter = inputs.iter();

    for op in operations {
        match *op {
            Operation::Inp(Symbol::VariableIndex(var)) => {
                registers[var] = *input_iter.next().expect("Ran out of input sequence");
            }
            Operation::Add(Symbol::VariableIndex(a), b) => {
                registers[a] = registers[a] + value(b, &registers);
            }
            Operation::Mul(Symbol::VariableIndex(a), b) => {
                registers[a] = registers[a] * value(b, &registers);
            }
            Operation::Div(Symbol::VariableIndex(a), b) => {
                registers[a] = registers[a] / value(b, &registers);
            }
            Operation::Mod(Symbol::VariableIndex(a), b) => {
                registers[a] = registers[a] % value(b, &registers);
            }
            Operation::Eql(Symbol::VariableIndex(a), b) => {
                registers[a] = if registers[a] == value(b, &registers) {
                    1
                } else {
                    0
                };
            }
            _ => panic!("Unexpected operation '{:?}'", op),
        }
    }

    registers
}

fn value(s: Symbol, registers: &Registers) -> i32 {
    match s {
        Symbol::Constant(c) => c,
        Symbol::VariableIndex(var) => registers[var],
    }
}

impl Module {
    fn execute(&self, carry: i32, input: i32) -> Registers {
        execute(&self.operations, &[input], &[0, 0, carry, 0])
    }
}

fn var_name_to_index(name: &str) -> usize {
    if name == "x" {
        0
    } else if name == "y" {
        1
    } else if name == "z" {
        2
    } else if name == "w" {
        3
    } else {
        panic!("Unexpected variable name '{}'", name);
    }
}

fn build_symbol(input: &str) -> Symbol {
    if let Ok(number) = input.parse::<i32>() {
        Symbol::Constant(number)
    } else {
        Symbol::VariableIndex(var_name_to_index(input))
    }
}

fn parse_operation(input: &str) -> Operation {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    if input.starts_with("inp") {
        Operation::Inp(build_symbol(parts[1]))
    } else if input.starts_with("add") {
        Operation::Add(build_symbol(parts[1]), build_symbol(parts[2]))
    } else if input.starts_with("mul") {
        Operation::Mul(build_symbol(parts[1]), build_symbol(parts[2]))
    } else if input.starts_with("div") {
        Operation::Div(build_symbol(parts[1]), build_symbol(parts[2]))
    } else if input.starts_with("mod") {
        Operation::Mod(build_symbol(parts[1]), build_symbol(parts[2]))
    } else if input.starts_with("eql") {
        Operation::Eql(build_symbol(parts[1]), build_symbol(parts[2]))
    } else {
        panic!("Unknown operation '{}'", parts[0])
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_operation)
        .collect::<Vec<Operation>>()
}

fn split_into_modules(operations: &[Operation]) -> Vec<Module> {
    let mut modules = vec![];
    for op in operations {
        if let Operation::Inp(_) = op {
            modules.push(Module {
                operations: vec![*op],
            });
        } else {
            let mut last_module = modules
                .last_mut()
                .expect("No previous module found - does the sequence start with 'inp'?");
            last_module.operations.push(*op);
        }
    }
    modules
}

fn find_model_number(
    model_number: i64,
    z_carry: i32,
    module_idx: usize,
    modules: &[Module],
    banned_states: &mut FxHashSet<(i32, usize)>,
    input_digits: &[i32],
) -> Option<i64> {
    if banned_states.contains(&(z_carry, module_idx)) {
        return None;
    }

    if module_idx >= modules.len() {
        return None;
    }

    if z_carry >= MAX_ALU_VALUE || z_carry <= -MAX_ALU_VALUE {
        return None; // z_carry got out of hand -> ignore such states
    }

    let module = &modules[module_idx];

    for input_digit in input_digits {
        let registers = module.execute(z_carry, *input_digit);
        let produced_z_val = registers[2];
        let produced_model_number = model_number * 10 + (*input_digit) as i64;

        if module_idx == 13 && produced_z_val == 0 {
            return Some(produced_model_number);
        }

        let new_model_number = find_model_number(
            produced_model_number,
            produced_z_val,
            module_idx + 1,
            modules,
            banned_states,
            input_digits,
        );

        if let Some(n) = new_model_number {
            return Some(n);
        }
    }

    banned_states.insert((z_carry, module_idx));

    None
}

#[cfg(test)]
mod tests_day_24 {

    use super::*;

    const EXAMPLE_INPUT: &str = "inp z\ninp x\nmul z 3\neql z x";

    #[test]
    fn test_parse_operations() {
        let operations = parse(EXAMPLE_INPUT);
        assert_eq!(operations.len(), 4);
        assert!(matches!(operations[0], Operation::Inp(_)));
        assert!(matches!(operations[3], Operation::Eql(_, _)));
    }

    #[test]
    fn test_modules() {
        let modules = split_into_modules(&parse(EXAMPLE_INPUT));
        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].operations.len(), 1);
        assert_eq!(modules[1].operations.len(), 3);
    }

    #[test]
    fn test_execute() {
        let operations = parse(EXAMPLE_INPUT);
        let registers = execute(&operations, &[1, 3], &EMPTY_REGISTERS);
        assert_eq!(registers, [3, 0, 1, 0]);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/24.in")), 39924989499969);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/24.in")), -1);
    }
}
