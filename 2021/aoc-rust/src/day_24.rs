use std::ops::Range;

use itertools::Itertools;

pub fn first_part(input: &str) -> i32 {
    todo!()
}
pub fn second_part(input: &str) -> i32 {
    todo!()
}
type Registers = [i32; 4];

const EMPTY_REGISTERS: Registers = [0, 0, 0, 0];

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

impl Module {
    fn execute(&self, carry: i32, input: i32) -> Registers {
        execute(&self.operations, &[input], &[0, 0, carry, 0])
    }

    fn possible_carries_and_inputs_for(
        &self,
        target_carry: i32,
        space: Range<i32>,
    ) -> Vec<(i32, i32)> {
        space
            .cartesian_product(1..10)
            .filter(|(carry, input)| self.execute(*carry, *input)[2] == target_carry)
            .collect::<Vec<_>>()
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

fn split_into_modules(operations: &[Operation]) -> Vec<Vec<Operation>> {
    let mut modules = vec![];
    for op in operations {
        if let Operation::Inp(_) = op {
            modules.push(vec![*op]);
        } else {
            let mut last_module = modules
                .last_mut()
                .expect("No previous module found - does the sequence start with 'inp'?");
            last_module.push(*op);
        }
    }
    modules
}

#[cfg(test)]
mod tests_day_24 {
    use std::collections::HashSet;

    use itertools::Itertools;

    use super::*;

    const EXAMPLE_INPUT: &str = "inp z\ninp x\nmul z 3\neql z x";

    const MODULE_FOR_DIGIT_14: &str = "
inp w
mul x 0
add x z
mod x 26
div z 26
add x -3
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y";

    const MODULE_FOR_DIGIT_13: &str = "inp w
mul x 0
add x z
mod x 26
div z 26
add x -9
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y";

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
        assert_eq!(modules[0].len(), 1);
        assert_eq!(modules[1].len(), 3);
    }

    #[test]
    fn test_execute() {
        let operations = parse(EXAMPLE_INPUT);
        let registers = execute(&operations, &[1, 3], &EMPTY_REGISTERS);
        assert_eq!(registers, [3, 0, 1, 0]);
    }

    #[test]
    fn test_module_values() {
        let module_14 = Module {
            operations: parse(MODULE_FOR_DIGIT_14),
        };
        let module_13 = Module {
            operations: parse(MODULE_FOR_DIGIT_13),
        };

        let possibilities_for_14 = module_14.possible_carries_and_inputs_for(0, -1000..1000);

        // let possibilities_for_13 = possibilities_for_14
        //     .iter()
        //     .flat_map(|(carry, _)| module_13.possible_carries_and_inputs_for(*carry, -1000..1000))
        //     .collect::<Vec<_>>();
        let possibilities_for_13 = module_13.possible_carries_and_inputs_for(4, -1000..1000);
        println!("pos 14 {:?}", possibilities_for_14);
        println!("pos 13 {:?}", possibilities_for_13);

        let r = module_13.execute(122, 9);
        assert_eq!(module_14.execute(r[2], 1)[2], 0);
    }

    // #[test]
    // fn test_first_part() {
    //     assert_eq!(first_part(include_str!("../inputs/.in")), -1);
    // }
    // #[test]
    // fn test_second_part() {
    //     assert_eq!(second_part(include_str!("../inputs/.in")), -1);
    // }
}
