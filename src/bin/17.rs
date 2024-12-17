use core::panic;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let data_registers = data[0].split("\n").collect::<Vec<&str>>();
    let data_program = data[1].trim().replace("Program: ", "");

    let mut reg_a = data_registers[0]
        .replace("Register A: ", "")
        .parse::<u64>()
        .unwrap();
    let mut reg_b = data_registers[1]
        .replace("Register B: ", "")
        .parse::<u64>()
        .unwrap();
    let mut reg_c = data_registers[2]
        .replace("Register C: ", "")
        .parse::<u64>()
        .unwrap();
    let program = data_program.split(",").collect::<Vec<&str>>();
    let mut instruction_pointer = 0;
    let mut output: Vec<String> = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1].parse::<u64>().unwrap();

        // bxl instruction (opcode 1)
        // calculates the bitwise XOR of register B and the instruction's literal operand
        // the result is stored in the B register
        if opcode == "1" {
            reg_b = reg_b ^ operand;
            instruction_pointer += 2;
            continue;
        }

        // litteral operand
        // jnz instruction (opcode 3)
        // does nothing if the A register is 0
        // else jumps by setting the instruction pointer to the value of its literal operand
        // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
        if opcode == "3" {
            if reg_a != 0 {
                instruction_pointer = operand as usize;
            } else {
                instruction_pointer += 2;
            }

            continue;
        }

        let combo = match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => {
                panic!("Invalid operand");
            }
        };

        match opcode {
            // adv instruction (opcode 0) performs division
            // numerator is the value in the A register
            // denominator is found by raising 2 to the power of the instruction's combo operand
            // the result is stored in the A register
            "0" => {
                let numerator = reg_a;
                let denominator = 2u64.pow(combo as u32);
                reg_a = numerator / denominator;
            }
            // bst instruction (opcode 2)
            // calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits)
            // the result is stored in the B register
            "2" => {
                reg_b = combo % 8;
            }
            // jnz instruction (opcode 3)
            // does nothing if the A register is 0
            // else jumps by setting the instruction pointer to the value of its literal operand
            // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            "4" => {
                reg_b = reg_b ^ reg_c;
            }
            // out instruction (opcode 5)
            // calculates the value of its combo operand modulo 8, then outputs that value
            // (If a program outputs multiple values, they are separated by commas.)
            "5" => {
                let modulo = (combo % 8).to_string();
                output.push(modulo);
            }
            // bdv instruction (opcode 6)
            // works exactly like the adv instruction except that the result is stored in the B register
            // (The numerator is still read from the A register.)
            "6" => {
                let numerator = reg_a;
                let denominator = 2u64.pow(combo as u32);
                reg_b = numerator / denominator;
            }
            // cdv instruction (opcode 7)
            // works exactly like the adv instruction except that the result is stored in the C register
            // (The numerator is still read from the A register.)
            "7" => {
                let numerator = reg_a;
                let denominator = 2u64.pow(combo as u32);
                reg_c = numerator / denominator;
            }
            _ => panic!("Invalid opcode"),
        }

        instruction_pointer += 2;
    }

    Some(output.join(","))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, None);
    }
}
