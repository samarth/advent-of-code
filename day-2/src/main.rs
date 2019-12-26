fn operate(code: u32, operand_one: u32, operand_two: u32) -> u32 {
    match code {
        1 => operand_one + operand_two,
        2 => operand_one * operand_two,
        _ => panic!("Unexpected operator"),
    }
}

fn compute(address_one_value: u32, address_two_value: u32, input: &mut Vec<u32>) -> &Vec<u32> {
    // Start from the beginning.
    let mut opcode_position = 0;
    const EXIT_CODE: u32 = 99;

    input[1] = address_one_value;
    input[2] = address_two_value;

    while input[opcode_position] != EXIT_CODE && opcode_position + 3 < input.len() {
        let operand_one_position = input[opcode_position + 1] as usize;
        let operand_two_position = input[opcode_position + 2] as usize;
        let result_position = input[opcode_position + 3] as usize;
        input[result_position] = operate(
            input[opcode_position],
            input[operand_one_position],
            input[operand_two_position],
        );
        opcode_position += 4;
    }
    return input;
}

fn main() {
    let input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 9, 19, 23, 1, 23, 5, 27, 2,
        27, 10, 31, 1, 6, 31, 35, 1, 6, 35, 39, 2, 9, 39, 43, 1, 6, 43, 47, 1, 47, 5, 51, 1, 51,
        13, 55, 1, 55, 13, 59, 1, 59, 5, 63, 2, 63, 6, 67, 1, 5, 67, 71, 1, 71, 13, 75, 1, 10, 75,
        79, 2, 79, 6, 83, 2, 9, 83, 87, 1, 5, 87, 91, 1, 91, 5, 95, 2, 9, 95, 99, 1, 6, 99, 103, 1,
        9, 103, 107, 2, 9, 107, 111, 1, 111, 6, 115, 2, 9, 115, 119, 1, 119, 6, 123, 1, 123, 9,
        127, 2, 127, 13, 131, 1, 131, 9, 135, 1, 10, 135, 139, 2, 139, 10, 143, 1, 143, 5, 147, 2,
        147, 6, 151, 1, 151, 5, 155, 1, 2, 155, 159, 1, 6, 159, 0, 99, 2, 0, 14, 0,
    ];

    let mut my_copy : Vec<u32> = input.clone();
    // Part 1
    let result = compute(12, 2, &mut my_copy);
    println!("{}", result[0]);

    // part 2
    let expected_output = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            if compute(noun, verb, &mut input.clone())[0] == expected_output {
                println!("{}", (100 * noun) + verb);
            }
        }
    }
}
