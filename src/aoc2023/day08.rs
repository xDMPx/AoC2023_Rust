use std::collections::HashMap;

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part01(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut documents = file.lines().filter(|line| !line.is_empty());

    let mut instructions = documents.next().unwrap().chars().cycle();

    let mut nodes = HashMap::new();
    let node_input = documents.map(|line| {
        let mut node_info = line.split('=');
        let name = node_info.next().unwrap().trim().to_owned();
        let mut direction = node_info.next().unwrap().trim().split(',');
        let left = direction.next().unwrap().trim().trim_start_matches('(');
        let right = direction.next().unwrap().trim().trim_end_matches(')');

        (name, Node { left, right })
    });

    node_input.for_each(|(name, node)| {
        nodes.insert(name, node);
    });

    let mut current_node = "AAA";
    let mut steps = 0;
    while current_node != "ZZZ" {
        let direction = instructions.next().unwrap();
        match direction {
            'L' => current_node = &nodes.get(current_node).unwrap().left,
            'R' => current_node = &nodes.get(current_node).unwrap().right,
            _ => unreachable!(),
        }
        steps += 1;
    }

    steps
}

pub fn part02(file_path: &str) -> usize {
    let file = std::fs::read_to_string(file_path).unwrap();
    let mut documents = file.lines().filter(|line| !line.is_empty());

    let instructions_str = documents.next().unwrap();

    let mut nodes = HashMap::new();
    let node_input = documents.map(|line| {
        let mut node_info = line.split('=');
        let name = node_info.next().unwrap().trim().to_owned();
        let mut direction = node_info.next().unwrap().trim().split(',');
        let left = direction.next().unwrap().trim().trim_start_matches('(');
        let right = direction.next().unwrap().trim().trim_end_matches(')');

        (name, Node { left, right })
    });

    let mut starting_nodes = Vec::new();
    node_input.for_each(|(name, node)| {
        if is_start_node(&name) {
            starting_nodes.push(name.clone());
        }
        nodes.insert(name, node);
    });

    let min_steps: Vec<usize> = starting_nodes
        .iter()
        .map(|node| calculate_min_steps(&node, instructions_str, &nodes))
        .collect();

    lcm(min_steps)
}

fn get_next_node<'a>(
    current_node: &str,
    direction: char,
    nodes: &HashMap<String, Node<'a>>,
) -> &'a str {
    match direction {
        'L' => &nodes.get(current_node).unwrap().left,
        'R' => &nodes.get(current_node).unwrap().right,
        _ => unreachable!(),
    }
}

fn is_start_node(node: &str) -> bool {
    node.ends_with('A')
}

fn is_end_node(node: &str) -> bool {
    node.ends_with('Z')
}

fn calculate_min_steps<'a>(
    current_node: &str,
    instructions_str: &str,
    nodes: &HashMap<String, Node<'a>>,
) -> usize {
    let mut current_node = current_node;
    let mut instructions = instructions_str.chars().cycle();
    let mut steps = 0;
    while !is_end_node(current_node) {
        let direction = instructions.next().unwrap();
        current_node = get_next_node(current_node, direction, nodes);
        steps += 1;
    }

    steps
}

// https://en.wikipedia.org/wiki/Least_common_multiple#Using_prime_factorization
fn lcm(numbers: Vec<usize>) -> usize {
    let factors = numbers.iter().map(|s| factorize(*s));
    let mut lcm_factors: HashMap<usize, usize> = HashMap::new();
    for (prime, atom_count) in factors.flatten() {
        lcm_factors
            .entry(prime)
            .and_modify(|e| *e = (*e).max(atom_count))
            .or_insert(1);
    }

    let lcm = lcm_factors.iter().fold(1, |acc, (prime, atom_count)| {
        acc * prime.pow((*atom_count).try_into().unwrap())
    });

    lcm
}

fn factorize(num: usize) -> Vec<(usize, usize)> {
    let mut n = num;
    let mut factors = Vec::new();
    let mut x = 0;
    while n % 2 == 0 {
        x += 1;
        n /= 2;
    }
    if x != 0 {
        factors.push((2, x));
    }

    let mut i = 3;
    while n != 1 {
        let mut x = 0;
        while n % i == 0 {
            x += 1;
            n /= i;
        }
        if x != 0 {
            factors.push((i, x));
        }
        i += 1;
    }

    factors
}
