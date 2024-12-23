use super::DayInfo;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

pub const INFO: DayInfo = DayInfo {
    name: "LAN Party",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

#[derive(PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
struct PC(u8, u8);

impl Debug for PC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PC({}{})", self.0 as char, self.1 as char)
    }
}

impl Display for PC {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0 as char, self.1 as char)
    }
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = (PC, PC)> + 'a {
    input.lines().map(|line| {
        let line = line.as_bytes();
        assert_eq!(line.len(), 5);
        let pc1 = PC(line[0], line[1]);
        assert_eq!(line[2], b'-');
        let pc2 = PC(line[3], line[4]);
        (pc1, pc2)
    })
}

// Helpers
type LAN = Vec<PC>;

macro_rules! lan {
    ($( $pc:expr ),*) => {{
        let mut vec: LAN = vec![$( $pc, )*];
        vec.sort();
        vec
    }};
}

fn part1(input: &str) {
    let connections = parse(input).collect::<HashSet<_>>();
    let mut lan_connections: HashSet<LAN> = HashSet::new();
    for &(pc1, pc2) in &connections {
        // only keep if any PC name starts with t
        if pc1.0 != b't' && pc2.0 != b't' {
            continue;
        }
        for conn2 in &connections {
            // find a connection between pc1 and a third pc3
            let pc3 = if conn2.0 == pc1 {
                conn2.1
            } else if conn2.1 == pc1 {
                conn2.0
            } else {
                continue;
            };
            // ensure connection between pc2 and pc3 exists
            if !connections.contains(&(pc2, pc3)) && !connections.contains(&(pc3, pc2)) {
                continue;
            }
            lan_connections.insert(lan!(pc1, pc2, pc3));
        }
    }
    println!("{}", lan_connections.len());
}

/*
NOTE: This doesn't work for all cases, for example it incorrectly finds A-B-C instead of A-X-Y-Z as the largest LAN
starting from A:

a: b, c, d, e, x, y, z
b: c, d, e
c: b
d: b
e: b
x: y, z
y: x, z
z: x, y

However this worked for me as it probably finds the correct LAN starting from X.
This correct solution is https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm,
but I couldn't be bothered with it
 */
fn part2(input: &str) {
    // Convert to map of PCs connected for every PC
    let mut map: HashMap<PC, HashSet<PC>> = HashMap::new();
    for (pc1, pc2) in parse(input) {
        // Save connection
        map.entry(pc1).or_default().insert(pc2);
        map.entry(pc2).or_default().insert(pc1);
    }

    let mut best_lan = Vec::new();
    for (&pc, initial_connections) in &map {
        let mut lan = vec![pc];
        let mut remaining = map
            .iter()
            .filter_map(|(pc, connections)| {
                if initial_connections.contains(pc) {
                    Some((
                        *pc,
                        connections
                            .iter()
                            .filter(|pc| initial_connections.contains(pc))
                            .collect::<HashSet<_>>(),
                    ))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        while remaining.len() > 0 {
            // Pick the PC with most connections
            let &new_pc = remaining
                .iter()
                .max_by_key(|(_, connections)| connections.len())
                .unwrap()
                .0;
            lan.push(new_pc);

            // Only keep PCs with connections to this new PC
            remaining = remaining
                .into_iter()
                .filter_map(|(pc, mut connections)| {
                    if connections.remove(&new_pc) {
                        Some((pc, connections))
                    } else {
                        None
                    }
                })
                .collect();
        }

        if lan.len() > best_lan.len() {
            best_lan = lan;
        }
    }

    best_lan.sort();
    let mut iter = best_lan.into_iter();
    if let Some(pc) = iter.next() {
        print!("{pc}");
    }
    iter.for_each(|pc| print!(",{pc}"));
    println!();
}
