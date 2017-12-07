//! Solution to [Day 7: Recursive Circus](http://adventofcode.com/2017/day/7) for Advent of Code
//! Part 2 not implemented

use utils::Part;
use std::collections::HashMap;
use std::collections::HashSet;

/// Runs this solution
pub fn run(part: Part, input: &String) -> Result<String, String> {
    let parsed = parse_input(input);

    if part == Part::One {
        find_root(&parsed).ok_or("Found no root node".to_string())
    } else {
        Err("Part two not supported".to_string())
    }
}

/// Finds the root of the given tree
fn find_root(parsed: &HashMap<String, Node>) -> Option<String> {
    let referenced_children = parsed.iter().flat_map(|e| e.1.children.iter()).collect::<HashSet<&String>>();
    let root = parsed.iter().filter(|e| !referenced_children.contains(e.0)).nth(0);
    root.map(|v| v.0.clone())
}

/// Parses all given nodes from an input
fn parse_input(input: &String) -> HashMap<String, Node> {
    let parsed_nodes: Vec<(String, u32, Vec<String>)> = input.split(";").map(|n| parse_node(&n.to_string())).collect();
    let mut nodes = HashMap::new();
    for parsed_node in parsed_nodes {
        nodes.insert(parsed_node.0.clone(), Node {
            name: parsed_node.0,
            weight: parsed_node.1,
            children: parsed_node.2,
        });
    }
    nodes
}

/// Parses a single node from the given string
fn parse_node(input: &String) -> (String, u32, Vec<String>) {
    let name = input.split_whitespace().nth(0).unwrap_or("").to_string();
    let bracket_start = input.find("(").unwrap_or(0) + 1;
    let bracket_end = input.find(")").unwrap_or(0);
    let weight = input[bracket_start..bracket_end].parse::<u32>().unwrap_or(0);
    let children_string = input.find(">").map(|i| input.split_at(i + 1).1.to_string());
    let children = children_string.map(|s| parse_children(&s));

    (name, weight, children.unwrap_or(vec!()))
}

/// Parses a vec of children names from the given string
fn parse_children(input: &String) -> Vec<String> {
    input.split(",").map(|c| c.trim().to_string()).collect::<Vec<String>>()
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl Node {
    /// Gets the total weight of this node
    fn total_weight(&self, nodes: &HashMap<String, Node>) -> u32 {
        let mut weight = self.weight;
        for child in self.children.iter() {
            weight += nodes.get(&child.clone()).map(|v| v.total_weight(nodes)).unwrap_or(0);
        }
        weight
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    #[test]
    fn test_given_1() {
        let input = "pbga (66); xhth (57); ebii (61); havc (66); ktlj (57); fwft (72) -> ktlj, cntj, xhth; qoyq (66); padx (45) -> pbga, havc, qoyq; tknk (41) -> ugml, padx, fwft; jptl (61); ugml (68) -> gyxo, ebii, jptl; gyxo (61); cntj (57)";
        assert_eq!(run(Part::One, &input.to_string()), Ok("tknk".to_string()))
    }

    #[test]
    fn test_parse_input() {
        let input = "ktlj (57); fwft (72) -> ktlj, cntj, xhth";
        let mut output = HashMap::new();
        output.insert("ktlj".to_string(), Node {
            name: "ktlj".to_string(),
            weight: 57,
            children: vec!(),
        });
        output.insert("fwft".to_string(), Node {
            name: "fwft".to_string(),
            weight: 72,
            children: vec!("ktlj".to_string(), "cntj".to_string(), "xhth".to_string()),
        });
        assert_eq!(parse_input(&input.to_string()), output);
    }

    #[test]
    fn test_parse_node() {
        let input = "fwft (72) -> ktlj, cntj, xhth";
        assert_eq!(parse_node(&input.to_string()), ("fwft".to_string(), 72, vec!("ktlj".to_string(), "cntj".to_string(), "xhth".to_string())));
    }

    #[test]
    fn test_parse_children() {
        let input = "ktlj, cntj, xhth";
        assert_eq!(parse_children(&input.to_string()), vec!("ktlj".to_string(), "cntj".to_string(), "xhth".to_string()))
    }
}
