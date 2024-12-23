use super::Day;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day23 {
    input: String,
}

impl Day23 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Day for Day23 {
    fn part1(&self) -> String {
        let mut peers = HashMap::new();
        for line in self.input.lines() {
            let (first, second) = line.split_once("-").unwrap();
            peers.entry(first).or_insert_with(Vec::new).push(second);
            peers.entry(second).or_insert_with(Vec::new).push(first);
        }
        // find sets of 3 computers where each computer is connected to the other two

        let mut triples = vec![];
        for (node, node_peers) in &peers {
            for peer in node_peers {
                let mut found = vec![];
                for inner in node_peers {
                    if peers[peer].contains(inner) {
                        found.push(inner);
                    }
                }
                for f in found {
                    if !triples.contains(&Triple {
                        first: node.to_string(),
                        second: peer.to_string(),
                        third: f.to_string(),
                    }) {
                        triples.push(Triple {
                            first: node.to_string(),
                            second: peer.to_string(),
                            third: f.to_string(),
                        });
                    }
                }
            }
        }
        let mut count_t = 0;
        for triple in triples.clone() {
            if triple.first.starts_with("t") {
                count_t += 1;
                continue;
            }
            if triple.second.starts_with("t") {
                count_t += 1;
                continue;
            }
            if triple.third.starts_with("t") {
                count_t += 1;
                continue;
            }
        }
        count_t.to_string()
    }

    fn part2(&self) -> String {
        let mut peers = HashMap::new();
        for line in self.input.lines() {
            let (first, second) = line.split_once("-").unwrap();
            peers
                .entry(first.to_string())
                .or_insert_with(Vec::new)
                .push(second.to_string());
            peers
                .entry(second.to_string())
                .or_insert_with(Vec::new)
                .push(first.to_string());
        }

        let mut cliques: Vec<HashSet<String>> = vec![];
        bron_kerbosch(
            HashSet::new(),
            peers.keys().cloned().collect(),
            HashSet::new(),
            &peers,
            &mut cliques,
        );
        // get all cliques, not just one
        while cliques[cliques.len() - 1].len() < 13 {
            bron_kerbosch(
                HashSet::new(),
                peers.keys().cloned().collect(),
                HashSet::new(),
                &peers,
                &mut cliques,
            );
        }

        // sort and join with commas
        let mut sorted: Vec<_> =
            <HashSet<std::string::String> as Clone>::clone(&cliques[cliques.len() - 1])
                .into_iter()
                .collect();
        sorted.sort();
        sorted.join(",")
    }
}

fn bron_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    mut x: HashSet<String>,
    peers: &HashMap<String, Vec<String>>,
    mut cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }
    let mut p = p.clone();
    while !p.is_empty() {
        let v = p.iter().next().unwrap().to_string();
        let mut r = r.clone();
        r.insert(v.clone());
        p = p
            .iter()
            .filter(|x| peers[&v].contains(x))
            .cloned()
            .collect();
        x = x
            .iter()
            .filter(|x| peers[&v].contains(x))
            .cloned()
            .collect();
        bron_kerbosch(r, p.clone(), x.clone(), peers, &mut cliques);
        p.remove(&v);
        x.insert(v);
    }
}

#[derive(Hash, Debug, Clone)]
struct Triple {
    first: String,
    second: String,
    third: String,
}

impl Eq for Triple {}
impl PartialEq for Triple {
    fn eq(&self, other: &Self) -> bool {
        if self.first == other.first {
            if self.second == other.second {
                self.third == other.third
            } else if self.second == other.third {
                self.third == other.second
            } else {
                false
            }
        } else if self.first == other.second {
            if self.second == other.first {
                self.third == other.third
            } else if self.second == other.third {
                self.third == other.first
            } else {
                false
            }
        } else if self.first == other.third {
            if self.second == other.first {
                self.third == other.second
            } else if self.second == other.second {
                self.third == other.first
            } else {
                false
            }
        } else {
            false
        }
    }
}
