use std::io;
use std::io::Write;
mod add;

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

    while true{

        println!("Olá, qual operação deseja fazer?\n");
        println!("1 - adicionar tarefa\n");
        println!("2 - ler tarefas adicionadas\n");
        println!("3 - sair\n");


        let mut operation = String::new();

        
        io::stdin().read_line(&mut operation).unwrap();
        
        if operation == "1" {
            
            println!("Digite a descricao da sua tarefa:");
            io::stdout().flush().expect("Cannot flush stdout");
            
            
            let mut content = String::new();
            
            let mut tsk = creat_task(content);
            tasks.push(tsk);
        }
        if operation == "2" {
            println!("{:?}", tasks);
        }
        if operation == "3" {
            break;
        }

}


}


