use std::io::{self, Read};
struct Task {
    content: String,
    done: bool
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


