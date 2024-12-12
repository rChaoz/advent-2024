use super::DayInfo;

const EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

pub const INFO: DayInfo = DayInfo {
    name: "Garden Groups",
    part1,
    part2,
    example1: EXAMPLE,
    example2: EXAMPLE,
};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn dfs<F: FnMut(&mut Vec<Vec<bool>>, usize, usize)>(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
    callback: &mut F,
) -> (u32, u32) {
    if visited[i][j] {
        return (0, 0);
    }
    visited[i][j] = true;

    let mut perimeter = 4;
    let mut area = 1;

    callback(visited, i, j);

    for dir in 0..DIRECTIONS.len() {
        let i2 = i as i32 + DIRECTIONS[dir].0;
        let j2 = j as i32 + DIRECTIONS[dir].1;
        if i2 < 0 || j2 < 0 || i2 >= map.len() as i32 || j2 >= map[0].len() as i32 {
            continue;
        }
        let i2 = i2 as usize;
        let j2 = j2 as usize;
        if map[i][j] == map[i2][j2] {
            perimeter -= 1;
            let (pd, ad) = dfs(map, visited, i2, j2, callback);
            perimeter += pd;
            area += ad;
        }
    }

    (perimeter, area)
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Vec<bool>>) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let visited = vec![vec![false; map[0].len()]; map.len()];
    (map, visited)
}

fn base<F: FnMut(usize, usize) -> u32>(map: &Vec<Vec<char>>, mut f: F) {
    let price: u32 = (0..map.len())
        .map(|i| (0..map[0].len()).map(|j| f(i, j)).sum::<u32>())
        .sum();

    println!("{price}");
}

fn part1(input: &str) {
    let (map, mut visited) = parse(input);
    base(&map, |i, j| {
        let (perimeter, area) = dfs(&map, &mut visited, i, j, &mut |_, _, _| ());
        perimeter * area
    })
}

fn part2(input: &str) {
    let (map, mut visited) = parse(input);
    let mut sides = vec![vec![[false; DIRECTIONS.len()]; map[0].len()]; map.len()];
    base(&map, |i, j| {
        let mut sides_count = 0u32;
        let (_, area) = dfs(&map, &mut visited, i, j, &mut |visited, i, j| {
            for d in 0..DIRECTIONS.len() {
                let i_next = i as i32 + DIRECTIONS[d].0;
                let j_next = j as i32 + DIRECTIONS[d].1;
                if i_next >= 0
                    && j_next >= 0
                    && i_next < map.len() as i32
                    && j_next < map[0].len() as i32
                    && map[i][j] == map[i_next as usize][j_next as usize]
                {
                    continue;
                }

                sides[i][j][d] = true;
                sides_count += 1;
                // Check if there are adjacent plots of the same type (char) that were already visited,
                // then this side was already counted
                for (dx, dy) in [DIRECTIONS[(d + 1) % 4], DIRECTIONS[(d + 3) % 4]] {
                    let i_near = i as i32 + dx;
                    let j_near = j as i32 + dy;
                    if i_near < 0
                        || j_near < 0
                        || i_near >= map.len() as i32
                        || j_near >= map[0].len() as i32
                    {
                        continue;
                    }
                    let i2 = i_near as usize;
                    let j2 = j_near as usize;
                    if map[i][j] != map[i2][j2] || !visited[i2][j2] {
                        continue;
                    }
                    if sides[i2][j2][d] {
                        sides_count -= 1;
                    }
                }
            }
        });
        area * sides_count
    })
}
