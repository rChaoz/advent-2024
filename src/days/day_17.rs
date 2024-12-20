use super::DayInfo;
use regex::Regex;
use std::fmt::Write;

pub const INFO: DayInfo = DayInfo {
    name: "Chronospatial Computer",
    part1,
    part2,

    example1: "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",

    example2: "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
};

fn parse(input: &str) -> ([i64; 3], Vec<u8>) {
    let regex = Regex::new(
        r"^Register A: (-?\d+)\s+Register B: (-?\d+)\s+Register C: (-?\d+)\s+Program: (\d(?:,\d)*)$",
    )
    .unwrap();
    let c = regex.captures(input.trim()).unwrap();
    (
        [
            c[1].parse().unwrap(),
            c[2].parse().unwrap(),
            c[3].parse().unwrap(),
        ],
        c[4].split(',').map(|s| s.parse().unwrap()).collect(),
    )
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

fn run(mut register: [i64; 3], program: &Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0usize;

    macro_rules! combo {
        ($op:expr) => {
            match $op {
                0..=3 => $op as i64,
                4 => register[A],
                5 => register[B],
                6 => register[C],
                _ => panic!("invalid combo operand: {}", $op),
            }
        };
    }

    macro_rules! literal {
        ($op:expr) => {
            $op as i64
        };
    }

    while ip < program.len() {
        let instr = program[ip];
        let op = program[ip + 1];

        match instr {
            /* adv */ 0 => register[A] >>= combo!(op),
            /* bxl */ 1 => register[B] ^= literal!(op),
            /* bst */ 2 => register[B] = combo!(op) % 8,
            /* jnz */
            3 => {
                if register[A] != 0 {
                    ip = op as usize;
                    continue;
                }
            }
            /* bxc */ 4 => register[B] ^= register[C],
            /* out */ 5 => output.push((combo!(op) % 8) as u8),
            /* bdv */ 6 => register[B] = register[A] >> combo!(op),
            /* cdv */ 7 => register[C] = register[A] >> combo!(op),
            _ => panic!("invalid opcode: {instr}"),
        }
        ip += 2;
    }

    output
}

fn part1(input: &str) {
    let (register, program) = parse(input);
    let mut output = run(register, &program)
        .iter()
        .fold(String::new(), |mut s, n| {
            write!(s, "{n},").unwrap();
            s
        });
    output.pop(); // remove the last comma
    println!("{output}");
}

/**

# Analyzing the example program

## Input program

```text
0,3,5,4,3,0
```

## Assembly

```text
0,3: A >>= 3
5,4: out A % 8
3,0: if A != 0 jump to start
```

## Pseudo-code

```text
do {
    A >>= 3
    print A % 8
} while A != 0
```

In other words, this splits A into 3-bit blocks and prints each.
As the program is `0,3,5,4,3,0`, we need the output to print 0, then 3, then 5, etc., which means the 3-bit blocks
that compose A (right-to-left) must be `000` (0), `011` (3), `101` (5) etc.

## Solution

```text
  need an additional any 3 digits at the start as it starts with A >>= 3,
  we choose 000 as it asks for the smallest number
            v
A = reverse(0, 0, 3, 5, 4, 3, 0).join(), where each number is written in base 2 as a length-3 0-padded string
A = reverse(000,000,011,101,100,011,000).join()
A = [000,011,100,101,011,000,000].join()
A = 11100101011000000
A = 117440 (in base 10)
```

# Analyzing my input

A general solution for all possible programs is hard/impossible to make, so I'll analyze my input.

## Input

```text
Register A: 64854237
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,1,5,4,0,5,5,0,3,3,0
```

## Assembly

```text
2,4: B = A % 8
1,1: B = B ^ 1
7,5: C = A >> B
1,5: B = B ^ 5
4,0: B = B ^ C
5,5: out B % 8
0,3: A = A >> 3
3,0: if A != 0 jump to start
```

## Pseudo-code

```text
do {
    B = (A % 8) ^ 1
    C = A >> B
    print (B ^ 5 ^ C) % 8
    A >>= 3
} while (A != 0)
```

Unlike the example, where the output depends on 3 bits at a time, while A is still processed in 3-bit blocks,
the output depends on the entire A value, which requires solving backwards.

## Solution

When the last number is outputted, A needs to be > 0, but A >> 3 becomes 0, so A is between 1 and 7, we'll call this a.
The last number outputted is 0, which means:

```text
(B ^ 5 ^ C) % 8 == 0
B ^ C == b101

B = a ^ 1
C = a >> B = a >> (a ^ 1)
a ^ 1 ^ (a >> (a ^ 1)) = b101 | ^1
a ^ (a >> (a ^ 1)) = b100

If a is:
< 4: the output cannot be 4
= 4: 4 ^ (4 >> 5) = 4
= 5: 5 ^ (5 >> 4) = 5
= 6: 6 ^ (6 >> 7) = 6
= 7: 7 ^ (7 >> 6) = 7
```

So, the last 3 digits of A must be `100`. We can continue for the next digits the same way.
The solution uses backtracking in case there are multiple solutions at one point.

# Final notes

In order to work for both the example and my program, the backtracking algorithm tries to fill A with 3 bits at a time,
this solution does not work if the program contains other shift-amounts.
 */
fn part2(input: &str) {
    let (register, program) = parse(input);

    fn backtrack(program: &Vec<u8>, index: usize, mut register: [i64; 3], num: i64) -> bool {
        let expected = &program[index..];
        for a in 0..8 {
            let num = num << 3 | a;
            register[A] = num;
            if run(register, program) != expected {
                continue;
            }
            if index == 0 {
                println!("{num}");
                return true;
            }
            if backtrack(program, index - 1, register, num) {
                return true;
            }
        }
        false
    }

    backtrack(&program, program.len() - 1, register, 0);
}
