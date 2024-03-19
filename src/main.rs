use std::io;
use std::io::Write;
mod add;
use std::any::type_name;

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

    loop{

        println!("Olá, qual operação deseja fazer?\n");
        println!("1 - adicionar tarefa\n");
        println!("2 - ler tarefas adicionadas\n");
        println!("3 - sair\n");


        let mut operation = String::new();

        io::stdin()
            .read_line(&mut operation)
            .expect("couldnt read value");

        operation = operation.trim().to_string();

        if operation == "1" {
            
            println!("Digite a descricao da sua tarefa:");           
            
            let mut content = String::new();
            io::stdin()
            .read_line(&mut content)
            .expect("couldnt read value");
            
            let mut tsk = creat_task(content);
            tasks.push(tsk);

        } else if operation == "2" {

            println!("{:?}", tasks);
            
        } else if operation == "3" {
            break;
        }

}


}


