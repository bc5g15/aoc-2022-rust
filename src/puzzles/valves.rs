use std::collections::HashMap;

type FlowMap = HashMap<String, u32>;
type PathMap = HashMap<String, Vec<String>>;

fn read_valves(input: &String) -> (FlowMap, PathMap) {
    let mut flow_map: FlowMap = HashMap::new();
    let mut path_map: PathMap = HashMap::new();

    input.trim().lines().for_each(|line| {
        let comma_stripped: String = line.trim().chars().filter(|c| *c!=',').collect();
        let words: Vec<&str> = comma_stripped.split(" ").collect();
        
        let id = words[1].to_string();
        
        let flow: u32 = words[4].chars().filter(|c| c.is_ascii_digit()).collect::<String>()
        .parse().unwrap();
        
        flow_map.insert(id.to_string(), flow);

        for i in 9..words.len() {
            let destination = words[i].to_string();
            if let Some(v) = path_map.get_mut(&id) {
                v.push(destination.to_string());
            } else {
                path_map.insert(id.to_string(), vec![destination.to_string()]);
            }

            if let Some(v) = path_map.get_mut(&destination) {
                v.push(id.to_string());
            } else {
                path_map.insert(destination.to_string(), vec![id.to_string()]);
            }
        }
    });

    (flow_map, path_map)
}

fn graphify(path_map: &PathMap)  -> (Vec<Vec<u32>>, HashMap<String, usize>) {
    let size = path_map.len();

    let idx_map: HashMap<String, usize> = path_map.iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (index, (name, _))| {
            map.insert(name.clone(), index);
            map
        });

    let mut graph = vec![vec![u32::MAX / 4; size]; size];

    path_map.iter().enumerate().for_each(|(i, (_name, paths))| {
        paths.iter().for_each(|s| {
            let address = *idx_map.get(s).unwrap();
            graph[i][address] = 1; // Direct link exists!
        })
    });

    (graph, idx_map)
}

fn floyd_warshall_roy(graph: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let l = graph.len();
    let mut distance = graph.clone();

    for k in 0..l {
        for i in 0..l {
            for j in 0..l {
                if distance[i][k] + distance[k][j] < distance[i][j] {
                    distance[i][j] = distance[i][k] + distance[k][j];
                }
            }
        }
    }

    distance
}

pub fn part1(input: &String) -> u32 {
    let (flow, _) = magical_calculation(input, 30);
    flow
}

pub fn magical_calculation(input: &String, turns: u32) -> (u32, HashMap<u64, u32>) {
    let (flows, paths) = read_valves(input);
    let (graph, idx_map) = graphify(&paths);
    let distances = floyd_warshall_roy(graph);

    let scored_nodes: Vec<String> = flows.iter()
        .filter(|(_name, score)| **score>0)
        .map(|(name, _score)| name.clone()).collect();
        // Find optimal path through all point-scoring nodes
    
    let start_index = *idx_map.get("AA").unwrap();

    let mut state_flow: HashMap<u64, u32> = HashMap::new();

    let flow = travelling_salesman(&flows, &mut state_flow, &scored_nodes, &distances, &idx_map, turns, 0, start_index, 0);

    (flow, state_flow)
}

pub fn part2(input: &String) -> u32 {
    let (_, memory) = magical_calculation(input, 26);

    let max_flow = memory.iter()
        .fold(0, |max, (&state1, &flow1)| {
            memory.iter()
                .fold(max, |max, (&state2, &flow2)| {
                    // Check there is no overlap
                    if state1 & state2 == 0 {
                        return max.max(flow1 + flow2);
                    }
                    max
                })
        });

    max_flow
}

fn travelling_salesman(
    flows: &FlowMap,
    memory: &mut HashMap<u64, u32>,
    scored_nodes: &Vec<String>,
    distances: &Vec<Vec<u32>>,
    idx_map: &HashMap<String, usize>,
    minutes: u32,
    flow: u32,
    current_index: usize,
    state: u64
) -> u32 {

    let mut max_flow = flow;

    memory.insert(state, *memory.get(&state).unwrap_or(&0).max(&flow));

    // println!("C:{}\nM:{}\nF:{}\nS:{:b}\n---", current_index, minutes, flow, state);
    // dbg!(minutes, flow, "---");

    for name in scored_nodes.iter() {
        let new_index: usize = *idx_map.get(name).unwrap();
        let current_minutes = minutes
            .checked_sub(distances[current_index][new_index])
            .and_then(|x| x.checked_sub(1))
            .unwrap_or(0);

        if 
            state & (1 << new_index) >= 1   // Already switched on
            || current_minutes <= 0         // Can't reach in time
        {
            // Don't care about this node
            continue;
        }

        // dbg!(name);
        let new_state = state | (1 << new_index);
        // let new_state = state;
        let new_flow = flow + (current_minutes * flows.get(name).unwrap());

        max_flow = max_flow.max(
            travelling_salesman(flows, memory, scored_nodes, distances, idx_map, current_minutes, new_flow, new_index, new_state)
        )
    }

    max_flow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample() {
        let input = r"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II".to_string();

        assert_eq!(part1(&input), 1651);
    }

    #[test]
    fn magic2() {
        let input = r"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II".to_string();

        assert_eq!(part2(&input), 1707);
    }

    #[test]
    fn array_wizardry() {
        let thing = vec![vec![u32::MAX / 4; 10]; 10];

        dbg!(thing);

        dbg!((1<<16)-1);
        println!("{:b}", (1<<16));
    }
}