use std::fmt;
use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]

struct MainTemplate<'a> {
    title: &'a str,
    name: &'a str,
    tasks: Vec<Task>,
}

struct Task {
    id: i32,
    name: String,
    done: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

#[tokio::main]
async fn main() {
    let tasks = vec![
        Task {
            id: 1,
            name: "Learn Rust".to_string(),
            done: true,
        },
        Task {
            id: 2,
            name: "Learn Askama".to_string(),
            done: false,
        },
    ];

    let main_template = MainTemplate {
        title: "My Tasks",
        name: "Dozie",
        tasks,
    };

    println!("{}", main_template.render().unwrap());
    

}
