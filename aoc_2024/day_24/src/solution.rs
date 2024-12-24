use std::{
    collections::{HashMap, VecDeque},
    iter,
};

use regex::Regex;

use crate::error::Result;
pub struct Solution;

type Operation<'a> = (&'a str, &'a str, &'a str, &'a str);

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        let registers = calculate_register(input)?;

        let mut z_registers = extract_register(&registers, "z");
        z_registers.sort_by(|a, b: &(&str, u8)| b.0.cmp(a.0));

        let result = transform_to_number(&z_registers);

        Ok(result)
    }

    pub fn part_2(input: &str) -> Result<String> {
        let registers = calculate_register(input)?;

        let mut x_registers = extract_register(&registers, "x");
        let mut y_registers = extract_register(&registers, "y");
        let mut z_registers = extract_register(&registers, "z");

        x_registers.sort_by(|a, b: &(&str, u8)| a.0.cmp(b.0));
        y_registers.sort_by(|a, b: &(&str, u8)| a.0.cmp(b.0));
        z_registers.sort_by(|a, b: &(&str, u8)| a.0.cmp(b.0));

        let mut carry_over = 0;
        let mut swapped_values = vec![];
        x_registers
            .iter()
            .zip(y_registers.iter())
            .for_each(|(x, y)| {
                let (_, x_value) = x;
                let (_, y_value) = y;
                let z_key = format!("z{}", &x.0[1..]);
                let z_value = registers.get(z_key.as_str()).unwrap();

                let result = match (*x_value, *y_value) {
                    (0, 1) => {
                        if carry_over > 0 {
                            carry_over -= 1;
                            0
                        } else {
                            1
                        }
                    }
                    (1, 0) => {
                        if carry_over > 0 {
                            carry_over -= 1;
                            0
                        } else {
                            1
                        }
                    }
                    (1, 1) => {
                        if carry_over > 0 {
                            carry_over -= 1;
                            carry_over += 1;
                            1
                        } else {
                            carry_over += 1;
                            0
                        }
                    }
                    _ => 0, // (0, 0)
                };

                if result != *z_value {
                    swapped_values.push(z_key);
                }
            });

        let last_z = z_registers.last().unwrap();

        if last_z.1 != carry_over {
            swapped_values.push(last_z.0.to_string())
        }

        Ok(swapped_values.join(","))
    }
}

fn extract_register<'a, 'b>(
    registers: &HashMap<&'a str, u8>,
    needle: &'b str,
) -> Vec<(&'a str, u8)> {
    let registers: Vec<(&str, u8)> = registers
        .iter()
        .filter_map(|(k, val)| {
            if k.starts_with(needle) {
                Some((*k, *val))
            } else {
                None
            }
        })
        .collect();

    registers
}

fn transform_to_number(registers: &Vec<(&str, u8)>) -> u64 {
    let binary_string: String = registers.iter().map(|(_, val)| val.to_string()).collect();
    let result = u64::from_str_radix(&binary_string, 2).expect("conversion failed");

    result
}

fn calculate_register(input: &str) -> Result<HashMap<&str, u8>> {
    let (mut registers, operations) = load_registers_operations(input)?;
    let (mut direct_graph, indirect_graph) = operations.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut direct_graph, mut indirect_graph), op| {
            let (fir_op, _, sec_op, res_op) = op;

            let entry = direct_graph.entry(*res_op).or_insert(vec![]);
            entry.push(*fir_op);
            entry.push(*sec_op);

            indirect_graph
                .entry(*fir_op)
                .or_insert(vec![])
                .push(*res_op);
            indirect_graph
                .entry(*sec_op)
                .or_insert(vec![])
                .push(*res_op);

            (direct_graph, indirect_graph)
        },
    );
    registers.keys().for_each(|key| {
        direct_graph.entry(key).or_insert(vec![]);
    });
    let ordered_ops = topological_sort(direct_graph, indirect_graph);
    let operations = operations.iter().fold(HashMap::new(), |mut acc, item| {
        let (fir_op, op, sec_op, res_op) = item;

        acc.entry(*res_op).or_insert((*fir_op, *op, *sec_op));

        acc
    });
    for ops in ordered_ops {
        // first we check if we have a calculated value in the register for this ops
        if registers.contains_key(ops) {
            continue;
        } else {
            if let Some((fir_op, op, sec_op)) = operations.get(ops) {
                let first_val = registers.get(fir_op).unwrap();
                let second_val = registers.get(sec_op).unwrap();

                let value = match *op {
                    "OR" => *first_val | *second_val,
                    "XOR" => *first_val ^ *second_val,
                    _ => *first_val & *second_val,
                };

                registers.insert(ops, value);
            }
        }
    }

    Ok(registers)
}

fn topological_sort<'a>(
    direct_graph: HashMap<&'a str, Vec<&'a str>>,
    indirect_graph: HashMap<&'a str, Vec<&'a str>>,
) -> Vec<&'a str> {
    let mut sorted_operations = vec![];

    let mut dependency_count =
        direct_graph
            .iter()
            .fold(HashMap::new(), |mut acc, (op, children)| {
                acc.entry(op).or_insert(children.len());
                acc
            });

    let mut queue: VecDeque<&str> = dependency_count
        .iter()
        .filter(|(_, val)| val == &&0)
        .map(|(key, _)| **key)
        .collect();

    while let Some(op) = queue.pop_front() {
        sorted_operations.push(op);

        if let Some(parents) = indirect_graph.get(op) {
            for parent in parents {
                if let Some(count) = dependency_count.get_mut(&parent) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(parent);
                    }
                }
            }
        }
    }

    sorted_operations
}

fn load_registers_operations<'a>(
    input: &'a str,
) -> Result<(HashMap<&'a str, u8>, Vec<Operation<'a>>)> {
    let mut input = input.split("\n\n");

    let registers = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line.split(": ");

            let register = line.next().unwrap();
            let value = line.next().unwrap().parse::<u8>().unwrap();

            (register, value)
        })
        .collect();

    let re = Regex::new(
        r#"(?<fi>[a-z0-9]{3})\s(?<op>OR|XOR|AND)\s(?<sec>[a-z0-9]{3})\s->\s(?<res>[a-z0-9]{3})"#,
    )?;
    let operations = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            if let Some(captures) = re.captures(line) {
                let first_operand = captures.name("fi").unwrap().as_str();
                let operation = captures.name("op").unwrap().as_str();
                let second_operand = captures.name("sec").unwrap().as_str();
                let result_operand = captures.name("res").unwrap().as_str();

                (first_operand, operation, second_operand, result_operand)
            } else {
                panic!("Invalid input");
            }
        })
        .collect();

    Ok((registers, operations))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;
        assert_eq!(Solution::part_1(input)?, 2024);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;
        assert_eq!(Solution::part_2(input)?, "");

        Ok(())
    }
}
