use std::io;

#[derive(Debug)]
struct Task {
    content: String,
    done: bool
}

fn creat_task(content: String) -> Task{
    
    let mut task = Task {
        content: content,
        done: false
    };

    return task;
}

fn main() {

    let mut tasks : Vec<Task> = Vec::new();
    println!("Digite a descricao da sua tarefa:");

    let mut content = String::new();
    io::stdin().read_line(&mut content).unwrap();

    let mut tsk = creat_task(content);

    println!("{:?}", tsk);

}


