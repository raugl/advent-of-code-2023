use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::mem;

// Stoer Wagner algorithm implemented on an adjacency array using indices as keys

fn process(input: &str) -> i64 {
    let graph = parse_graph(input);
    let graph_len = graph.len();
    let partition_len = stoer_wagner(graph);
    ((graph_len - partition_len) * partition_len) as i64
}

type Graph = Vec<BTreeMap<usize, i32>>;

fn parse_graph(input: &str) -> Graph {
    let mut graph = HashMap::new();
    for line in input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
    {
        let (node, neighbours) = line.split_once(':').unwrap();
        for next in neighbours.split_whitespace() {
            graph.entry(node).or_insert(BTreeSet::new()).insert(next);
            graph.entry(next).or_insert(BTreeSet::new()).insert(node);
        }
    }

    let mut nodes = vec![BTreeMap::new(); graph.len()];
    let str_to_idx: HashMap<&str, usize> = graph.keys().cloned().zip(0..).collect();

    for (node, neighbours) in graph.into_iter() {
        for next in neighbours {
            nodes[str_to_idx[node]].insert(str_to_idx[next], 1);
            nodes[str_to_idx[next]].insert(str_to_idx[node], 1);
        }
    }
    nodes
}

fn stoer_wagner(mut graph: Graph) -> usize {
    let cuts: Vec<_> = (0..graph.len() - 1)
        .map(|_| {
            let (prev, last, cut) = min_cut_phase(&graph);
            let neighbours = mem::replace(&mut graph[last], BTreeMap::new());

            for (next, weight) in neighbours {
                if next != prev {
                    *graph[next].entry(prev).or_insert(0) += weight;
                    *graph[prev].entry(next).or_insert(0) += weight;
                }
                graph[next].remove(&last);
            }
            (prev, last, cut)
        })
        .collect();

    let (best_phase, (_prev, last, _min_cut)) = cuts
        .iter()
        .cloned()
        .enumerate()
        .min_by_key(|(_i, (_prev, _last, cut))| *cut)
        .unwrap();

    let mut stack = vec![last];
    let mut visited = vec![false; graph.len()];

    let mut graph = vec![BTreeSet::new(); graph.len()];
    let mut partition_len = 0;

    for (node, next, _cut) in cuts.into_iter().take(best_phase) {
        graph[node].insert(next);
        graph[next].insert(node);
    }

    while let Some(node) = stack.pop() {
        if !visited[node] {
            partition_len += 1;
            visited[node] = true;

            for next in graph[node].iter() {
                stack.push(*next);
            }
        }
    }
    partition_len
}

fn min_cut_phase(graph: &Graph) -> (usize, usize, i32) {
    let start = graph.iter().position(|node| !node.is_empty()).unwrap();
    let mut prev = 0;
    let mut last = 0;
    let mut cut = 0;

    let mut weights = vec![0; graph.len()];
    let mut visited = vec![false; graph.len()];
    let mut queue = BinaryHeap::from([(0, start)]);

    while let Some((weight, node)) = queue.pop() {
        if !visited[node] {
            visited[node] = true;
            prev = last;
            last = node;
            cut = weight;

            for (next, weight) in graph[node].iter() {
                if !visited[*next] {
                    weights[*next] += weight;
                    queue.push((weights[*next], *next));
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
