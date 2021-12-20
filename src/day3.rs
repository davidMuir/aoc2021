#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

type NodeId = usize;

#[derive(Debug)]
struct Node {
    value: Option<char>,
    left: Option<NodeId>,
    right: Option<NodeId>,
}

impl<'a> Tree {
    fn new(lines: Vec<String>) -> Self {
        let mut tree = Self { nodes: Vec::new() };

        let root = Node {
            value: None,
            left: None,
            right: None,
        };

        tree.nodes.push(root);

        for line in lines {
            tree.add(line);
        }

        tree
    }

    fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    fn get_count(&self, id: Option<NodeId>) -> usize {
        if let Some(id) = id {
            if let Some(n) = self.get_node(id) {
                1 + self.get_count(n.left) + self.get_count(n.right)
            } else {
                0
            }
        } else {
            0
        }
    }

    fn get_most_common(&self) -> Option<String> {
        let mut current_id = Some(0);
        let mut s = String::new();

        while let Some(id) = current_id {
            let n = self.get_node(id)?;

            if let Some(c) = n.value {
                s.push(c);
            }

            let count_left = self.get_count(n.left);
            let count_right = self.get_count(n.right);

            current_id = match (count_left, count_right) {
                (0, 0) => None,
                (0, _) => n.right,
                (_, 0) => n.left,
                (l, r) if l == r => n.right,
                (l, r) if l > r => n.left,
                (l, r) if l < r => n.right,
                _ => None,
            };
        }

        Some(s)
    }

    fn get_least_common(&self) -> Option<String> {
        let mut current_id = Some(0);
        let mut s = String::new();

        while let Some(id) = current_id {
            let n = self.get_node(id)?;

            if let Some(c) = n.value {
                s.push(c);
            }

            let count_left = self.get_count(n.left);
            let count_right = self.get_count(n.right);

            current_id = match (count_left, count_right) {
                (0, 0) => None,
                (0, _) => n.right,
                (_, 0) => n.left,
                (l, r) if l == r => n.left,
                (l, r) if l > r => n.right,
                (l, r) if l < r => n.left,
                _ => None,
            };
        }

        Some(s)
    }

    fn add(&mut self, line: String) {
        let mut current_index = 0;

        for c in line.chars() {
            let left_id: Option<usize>;
            let right_id: Option<usize>;

            {
                let current_node = self.get_node(current_index).unwrap();
                left_id = current_node.left;
                right_id = current_node.right;
            }

            if c == '1' {
                if let Some(n) = right_id {
                    current_index = n;
                } else {
                    let next_index = self.push_node(Node {
                        value: Some(c),
                        left: None,
                        right: None,
                    });

                    {
                        let current_node = self.nodes.get_mut(current_index).unwrap();

                        current_node.right = Some(next_index);
                    }
                    current_index = next_index;
                }
            } else if let Some(n) = left_id {
                current_index = n;
            } else {
                let next_index = self.push_node(Node {
                    value: Some(c),
                    left: None,
                    right: None,
                });

                {
                    let current_node = self.nodes.get_mut(current_index).unwrap();

                    current_node.left = Some(next_index);
                }
                current_index = next_index;
            }
        }
    }

    fn push_node(&mut self, node: Node) -> NodeId {
        let next_id = self.nodes.len();

        self.nodes.push(node);

        next_id
    }
}

impl Node {}

fn get_oxygen_rating(tree: &Tree) -> Option<u32> {
    let most_common = tree.get_most_common()?;

    Some(u32::from_str_radix(&*most_common, 2).unwrap())
}

fn get_co2_scrubber_rating(tree: &Tree) -> Option<u32> {
    let least_common = tree.get_least_common()?;

    println!("{}", least_common);

    Some(u32::from_str_radix(&*least_common, 2).unwrap())
}

pub fn solve_day_3(lines: impl Iterator<Item = Result<String, std::io::Error>>) -> Option<u32> {
    let lines = lines.flatten().collect::<Vec<String>>();

    let tree = Tree::new(lines);

    let oxygen = get_oxygen_rating(&tree).unwrap();
    let scrubber = get_co2_scrubber_rating(&tree).unwrap();

    Some(oxygen * scrubber)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let lines = vec![
            Ok("00100".to_string()),
            Ok("11110".to_string()),
            Ok("10110".to_string()),
            Ok("10111".to_string()),
            Ok("10101".to_string()),
            Ok("01111".to_string()),
            Ok("00111".to_string()),
            Ok("11100".to_string()),
            Ok("10000".to_string()),
            Ok("11001".to_string()),
            Ok("00010".to_string()),
            Ok("01010".to_string()),
        ]
        .into_iter();

        let expected = 230;
        let result = solve_day_3(lines).unwrap();

        assert_eq!(expected, result);
    }
}
