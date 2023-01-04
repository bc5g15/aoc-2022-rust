use std::collections::HashMap;
use std::cmp::max;

type FlowMap = HashMap<String, u32>;
type PathMap = HashMap<String, Vec<String>>;

fn read_valves(input: &String) -> (FlowMap, PathMap) {
    let mut flow_map: FlowMap = HashMap::new();
    let mut path_map: PathMap = HashMap::new();

    input.trim().lines().for_each(|line| {
        let words: Vec<&str> = line.trim().split(" ").collect();
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

// Type Node = (Name, Time passed, Pressure released, Valves opened, route)
type Node = (String, u32, u32, Vec<String>, Vec<String>);

fn best_time_pressure(input: &String) -> u32 {
    let (flows, paths) = read_valves(input);
    
    let mut nodes: Vec<Node> = vec![("AA".to_string(), 0, 0, Vec::new(), Vec::new())];

    let mut max_score: u32 = 0;
    let mut best_score_at_minute: HashMap<u32, u32> = HashMap::new();
    let mut best_path: Vec<String> = Vec::new();

    
    while nodes.len() > 0 {
        let (address, time, score, open, path) = nodes.pop().unwrap();
        // dbg!(nodes.len(), &time);

        // if we've hit the time limit then compare with the maximum
        if time > 30 {
            max_score = max(max_score, score); 
            best_path = path;
            continue;
        }

        let best_minute = best_score_at_minute.get(&time);

        if let Some(v) = best_minute {
            // Just skip on if we have a better historical score
            if *v > score {
                continue;
            }
        }
        best_score_at_minute.insert(time, score);
        // Calculate the new score (before opening any new valves)
        let new_score: u32 = score + open.iter().map(|v| flows.get(v).unwrap()).sum::<u32>();

        let mut new_path = path.clone();
        new_path.push(address.to_string());

        // Add a route for opening the current valve
        if let Some(v) = flows.get(&address) {
            if *v > 0 && !open.contains(&address) {
                let mut new_open = open.clone();
                new_open.push(address.to_string());
                nodes.push((
                    address.to_string(),
                    time+1,
                    new_score,
                    new_open,
                    new_path.clone()
                ));
                // continue;
            }
        }

        // Add routes for the different passages
        if let Some(v) = paths.get(&address) {
            v.iter().for_each(|destination| {
                nodes.push((
                    destination.to_string(),
                    time + 1,
                    new_score,
                    open.clone(),
                    new_path.clone()
                ))
            })
        }
    }
    // dbg!(best_score_at_minute);
    dbg!(best_path);
    dbg!(&max_score);
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
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

        assert_eq!(best_time_pressure(&input), 1651);
    }
}