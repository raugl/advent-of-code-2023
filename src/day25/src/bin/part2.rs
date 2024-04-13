use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};

// Stoer Wagner algorithm implemented on an adjacency hash map using string as keys

fn process(input: &str) -> i64 {
    let graph = parse_graph(input);
    let graph_len = graph.len();
    let (_min_cut, partition_len) = stoer_wagner(graph);
    ((graph_len - partition_len) * partition_len) as i64
}

type Graph<'a> = HashMap<&'a str, BTreeMap<&'a str, i32>>;

fn parse_graph<'a>(input: &'a str) -> Graph<'a> {
    let mut graph = Graph::default();

    for line in input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
    {
        let (node, neighbours) = line.split_once(':').unwrap();
        for next in neighbours.split_whitespace() {
            graph.entry(node).or_default().insert(next, 1);
            graph.entry(next).or_default().insert(node, 1);
        }
    }
    graph
}

fn stoer_wagner<'a>(mut graph: Graph<'a>) -> (i32, usize) {
    let cuts: Vec<(&str, &str, i32)> = (0..graph.len() - 1)
        .map(|_| {
            let (prev, last, cut) = min_cut_phase(&graph);
            let neighbours = graph.remove(last).unwrap();

            for (next, weight) in neighbours {
                if next != prev {
                    *graph.get_mut(next).unwrap().entry(prev).or_insert(0) += weight;
                    *graph.get_mut(prev).unwrap().entry(next).or_insert(0) += weight;
                }
                graph.get_mut(next).unwrap().remove(last);
            }
            (prev, last, cut)
        })
        .collect();

    let (best_phase, (_prev, last, min_cut)) = cuts
        .iter()
        .cloned()
        .enumerate()
        .min_by_key(|(_i, (_prev, _last, cut))| *cut)
        .unwrap();

    let mut graph = HashMap::new();
    for (prev, last, _cut) in cuts.into_iter().take(best_phase) {
        graph.entry(prev).or_insert(BTreeSet::new()).insert(last);
        graph.entry(last).or_insert(BTreeSet::new()).insert(prev);
    }

    let mut stack = vec![last];
    let mut visited = HashSet::new();

    while let Some(node) = stack.pop() {
        if !visited.contains(node) {
            visited.insert(node);

            if let Some(neighbours) = graph.get(node) {
                for next in neighbours {
                    stack.push(next);
                }
            }
        }
    }
    (min_cut, visited.len())
}

fn min_cut_phase<'a>(graph: &Graph<'a>) -> (&'a str, &'a str, i32) {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::from([(0, *graph.keys().next().unwrap())]);
    let mut weights: HashMap<&str, i32> = graph.keys().map(|node| (*node, 0)).collect();

    let mut prev = "-";
    let mut last = "-";
    let mut cut = 0;

    while let Some((weight, node)) = queue.pop() {
        if !visited.contains(node) {
            visited.insert(node);
            prev = last;
            last = node;
            cut = weight;

            for (next, weight) in graph.get(node).unwrap() {
                if !visited.contains(next) {
                    let old_weight = weights.entry(next).or_insert(0);
                    *old_weight += weight;
                    queue.push((*old_weight, next));
                }
            }
        }
    }
    (prev, last, cut)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr
        ";
        assert_eq!(process(input), 54);
    }
}
