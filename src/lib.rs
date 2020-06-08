#![allow(dead_code)]
#![allow(unused_variables)]

use serde::Serialize;

#[derive(Serialize)]
pub struct Parser {
    pub headers: Arena,
    pub options: Vec<Options>,
}

#[derive(Debug, Serialize)]
pub struct Arena {
    nodes: Vec<Node>,
}

#[derive(Debug, Serialize)]
pub struct Node {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Vec<NodeId>,

    pub data: Header,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct NodeId {
    index: usize,
}

#[derive(Debug, Serialize)]
pub struct Header {
    level: u32,
    title: String,
    text: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize)]
pub enum Options {
    SEQ_TODO(Vec<SEQ_STATES>),
    // Author
    // etc.
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize)]
pub enum SEQ_STATES {
    TODO(Vec<String>),
    DONE(Vec<String>),
}

impl Parser {
    pub fn parse(data: String) -> Self {
        // Options
        let mut seq_options: Vec<SEQ_STATES> = Vec::with_capacity(2);

        for line in data.lines() {
            if line.starts_with("#+TODO") || line.starts_with("#+SEQ_TODO") {
                let seq_todo: &str = line.split(":").nth(1).expect("SEQ_TODO requires values!");
                let mut seq_todo_options = Vec::new();
                let mut seq_done_options = Vec::new();

                let mut todo: bool = true; // KEYWORDS_TODO | KEYWORDS_DONE
                for option in seq_todo.split(' ') {
                    if option.is_empty() {
                        continue;
                    }

                    if option == "|" {
                        todo = false;
                        continue;
                    }

                    if todo {
                        seq_todo_options.push(option.to_string());
                    } else {
                        seq_done_options.push(option.to_string());
                    }
                }

                seq_options.push(SEQ_STATES::TODO(seq_todo_options));
                seq_options.push(SEQ_STATES::DONE(seq_done_options));
                continue;
            }

            break;
        }

        let mut options: Vec<Options> = Vec::new();
        options.push(Options::SEQ_TODO(seq_options));
        let headers = Self::parse_headers(data);

        Self { headers, options }
    }

    fn count_level(line: &str) -> u32 {
        let mut header_level = 0;
        for star in line.chars() {
            if star == '*' {
                header_level += 1;
            } else {
                break;
            }
        }

        header_level
    }

    fn parse_headers(data: String) -> Arena {
        let mut arena = Arena { nodes: Vec::new() };

        let mut current_node: Option<NodeId> = None;
        let mut previous_level = 1;

        for line in data.lines() {
            if !line.starts_with("*") {
                // Append text
                match current_node {
                    None => (), // Some other thing like options
                    Some(node) => arena.nodes[node.index]
                        .data
                        .text
                        .push(line.trim_end().into()),
                }

                continue;
            }

            // New node

            let new_header_level: u32 = Self::count_level(line);
            let title = line
                .split_at(line.find(' ').expect("A header needs a title!") + 1)
                .1;

            let data = Header {
                level: new_header_level,
                title: title.to_string(),
                text: Vec::new(),
            };

            let node_id = arena.new_node(data);

            loop {
                if new_header_level > previous_level {
                    // Child
                    arena.nodes[current_node.unwrap().index]
                        .children
                        .push(node_id);

                    // Parent
                    arena.nodes[node_id.index].parent = current_node;

                    previous_level += 1;
                    current_node = Some(node_id);
                    break;
                }

                if new_header_level == previous_level {
                    if current_node.is_none() {
                        // Special case, the first node of the parser
                        current_node = Some(node_id);
                        break;
                    }

                    // To set the parent, we go to the previous_sibling and get that parent
                    let parent_id = arena.nodes[current_node.unwrap().index].parent;
                    arena.nodes[node_id.index].parent = parent_id;

                    // Set node_id to be the child of my parent
                    match parent_id {
                        None => (),
                        Some(parent_id) => arena.nodes[parent_id.index].children.push(node_id),
                    }

                    // next_sibling
                    arena.nodes[current_node.unwrap().index].next_sibling = Some(node_id);
                    // previous_sibling
                    arena.nodes[node_id.index].previous_sibling = current_node;

                    current_node = Some(node_id);
                    break;
                }

                // new_header_level < previous_level

                current_node = arena.nodes[current_node.unwrap().index].parent;
                previous_level -= 1;
            }
        }

        arena
    }
}

impl Arena {
    pub fn new_node(&mut self, data: Header) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node {
            parent: None,
            children: Vec::new(),
            previous_sibling: None,
            next_sibling: None,
            data, // Title, Text, Level
        });

        // Return the node identifier
        NodeId { index: next_index }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn count_level() {
        assert_eq!(Parser::count_level("*** H3 Heading"), 3);
        assert_eq!(Parser::count_level("** H2 Heading"), 2);
        assert_eq!(Parser::count_level("* H1 Heading"), 1);
        assert_eq!(Parser::count_level("*** H3 Heading"), 3);
    }
}
