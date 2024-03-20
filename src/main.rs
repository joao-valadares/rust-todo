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
        println!("3 - Concluir Tarefa\n");
        println!("4 - Editar tarefa\n");
        println!("5 - sair\n");


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

            println!("{:#?}", tasks);

        } else if operation == "3" {
            println!("Qual tarefa você deseja completar?");

            let mut tarefa : String = String::new();

            io::stdin()
            .read_line(&mut tarefa)
            .expect("could not read index");

            let index : usize = tarefa.trim().parse().expect("invalid input");

            tasks[index].done = true;

            println!("{:#?}", tasks[index]);

        } else if operation == "4" {
            





        } else if operation == "5" {
            break;
        } else {
            println!("Operação inválida");
        }

}


}


