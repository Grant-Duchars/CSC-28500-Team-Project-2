// Author: Grant Duchars
// Project: CSC 28500 Team Project 2
// Problem: Chapter 10 Computer Project 3
use crossterm::{
    cursor, execute,
    style::{self, Attribute, Color, Stylize},
    terminal,
};
use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout};

enum EdgeOption {
    Edge((String, String)),
    WrongInput,
    Done,
}

fn main() {
    // Print out greeting and instructions to user
    print_welcome();

    // Create a map to store edges input by user
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    // Loop and grab entered edges until user enters done
    loop {
        match ask_for_edge() {
            EdgeOption::Edge(edge) => {
                let (a, b) = edge;
                edges
                    .entry(a.to_owned())
                    .or_insert(Vec::new())
                    .push(b.to_owned());
                edges.entry(b).or_insert(Vec::new()).push(a);
            }
            EdgeOption::WrongInput => continue,
            EdgeOption::Done => break,
        };
    }

    // Create a set for red and blue verticies for bipartiteness checking
    let mut red: HashSet<String> = HashSet::new();
    let mut blue: HashSet<String> = HashSet::new();

    // Loop trough each edge in the map and check the verticies' bipartiteness with all of its neighbors
    for edge in edges.iter() {
        let (vertex, connections) = edge;
        // if the current vertex is red, check if any neighbors are also red
        if red.contains(vertex) {
            if check_connections(&red, connections) {
                return print_result(false);
            }
        // if the current vertex is blue, check if any neighbors are also blue
        } else if blue.contains(vertex) {
            if check_connections(&blue, connections) {
                return print_result(false);
            }
        // if the current vertex was neither red nor blue, make it red and each of its neighbors blue
        } else {
            red.insert(vertex.to_string());
            for vertex in connections {
                blue.insert(vertex.to_string());
            }
            // then check if any of the current vertex's neighbors are also red
            if check_connections(&red, connections) {
                return print_result(false);
            }
        }
    }

    // If it makes it here then the graph is bipartite
    print_result(true);
}

fn ask_for_edge() -> EdgeOption {
    // Ask for user input
    let mut stdout = stdout();
    execute!(
        stdout,
        style::PrintStyledContent(
            "Please give me an edge:"
                .with(Color::Yellow)
                .attribute(Attribute::Underlined)
        )
    )
    .unwrap();

    // Store the user input
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let len = input.trim_end_matches(&['\r', '\n']).len();
    input.truncate(len);

    // Check if the user entered an edge or done
    match input.as_str() {
        "done" => return EdgeOption::Done,
        "Done" => return EdgeOption::Done,
        "DONE" => return EdgeOption::Done,
        _ => {
            let mut input = input.split(',');
            return EdgeOption::Edge((
                match input.next() {
                    Some(str) => str.to_string(),
                    None => {
                        execute!(
                            stdout,
                            style::PrintStyledContent("Error".with(Color::Red)),
                            style::Print(": Please enter an edge (ex. 'a,b') or type 'done'\n")
                        )
                        .unwrap();
                        return EdgeOption::WrongInput;
                    }
                },
                match input.next() {
                    Some(str) => str.to_string(),
                    None => {
                        execute!(
                            stdout,
                            style::PrintStyledContent("Error".with(Color::Red)),
                            style::Print(": Please enter an edge (ex. 'a,b') or type 'done'\n")
                        )
                        .unwrap();
                        return EdgeOption::WrongInput;
                    }
                },
            ));
        }
    }
}

fn check_connections(color: &HashSet<String>, connections: &Vec<String>) -> bool {
    for vertex in connections {
        if color.contains(vertex) {
            return true;
        }
    }
    false
}

fn print_welcome() {
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print("\nHello, I am the "),
        style::PrintStyledContent("ora".with(Color::Rgb {r: 255, g: 0, b: 200})),
        style::PrintStyledContent("cle".with(Color::Rgb {r: 213, g: 43, b: 209})),
        style::PrintStyledContent(" of".with(Color::Rgb {r: 170, g: 85, b: 218})),
        style::PrintStyledContent(" bip".with(Color::Rgb {r: 128, g: 128, b: 228})),
        style::PrintStyledContent("art".with(Color::Rgb {r: 85, g: 170, b: 237})),
        style::PrintStyledContent("ite".with(Color::Rgb {r: 43, g: 213, b: 246})),
        style::PrintStyledContent("ness".with(Color::Rgb {r: 0, g: 255, b: 255})),
        style::SetForegroundColor(Color::White),
        style::Print(" and if you give me a list of edges from a graph I will tell you if that graph is bipartite or not.\n"),
        style::Print("Now start giving me your edges in the form of "),
        style::PrintStyledContent("two strings separated by a comma".attribute(Attribute::Underlined)),
        style::Print(". Once you have given me all of your edges simply type "),
        style::PrintStyledContent("done".attribute(Attribute::Underlined)),
        style::Print(" and I will give you the information you seek.\n"),
    )
    .unwrap();
}

fn print_result(is_bipartite: bool) {
    let mut stdout = stdout();
    if is_bipartite {
        execute!(
            stdout,
            style::Print("In my infinite wisdom and knowledge, I do declare that the graph you have provided me is "),
            style::PrintStyledContent("b".with(Color::Rgb {r: 255, g: 0, b: 200})),
            style::PrintStyledContent("i".with(Color::Rgb {r: 223, g: 32, b: 207})),
            style::PrintStyledContent("p".with(Color::Rgb {r: 191, g: 64, b: 214})),
            style::PrintStyledContent("a".with(Color::Rgb {r: 159, g: 96, b: 221})),
            style::PrintStyledContent("r".with(Color::Rgb {r: 128, g: 128, b: 228})),
            style::PrintStyledContent("t".with(Color::Rgb {r: 96, g: 159, b: 234})),
            style::PrintStyledContent("i".with(Color::Rgb {r: 64, g: 191, b: 241})),
            style::PrintStyledContent("t".with(Color::Rgb {r: 32, g: 223, b: 248})),
            style::PrintStyledContent("e".with(Color::Rgb {r: 0, g: 255, b: 255})),
            style::Print(".\n")
        ).unwrap();
    } else {
        execute!(
            stdout,
            style::Print("In my infinite wisdom and knowledge, I do declare that the graph you have provided me is "),
            style::PrintStyledContent("n".with(Color::Rgb {r: 255, g: 0, b: 200})),
            style::PrintStyledContent("o".with(Color::Rgb {r: 232, g: 23, b: 205})),
            style::PrintStyledContent("n".with(Color::Rgb {r: 209, g: 46, b: 210})),
            style::PrintStyledContent("b".with(Color::Rgb {r: 185, g: 70, b: 215})),
            style::PrintStyledContent("i".with(Color::Rgb {r: 162, g: 93, b: 220})),
            style::PrintStyledContent("p".with(Color::Rgb {r: 139, g: 116, b: 225})),
            style::PrintStyledContent("a".with(Color::Rgb {r: 116, g: 139, b: 230})),
            style::PrintStyledContent("r".with(Color::Rgb {r: 93, g: 162, b: 235})),
            style::PrintStyledContent("t".with(Color::Rgb {r: 70, g: 185, b: 240})),
            style::PrintStyledContent("i".with(Color::Rgb {r: 46, g: 209, b: 245})),
            style::PrintStyledContent("t".with(Color::Rgb {r: 23, g: 232, b: 250})),
            style::PrintStyledContent("e".with(Color::Rgb {r: 0, g: 255, b: 255})),
            style::Print(".\n")
        ).unwrap();
    }
}
