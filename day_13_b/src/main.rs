use std::cmp::Ordering;
use std::{error::Error, fs};

#[derive(Debug, Clone)]
enum Node {
    List(Vec<Node>),
    Value(i32),
}

fn parse(line: &[char]) -> (Node, usize) {
    if line[0] != '[' {
        let val = if line[0] == 'a' {
            10
        } else {
            line[0].to_string().parse::<i32>().unwrap()
        };
        return (Node::Value(val), 1);
    }

    if line[1] == ']' {
        return (Node::List(Vec::new()), 2);
    }

    let mut pos = 1;

    let mut sub_nodes = Vec::new();

    loop {
        let sub_node = parse(&line[pos..]);

        sub_nodes.push(sub_node.0);
        pos += sub_node.1;

        if line[pos] == ',' {
            pos += 1;
            continue;
        }
        assert!(line[pos] == ']');
        break;
    }

    (Node::List(sub_nodes), pos + 1)
}

fn compare(left: &Node, right: &Node) -> Ordering {
    match left {
        Node::List(left_list) => match right {
            Node::List(right_list) => {
                for i in 0.. {
                    if i < left_list.len() && i >= right_list.len() {
                        return Ordering::Greater;
                    }
                    if i >= left_list.len() && i < right_list.len() {
                        return Ordering::Less;
                    }
                    if i >= left_list.len() && i >= right_list.len() {
                        return Ordering::Equal;
                    }
                    let sub_comp = compare(&left_list[i], &right_list[i]);
                    if matches!(sub_comp, Ordering::Equal) {
                        continue;
                    }
                    return sub_comp;
                }
                panic!("?");
            }
            &Node::Value(right_val) => compare(left, &Node::List(vec![Node::Value(right_val)])),
        },
        &Node::Value(left_val) => match right {
            Node::List(_list) => compare(&Node::List(vec![Node::Value(left_val)]), right),
            &Node::Value(right_val) => match (left_val, right_val) {
                (left_val, right_val) if left_val > right_val => Ordering::Greater,
                (left_val, right_val) if right_val > left_val => Ordering::Less,
                _ => Ordering::Equal,
            },
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?.replace("10", "a");

    let pairs = input.split("\n\n");

    let mut list = Vec::new();

    for (left, right) in pairs.map(|s| s.split_once('\n').unwrap()) {
        let (left, _) = parse(&left.chars().collect::<Vec<char>>());
        let (right, _) = parse(&right.chars().collect::<Vec<char>>());
        list.push(left);
        list.push(right);
    }

    let list_len = list.len();
    let divider_1 = list[list_len - 2].clone();
    let divider_2 = list[list_len - 1].clone();

    list.sort_by(compare);

    let divider_1 = list
        .iter()
        .position(|n| matches!(compare(n, &divider_1), Ordering::Equal))
        .unwrap()
        + 1;
    let divider_2 = list
        .iter()
        .position(|n| matches!(compare(n, &divider_2), Ordering::Equal))
        .unwrap()
        + 1;

    println!("{}", divider_1 * divider_2);

    Ok(())
}
