use std::collections::HashMap;

struct Graph {
    map: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            map: HashMap::new(),
        }
    }

    fn find_all_paths(
        &self,
        start: &str,
        history: &mut HashMap<String, u8>,
        mut can_visit_cave_twice: bool,
    ) -> u32 {
        self.map.get(start).unwrap().iter().fold(0u32, |acc, path| {
            if path == "start" {
                return acc;
            }

            if path == "end" {
                return acc + 1;
            }

            let is_small = path.to_uppercase() != *path;

            if is_small {
                if let Some(&value) = history.get(path) {
                    if value == 0 && can_visit_cave_twice {
                        history.insert(path.to_owned(), 1);
                        can_visit_cave_twice = false
                    } else {
                        return acc;
                    }
                } else {
                    history.insert(path.to_owned(), 0);
                }
            }

            let res = acc + self.find_all_paths(path, history, can_visit_cave_twice);

            if is_small {
                let &value = history.get(path).unwrap();
                if value == 0 {
                    history.remove(path);
                } else {
                    history.insert(path.to_owned(), value - 1);
                    can_visit_cave_twice = true;
                }
            }

            res
        })
    }
}

fn get_input() -> Graph {
    let mut graph: Graph = Graph::new();

    include_str!("./inputs/day12.real")
        .lines()
        .map(|s| s.split_once('-').unwrap())
        .for_each(|(a, b)| {
            graph
                .map
                .entry(a.to_string())
                .or_insert_with(Vec::new)
                .push(b.to_string());

            graph
                .map
                .entry(b.to_string())
                .or_insert_with(Vec::new)
                .push(a.to_string());
        });

    graph
}

pub fn solve_one() {
    let graph = get_input();

    println!("{:#?}", graph.map);

    println!("---- GRAPH ----");

    println!(
        "{:#?}",
        graph.find_all_paths("start", &mut HashMap::new(), true)
    );
}
