use super::DayInfo;
use std::iter::repeat;

pub const INFO: DayInfo = DayInfo {
    name: "Disk Fragmenter",
    part1,
    part2,
    example1: "2333133121414131402",
    example2: "2333133121414131402",
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Block {
    File(u32),
    Free,
}

fn part1(input: &str) {
    let mut disk = parse(input);
    let mut left = 0;
    let mut right = disk.len() - 1;
    'outer: loop {
        while disk[left] != Block::Free {
            left += 1;
            if left >= right {
                break 'outer;
            }
        }
        while disk[right] == Block::Free {
            right -= 1;
            if left >= right {
                break 'outer;
            }
        }
        disk.swap(left, right);
    }
    println!("{}", checksum(&disk));
}

fn part2(input: &str) {
    let mut disk = parse(input);
    // left-most empty space
    let mut left = 0;
    while disk[left] != Block::Free {
        left += 1;
    }
    // right-most unmoved block
    let mut right = disk.len() - 1;
    // loop over files
    while left < right {
        // Find the next file to be moved
        let mut right_start = right;
        while disk[right_start - 1] == disk[right] {
            right_start -= 1;
        }
        // Find an empty spot of the same size
        let mut left_start = left;
        let mut left_end = left;
        let can_move = loop {
            if left_end - left_start == right - right_start {
                break true;
            }
            left_end += 1;
            if disk[left_end] != Block::Free {
                if left_end >= right_start {
                    break false;
                }
                while disk[left_end] != Block::Free {
                    left_end += 1;
                }
                if left_end >= right_start {
                    break false;
                }
                left_start = left_end;
            }
        };
        // Move the file
        if can_move {
            let (s1, s2) = disk.split_at_mut(right_start);
            s1[left_start..=left_end].swap_with_slice(&mut s2[0..=right - right_start]);
            // Find the next empty space
            if left == left_start {
                left = left_end + 1;
            }
            while disk[left] != Block::Free {
                left += 1;
            }
        }
        // Find the next file
        right = right_start - 1;
        while disk[right] == Block::Free {
            right -= 1;
        }
    }
    println!("{}", checksum(&disk));
}

fn parse(input: &str) -> Vec<Block> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(index, len)| {
            repeat(if index % 2 != 0 {
                Block::Free
            } else {
                Block::File(index as u32 / 2)
            })
            .take(len as usize)
        })
        .collect()
}

fn checksum(disk: &[Block]) -> u64 {
    disk.iter()
        .enumerate()
        .map(|(index, &file)| {
            if let Block::File(id) = file {
                index as u64 * id as u64
            } else {
                0
            }
        })
        .sum()
}
