use super::DayInfo;
use crate::api::is_example;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub const INFO: DayInfo = DayInfo {
    name: "Crossed Wires",
    part1,
    part2,

    example1: "\
x00: 1
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
tnw OR pbm -> gnj",

    example2: "\
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00",
};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Gate<'a> {
    input_a: &'a str,
    input_b: &'a str,
    output: &'a str,
    gate_type: GateType,
}

impl<'a> Gate<'a> {
    fn run(&self, values: &mut HashMap<&'a str, bool>) {
        let a = values[self.input_a];
        let b = values[self.input_b];
        values.insert(
            self.output,
            match self.gate_type {
                GateType::AND => a & b,
                GateType::OR => a | b,
                GateType::XOR => a ^ b,
            },
        );
    }

    fn try_run(&self, values: &mut HashMap<&'a str, bool>) -> bool {
        if values.contains_key(self.input_a) && values.contains_key(self.input_b) {
            self.run(values);
            true
        } else {
            false
        }
    }
}

impl Display for Gate<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.input_a, self.gate_type, self.input_b, self.output
        )
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum GateType {
    AND,
    OR,
    XOR,
}

impl Display for GateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GateType::AND => f.write_str("AND"),
            GateType::OR => f.write_str("OR"),
            GateType::XOR => f.write_str("XOR"),
        }
    }
}

impl FromStr for GateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateType::AND),
            "OR" => Ok(GateType::OR),
            "XOR" => Ok(GateType::XOR),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct ParseResult<'a> {
    wires: HashSet<&'a str>,
    initial: HashMap<&'a str, bool>,
    gates: Vec<Gate<'a>>,
}

fn parse(input: &str) -> ParseResult<'_> {
    let mut wires = HashSet::new();
    let mut lines = input.lines();
    let mut initial = HashMap::new();
    let mut gates = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut it = line.split(": ");
        let wire = it.next().unwrap();
        let value = it.next().unwrap();
        assert_eq!(it.next(), None);
        assert!(value == "0" || value == "1");
        initial.insert(wire, value == "1");
        wires.insert(wire);
    }

    let regex = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
    while let Some(line) = lines.next() {
        let captures = regex.captures(line).unwrap();
        let mut groups = captures.iter();
        groups.next(); // Skip the full match
        let input_a = groups.next().unwrap().unwrap().as_str();
        let gate_type: GateType = groups.next().unwrap().unwrap().as_str().parse().unwrap();
        let input_b = groups.next().unwrap().unwrap().as_str();
        let output = groups.next().unwrap().unwrap().as_str();
        gates.push(Gate {
            input_a,
            input_b,
            output,
            gate_type,
        });
        wires.insert(input_a);
        wires.insert(input_b);
        wires.insert(output);
    }

    ParseResult {
        wires,
        initial,
        gates,
    }
}

fn form_output(values: &HashMap<&str, bool>) -> u64 {
    values
        .iter()
        .filter_map(|(&wire, &value)| {
            if wire.starts_with("z") {
                let shift = wire[1..].parse::<u8>().unwrap();
                Some((value as u64) << shift)
            } else {
                None
            }
        })
        .reduce(|a, b| a | b)
        .unwrap()
}

fn part1(input: &str) {
    let ParseResult {
        initial: mut values,
        gates: mut remaining_gates,
        ..
    } = parse(input);

    while !remaining_gates.is_empty() {
        remaining_gates.retain(|gate| !gate.try_run(&mut values));
    }

    println!("{}", form_output(&values));
}

/**

# Binary adders

A binary adder is made up of multiple adder cells, which add 2 bits together, taking into account a possible carry-in
and outputting a carry-out as well. These cells are connected together in series (carry-out from adder 0 to carry-in
of adder 1, etc.), and each take 1 bit as input from both numbers.

For example, if we want to calculate `a + b = c`, where each number has 4 bits:

```text
     a0,b0   a1,b1   a2,b2   a3,b3   a4,b4
       |       |       |       |       |
       v       v       v       v       v
0 -> ADD0 -> ADD1 -> ADD2 -> ADD3 -> ADD4 -> c5/overflow indicator
       |       |       |       |       |
       v       v       v       v       v
      c0      c1      c2      c3      c4
```

## Binary adder cell

In total, an adder cell has a 3-bit input (a, b, carry-in) and 2-bit output
(output, carry-out). All inputs/outputs have the same significance, except for the carry-out, which represents a two.
The adder basically does `(0|1) + (0|1) + (0|1) = 0-2`, outputting a 2-bit binary number for the result, composed of the
output (bit 0 - least significant bit) and carry-out (bit 1 - most significant bit).

It only needs to respect two simple rules:
- `output` (result bit 0) if `1` if either 1 or 3 inputs are `1`
- `carry-out` (result bit 1) if at least two inputs are `1`

This is achieved using logical gates like so:
- `output = (a ^ b) ^ carry-in`
- `carry-out = (a & b) | ((a ^ b) & carry-in)`

_`+` is a wire intersection where the two wires (`-`, `|`) do not connect_

```text
         A       B

         |       |
         |->AND<-|
         |   v   |
         |   \---+------\
         |       |      |
         \->XOR<-/      |
             |          |
             |---v      v
             |  AND--->OR
carry-in ----+---^      \--> carry-out
         |   v
         \->XOR
             |
             v

             C
```

## First & last bits

The first adder cell in the input/given adder is different, as the carry-in is missing.
The carry-out of the last adder is directly connected to the last bit of the output number, meaning
the output has 1 more bit than the inputs./

```text
         A       B

         |       |
         |->AND<-|
         |   v   |
         |   \---+--> carry-out
         |       |
         \->XOR<-/
             |
             v

             C
```

# Solution

The solution consists of figuring out how many bits (N) the input adder has, then creating a new N-bit adder by stacking
adder cells as described above. While creating this new adder, inputs/outputs of gates are compared with the given
adder to find.

This only outputs discrepancies (errors) and potential bad gates, final results need to be manually obtained by
inspecting the rendered graph. An edge-list rendering is also provided for pasting & viewing in an online graph viewer.

 */
fn part2(input: &str) {
    if is_example() {
        println!("This part can only run in full mode; not implemented for example");
        return;
    }

    let ParseResult { wires, gates, .. } = parse(input);

    println!("\nGraph for online viewing: https://graphonline.top/create_graph_by_edge_list");
    let mut gate_counters = [0u32; 3];
    for gate in &gates {
        let gate_ident = format!(
            "{}{}",
            gate.gate_type, gate_counters[gate.gate_type as usize]
        );
        gate_counters[gate.gate_type as usize] += 1;
        println!("{}-{gate_ident}", gate.input_a);
        println!("{}-{gate_ident}", gate.input_b);
        println!("{gate_ident}-{}", gate.output);
    }
    println!("\n");

    // Find number of output bits
    let output_bits: u8 = wires
        .iter()
        .filter_map(|&wire| {
            if wire.starts_with('z') {
                Some(wire[1..].parse::<u8>().unwrap())
            } else {
                None
            }
        })
        .max()
        .unwrap();

    // Adder/input bits
    let bits = output_bits - 1;

    macro_rules! wire {
        (x $num:expr) => {
            format!("x{:02}", $num)
        };
        (y $num:expr) => {
            format!("y{:02}", $num)
        };
        (z $num:expr) => {
            format!("z{:02}", $num)
        };
    }

    macro_rules! find_gate {
        // By output wire
        ($type:path, $wire:expr) => {
            gates
                .iter()
                .find(|gate| gate.output == $wire && gate.gate_type == $type)
        };
        // By input wires
        ($a:expr, $type:path, $b:expr) => {
            gates.iter().find(|gate| {
                ((gate.input_a == $a && gate.input_b == $b)
                    || gate.input_a == $b && gate.input_b == $a)
                    && gate.gate_type == $type
            })
        };
    }

    // Special treatment for first (incomplete) adder
    if let Some(first_xor) = find_gate!(wire!(x 0), GateType::XOR, wire!(y 0)) {
        if first_xor.output != "z00" {
            println!("expected first XOR gate to output to z00: [{first_xor}]");
        }
    } else {
        println!("could not find first XOR gate with output to z00")
    }

    println!("Errors:");
    let mut bad_gates: HashSet<Gate> = HashSet::new();
    let mut bad_carry_gate = false;
    let mut carry_gate = find_gate!(wire!(x 0), GateType::AND, wire!(y 0)).unwrap();
    for bit in 1..=bits {
        // check output - cannot fail as only outputs are swapped
        let input_xor = find_gate!(wire!(x bit), GateType::XOR, wire!(y bit)).unwrap();

        // check output
        if let Some(output_xor) = find_gate!(GateType::XOR, wire!(z bit)) {
            if input_xor.output != output_xor.input_a && input_xor.output != output_xor.input_b {
                bad_gates.insert(input_xor.clone());
                println!(
                    "expected partial input [{input_xor}] to connect to output XOR [{output_xor}]"
                )
            } else if bad_carry_gate {
                if let Some(new_carry_gate) = find_gate!(
                    GateType::OR,
                    if input_xor.output == output_xor.input_a {
                        output_xor.input_b
                    } else {
                        output_xor.input_a
                    }
                ) {
                    carry_gate = new_carry_gate;
                }
            }
            if carry_gate.output != output_xor.input_a && carry_gate.output != output_xor.input_b {
                bad_gates.insert(carry_gate.clone());
                println!(
                    "expected carry out [{carry_gate}] to connect to output XOR [{output_xor}]"
                )
            }
        } else {
            bad_gates.insert(
                gates
                    .iter()
                    .find(|gate| gate.output == wire!(z bit))
                    .unwrap()
                    .clone(),
            );
            println!("could not find output XOR gate with output z{bit:02}",);
        }

        // check carry-out
        let input_and = find_gate!(wire!(x bit), GateType::AND, wire!(y bit)).unwrap();
        if let Some(carry_and) = find_gate!(carry_gate.output, GateType::AND, input_xor.output) {
            if let Some(carry_or) = find_gate!(carry_and.output, GateType::OR, input_and.output) {
                carry_gate = carry_or;
                continue;
            } else {
                bad_gates.insert(carry_and.clone());
                bad_gates.insert(input_and.clone());
                println!(
                    "could not find carry OR gate with inputs {} and {}",
                    carry_and.output, input_and.output
                );
            }
        } else {
            bad_gates.insert(carry_gate.clone());
            bad_gates.insert(input_xor.clone());
            println!(
                "could not find carry AND gate with inputs {} and {}",
                carry_gate.output, input_xor.output
            );
        }
        bad_carry_gate = true;
    }

    println!("\nPossible problematic gates:");
    for gate in bad_gates {
        println!("*  {gate}");
    }
    println!();
}
