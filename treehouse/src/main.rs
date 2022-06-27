use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("Failuuuuure");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi steve. Your milk in on the fridge."),
        Visitor::new("fred", "Wow, who invited fred ?"),
    ];

    loop {
        println!("Hello, what's your name ?");

        let your_name = what_is_your_name();

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == your_name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if your_name.is_empty() {
                    break;
                } else {
                    println!("Sorry you are not on the list.");
                    visitor_list.push(Visitor::new(&your_name, "New friend"));
                }
            }
        };
        break;
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
