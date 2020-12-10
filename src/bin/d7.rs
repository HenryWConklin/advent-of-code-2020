use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{BufRead, stdin};

use regex::Regex;

fn inner_count<T>(start: &T, graph: &HashMap<T, Vec<(T, usize)>>) -> usize
    where
        T: Eq + Hash
{
    fn inner<'a, T>(curr: &'a T, graph: &'a HashMap<T, Vec<(T, usize)>>, mem: &mut HashMap<&'a T, usize>) -> usize
        where
            T: Eq + Hash
    {
        match mem.get(curr) {
            Some(x) => *x,
            None => {
                let mut tot_inner = 0;
                match graph.get(curr) {
                    Some(v) => {
                        for (adj, count) in v {
                            tot_inner += count * (inner(adj, graph, mem) + 1);
                        }
                    }
                    None => {}
                }
                mem.insert(curr, tot_inner);
                tot_inner
            }
        }
    }
    let mut mem = HashMap::new();
    inner(start, graph, &mut mem)
}

fn main() {
    let mut rev_graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut graph: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let line_regex =
        Regex::new(r"^(.*) bags contain ((?:\d+ .* bags?(?:, )?)+|no other bags).$").unwrap();
    let inner_regex = Regex::new(r"(\d+) ([\w ]+) bags?").unwrap();
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let m = line_regex.captures(&line).unwrap();
        let outer = m.get(1).unwrap().as_str();
        let inner = m.get(2).unwrap().as_str();

        for b in inner_regex.captures_iter(inner) {
            let count: usize = b.get(1).unwrap().as_str().parse().unwrap();
            let name = b.get(2).unwrap().as_str();

            match graph.get_mut(outer) {
                Some(v) => v.push((name.to_owned(), count)),
                None => {
                    graph.insert(outer.to_owned(), vec![(name.to_owned(), count)]);
                }
            }

            match rev_graph.get_mut(name) {
                Some(v) => v.push(outer.to_owned()),
                None => {
                    rev_graph.insert(name.to_owned(), vec![outer.to_owned()]);
                }
            }
        }
    }

    let init_str = "shiny gold".to_string();
    let mut visited = HashSet::new();
    let mut stack = vec![&init_str];
    while let Some(curr) = stack.pop() {
        match rev_graph.get(curr) {
            Some(v) => {
                for adj in v {
                    if visited.insert(adj) {
                        stack.push(adj)
                    }
                }
            }
            None => {}
        }
    }
    println!("{}", visited.len());

    println!("{}", inner_count(&init_str, &graph))
}
