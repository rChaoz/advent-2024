mod util;

pub struct DayInfo {
    pub name: &'static str,
    pub part1: fn(&str) -> (),
    pub part2: fn(&str) -> (),
    pub example1: &'static str,
    pub example2: &'static str,
}

macro_rules! days {
    ($num:literal) => {
        use seq_macro::seq;

        seq!(N in 1..=$num {
            pub const DAYS: [DayInfo; $num] = [
                #(
                    day_~N::INFO,
                )*
            ];
        });

        seq!(N in 1..=$num {
            mod day_~N;
        });
    };
}

days!(23);
