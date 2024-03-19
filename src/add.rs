use std::io::{self, Read};

use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct Task {
    content: String,
    done: Bool
}


fn creat_task(content: String) -> Task{
    let mut tasks : Vec<Task> = Vec::new();

    let mut content = String::new();
    io::stdin().read_to_string(&mut content).unwrap();

    let mut task = Task {
        content: content,
        done: false
    };

    return task;
}


