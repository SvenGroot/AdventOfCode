use std::{fmt::Display, path::Path, str::FromStr};

use aoc::{aoc_input, get_input, tree::TreeNode};

fn main() {
    println!("Part 1: {}", part1(aoc_input()));
    println!("Part 2: {}", part2(aoc_input()));
}

fn part1(path: impl AsRef<Path>) -> usize {
    let mut tree = parse_input(path);
    println!("{}", tree);
    calc_dir_sizes(&mut tree);
    let mut sum = 0;
    tree.walk(&mut |f| {
        if f.is_dir && f.size < 100000 {
            sum += f.size
        }
    });

    sum
}

fn part2(path: impl AsRef<Path>) -> usize {
    let mut tree = parse_input(path);
    calc_dir_sizes(&mut tree);
    const TOTAL_SIZE: usize = 70000000;
    const NEEDED_SPACE: usize = 30000000;

    let free_space = TOTAL_SIZE - tree.content().size;
    let additional_needed = NEEDED_SPACE - free_space;
    let mut min = usize::MAX;
    tree.walk(&mut |f| {
        if f.is_dir && f.size > additional_needed && f.size < min {
            min = f.size;
        }
    });

    min
}

struct FileSystemItem {
    is_dir: bool,
    name: String,
    size: usize,
}

impl FileSystemItem {
    fn dir(name: String) -> Self {
        Self {
            is_dir: true,
            name,
            size: 0,
        }
    }

    fn file(name: String, size: usize) -> Self {
        Self {
            is_dir: false,
            name,
            size,
        }
    }
}

impl Display for FileSystemItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_dir {
            write!(f, "{} (dir)", self.name)?;
        } else {
            write!(f, "{} (file, size={})", self.name, self.size)?;
        }

        Ok(())
    }
}

fn parse_input(path: impl AsRef<Path>) -> TreeNode<FileSystemItem> {
    let mut root = TreeNode::new(FileSystemItem::dir("/".into()));
    let mut iter = get_input(path);
    parse_node(&mut root, &mut iter);

    root
}

fn parse_node(
    node: &mut TreeNode<FileSystemItem>,
    iter: &mut impl Iterator<Item = String>,
) -> Option<()> {
    loop {
        let line = iter.next()?;
        if let Some(dir) = line.strip_prefix("$ cd ") {
            // "/" only occurs at the start of the input so we don;t actually need to handle it.
            if dir == ".." {
                break;
            } else if dir != "/" {
                let child = node.find_mut(|item| item.name == dir).unwrap();
                parse_node(child, iter)?;
            }
        } else if line != "$ ls" {
            let (size, name) = line.split_once(' ').unwrap();
            if size == "dir" {
                node.add_child(FileSystemItem::dir(name.into()));
            } else {
                let size = usize::from_str(size).unwrap();
                node.add_child(FileSystemItem::file(name.into(), size));
            }
        }
    }

    Some(())
}

fn calc_dir_sizes(node: &mut TreeNode<FileSystemItem>) -> usize {
    if node.content().is_dir {
        node.content_mut().size = node.children_mut().map(calc_dir_sizes).sum();
    }

    node.content().size
}

#[cfg(test)]
mod tests {
    use aoc::aoc_sample_input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(95437, part1(aoc_sample_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(24933642, part2(aoc_sample_input()));
    }
}
