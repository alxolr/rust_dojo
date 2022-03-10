use std::collections::HashMap;

type Registers = HashMap<String, i64>;

pub fn simple_assembler(program: Vec<&str>) -> Registers {
    let mut registers = HashMap::new();
    let mut step: usize = 0;

    loop {
        let value = program[step];
        let chunks_of_command: Vec<_> = value.split(" ").collect();
        let command = chunks_of_command.first().unwrap();

        match *command {
            "mov" => {
                let register = chunks_of_command[1];
                let value = chunks_of_command[2];

                if is_in_register(value, &registers) {
                    registers.insert(register.to_string(), registers.get(value).unwrap().clone());
                } else {
                    registers.insert(register.to_string(), value.parse::<i64>().unwrap());
                }
            }
            "inc" => {
                let register = chunks_of_command[1];
                *registers.get_mut(register).unwrap() += 1;
            }
            "dec" => {
                let register = chunks_of_command[1];
                *registers.get_mut(register).unwrap() -= 1;
            }
            "jnz" => {
                let value = chunks_of_command[1];
                let steps = chunks_of_command[2].parse::<i64>().unwrap();

                let jump = if is_in_register(value, &registers) {
                    if *registers.get(value).unwrap() != 0i64 {
                        true
                    } else {
                        false
                    }
                } else {
                    if value.parse::<i64>().unwrap() != 0i64 {
                        true
                    } else {
                        false
                    }
                };

                if jump {
                    step = (step as i64 + steps) as usize;
                    if step >= program.len() {
                        break;
                    }
                    continue;
                }
            }
            _ => panic!("Command not supported"),
        }

        step += 1;

        if step >= program.len() {
            break;
        }
    }

    registers
}

pub fn is_in_register(value: &str, registers: &Registers) -> bool {
    let registers = registers.keys().into_iter().collect::<Vec<_>>();

    registers.contains(&&value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));

        let program = vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ];
        let expected = map! { "a" => 409600, "c" => 409600, "b" => 409600};
        compare_registers(expected, simple_assembler(program));
    }

    #[test]
    fn test_minus_program() {
        let program = vec!["mov a -10", "mov b a", "inc a", "dec b", "jnz a -2"];

        let expected = map! { "a" => 0, "b" => -20 };
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}
