use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Parser {
    options: Vec<Options>,
    headers: Arena,
}

#[derive(Debug)]
pub struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    pub fn new_node(&mut self, data: Header) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node {
            parent: None,
            children: Vec::new(),
            previous_sibling: None,
            next_sibling: None,
            index: next_index,
            data, // Title, Text, Level
        });

        // Return the node identifier
        NodeId { index: next_index }
    }
}

impl Serialize for Arena {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // In the end we want to output:
        /* [
            {
                level: u32,
                title: str,
                text: [
                    "some line",
                    "some other line",
                    "some third line"
                ],
            }
        ]
        */
        let mut seq = serializer.serialize_seq(Some(self.nodes.len()))?;
        for node in &self.nodes {
            seq.serialize_element(&node)?;
        }
        seq.end()
    }
}

#[derive(Debug)]
pub struct Node {
    parent: Option<NodeId>,
    previous_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Vec<NodeId>,
    index: usize,

    pub data: Header,
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut header = serializer.serialize_struct("Header", 6)?;

        header.serialize_field("index", &self.index)?;
        header.serialize_field("data", &self.data)?;

        header.serialize_field("parent", &self.parent.map(|node| node.index))?;
        header.serialize_field("previous", &self.previous_sibling.map(|node| node.index))?;
        header.serialize_field("next", &self.previous_sibling.map(|node| node.index))?;
        header.serialize_field("children", &self.children)?;

        header.end()
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct NodeId {
    index: usize,
}

#[derive(Debug, Serialize)]
pub struct Header {
    level: usize,
    title: String,
    text: Vec<String>,
    state: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, PartialEq)]
pub enum Options {
    SEQ_TODO(Vec<SEQ_STATES>),
    // Author
    // etc.
}

#[allow(non_camel_case_types)]
type TODO_KEYWORDS = Vec<String>;
#[allow(non_camel_case_types)]
type DONE_KEYWORDS = Vec<String>;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, PartialEq)]
pub enum SEQ_STATES {
    TODO(Vec<String>),
    DONE(Vec<String>),
}

impl Parser {
    pub fn parse(data: &String) -> Self {
        let options = Self::parse_options(&data);
        let states = Self::get_states(&options);
        let headers = Self::parse_headers(&data, states);

        Self { options, headers }
    }

    pub fn print_json(&self, output: &String) -> Result<(), std::io::Error> {
        let contents = serde_json::to_string(&self)?;
        std::fs::write(output, contents)
    }

    pub fn print_json_pretty(&self, output: &String) -> Result<(), std::io::Error> {
        let contents = serde_json::to_string_pretty(&self)?;
        std::fs::write(output, contents)
    }

    fn count_level(line: &str) -> usize {
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

    fn parse_options(data: &String) -> Vec<Options> {
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

                    // DONE(d) => DONE, CRITICAL(c) => CRITICAL, ...
                    let option = if option.contains('(') && option.contains(')') {
                        option.split('(').next().unwrap().to_string()
                    } else {
                        option.to_string()
                    };

                    if todo {
                        seq_todo_options.push(option);
                    } else {
                        seq_done_options.push(option);
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

        options
    }

    fn get_states(options: &Vec<Options>) -> (Option<&TODO_KEYWORDS>, Option<&DONE_KEYWORDS>) {
        // Handle States - TODO | DONE | CUSTOM
        let mut todo_keywords: Option<&Vec<String>> = None;
        let mut done_keywords: Option<&Vec<String>> = None;

        for Options::SEQ_TODO(states) in options {
            for state in states {
                match state {
                    SEQ_STATES::TODO(todo) => todo_keywords = Some(todo),
                    SEQ_STATES::DONE(done) => done_keywords = Some(done),
                }
            }
        }

        (todo_keywords, done_keywords)
    }

    fn parse_state(
        line: &str,
        states: (Option<&TODO_KEYWORDS>, Option<&DONE_KEYWORDS>),
    ) -> Option<String> {
        let (todo_keywords, done_keywords) = states;
        let mut header_state: Option<String>;

        // *** TODO Some Title => TODO
        let word = line.split_ascii_whitespace().nth(1);
        if word.is_none() || word.unwrap().is_empty() {
            return None;
        }

        let word = word.unwrap();

        match todo_keywords {
            None => {
                header_state = Some("TODO".to_string());
            }
            Some(keywords) => {
                header_state = keywords
                    .iter()
                    .filter(|keyword| *keyword == word)
                    .next()
                    .and_then(|x| Some(x.clone().to_string()));
            }
        }

        if header_state.is_some() {
            return header_state; // Found TODO related header
        }

        match done_keywords {
            None => {
                header_state = Some("DONE".to_string());
            }
            Some(keywords) => {
                header_state = keywords
                    .iter()
                    .filter(|keyword| *keyword == word)
                    .next()
                    .and_then(|x| Some(x.clone().to_string()));
            }
        }

        header_state // Found DONE related header
    }

    /// Parse the title of a Header, removing the options if they exist
    fn parse_title(line: &str, header_level: usize, state: &Option<String>) -> String {
        let title = line.split_at(header_level).1.trim_start().to_string();

        if let Some(keyword) = &state {
            // Prevent panic on empty titles:
            if title.len() > keyword.len() {
                title.split_at(keyword.len() + 1).1.to_string()
            } else {
                "".to_string()
            }
        } else {
            title
        }
    }

    fn parse_headers(
        data: &String,
        options: (Option<&TODO_KEYWORDS>, Option<&DONE_KEYWORDS>),
    ) -> Arena {
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

            let new_header_level = Self::count_level(line);
            let state = Self::parse_state(line, options);
            let title = Self::parse_title(line, new_header_level, &state);

            let data = Header {
                level: new_header_level,
                title,
                text: Vec::new(),
                state,
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

#[cfg(test)]
mod tests {
    use super::Options;
    use super::Parser;
    use super::SEQ_STATES;

    #[test]
    fn test_count_level() {
        assert_eq!(Parser::count_level("*** H3 Heading"), 3);
        assert_eq!(Parser::count_level("** H2 Heading"), 2);
        assert_eq!(Parser::count_level("* H1 Heading"), 1);
        assert_eq!(Parser::count_level("*** H3 Heading"), 3);
    }

    #[test]
    fn test_parse_options_simple() {
        let options = "#+TODO: TODO(t) | DONE(d)".to_string();
        let todo = SEQ_STATES::TODO(vec!["TODO".to_string()]);
        let done = SEQ_STATES::DONE(vec!["DONE".to_string()]);

        let output = Parser::parse_options(&options);
        assert_eq!(output, [Options::SEQ_TODO(vec![todo, done])]);
    }

    #[test]
    fn test_parse_options_complex() {
        let options = "#+TODO: TODO(t) CRITICAL(c) NOT_DONE(n) | COMPLETED(e) DONE(d)".to_string();
        let todo = SEQ_STATES::TODO(vec![
            "TODO".to_string(),
            "CRITICAL".to_string(),
            "NOT_DONE".to_string(),
        ]);
        let done = SEQ_STATES::DONE(vec!["COMPLETED".to_string(), "DONE".to_string()]);

        let output = Parser::parse_options(&options);
        assert_eq!(output, [Options::SEQ_TODO(vec![todo, done])]);
    }

    #[test]
    fn test_parse_title() {
        let title = "* Header";
        assert_eq!("Header", Parser::parse_title(title, 1, &None));

        let title = "*** Some title";
        assert_eq!("Some title", Parser::parse_title(title, 3, &None));

        let title = "** CUSTOM_TODO test";
        assert_eq!(
            "test",
            Parser::parse_title(title, 2, &Some("CUSTOM_TODO".to_string()))
        );
    }
}
