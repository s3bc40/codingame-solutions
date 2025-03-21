use std::io;

use itertools::Itertools;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// NEEDS
// 1. Get all vertices possible
// 2. Get all exits possible
// 3. Position of Bobnet
// 4. Determine the closest gateway and severe connection if it exist with agent 

struct Link {
    n1: i32,
    n2: i32,
}

impl Link {
    fn new(n1: i32, n2: i32) -> Self {
        Self{ n1, n2 }
    }

    fn severe(&self) {
        println!("{} {}", self.n1, self.n2);
    }

    fn has_node(&self, &node: &i32) -> bool {
        self.n1 == node || self.n2 == node
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = parse_input!(inputs[0], i32); // the total number of nodes in the level, including the gateways
    let l = parse_input!(inputs[1], i32); // the number of links
    let e = parse_input!(inputs[2], i32); // the number of exit gateways

    // Get possible links and gateway
    let mut links = Vec::new();
    let mut gateways = Vec::new();

    for i in 0..l as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n1 = parse_input!(inputs[0], i32); // N1 and N2 defines a link between these nodes
        let n2 = parse_input!(inputs[1], i32);
        links.push(Link::new(n1, n2));
    }
    for i in 0..e as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let ei = parse_input!(input_line, i32); // the index of a gateway node
        gateways.push(ei);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let si = parse_input!(input_line, i32); // The index of the node on which the Bobnet agent is positioned this turn

        let link_to_severe = links
            .iter()
            .filter(|link| link.has_node(&si)).find_or_first(|link| {
                for gateway in &gateways {
                    if link.has_node(&gateway) {
                        return true;
                    }
                }
                false
            }).unwrap();

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // Example: 0 1 are the indices of the nodes you wish to sever the link between
        link_to_severe.severe();
    }
}
