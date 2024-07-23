use std::collections::VecDeque;

pub struct OrderGraph {
    directions: Box<[Box<[usize]>]>,
    directors_count: Box<[usize]>,
    initial_nodes: Box<[usize]>,
}

impl OrderGraph {
    pub fn new(
        directions: Box<[Box<[usize]>]>,
    ) -> Option<Self> {
        let mut directors_count = std::iter::repeat(0_usize).take(directions.len()).collect::<Box<_>>();

        for node in 0..directions.len() {
            for &directed_node in directions[node].iter() {
                if directed_node == node {
                    return None;
                }

                directors_count[directed_node] += 1;
            }
            if directions[node].iter().any(|j| *j == node) {
                return None;
            }
        }

        let initial_nodes = directors_count.iter().enumerate().filter(|(_, c)| **c == 0).map(|(i, _)| i).collect();

        // todo: Add graph validation.

        Some(Self {
            directions,
            directors_count,
            initial_nodes,
        })
    }

    pub fn iter(&self) -> OrderGraphIterator {
        OrderGraphIterator::new(self)
    }
}

pub struct OrderGraphIterator {
    directions: Box<[Box<[usize]>]>,
    queue: VecDeque<usize>,
    unvisited_directors_count: Box<[usize]>,
    processing_count: usize,
}

impl OrderGraphIterator {
    pub fn new(graph: &OrderGraph) -> Self {
        let mut queue = VecDeque::new();

        for initial_node in graph.initial_nodes.iter() {
            queue.push_back(*initial_node)
        }

        let unvisited_directors_count = graph.directors_count.clone();

        Self {
            directions: graph.directions.clone(),
            queue,
            unvisited_directors_count,
            processing_count: 0,
        }
    }

    pub fn start_next(&mut self) -> Option<usize> {
        let node = self.queue.pop_front()?;

        self.processing_count += 1;

        Some(node)
    }

    pub fn end(&mut self, node: usize) {
        self.processing_count -= 1;

        for direction in self.directions[node].iter() {
            let direction_directors_count = &mut self.unvisited_directors_count[*direction];
    
            *direction_directors_count -= 1;

            if *direction_directors_count == 0 {
                self.queue.push_back(*direction);
            } 
        }
    }

    pub fn all_started(&self) -> bool {
        self.queue.len() == 0
    }

    pub fn all_ended(&self) -> bool {
        self.all_started() && self.processing_count == 0
    }
}