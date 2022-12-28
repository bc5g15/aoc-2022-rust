use std::collections::HashMap;

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


fn best_time_pressure(input: &String) {
    let (flows, paths) = read_valves(input);
    
}
