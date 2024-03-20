use std::io;
mod add;

#[derive(Debug)]
struct Task {
    content: String,
    done: bool,
    id: usize
}

fn creat_task(content: String, id: usize) -> Task{
    
    let mut task = Task {
        content: content,
        done: false,
        id: id
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
        println!("5 - Deletar Tarefa\n");
        println!("6 - sair\n");


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
            
            let content = content.trim();

            let mut tsk = creat_task(content.to_owned(), tasks.len());
            tasks.push(tsk);

        } else if operation == "2" {

            println!("{:#?}", tasks);

        } else if operation == "3" {
            println!("Qual tarefa você deseja completar?");

            let mut task_index : String = String::new();

            io::stdin()
            .read_line(&mut task_index)
            .expect("could not read index");

            let index : usize = task_index.trim().parse().expect("invalid input");

            tasks[index].done = true;

            println!("{:#?}", tasks[index]);

        } else if operation == "4" {
            println!("Qual tarefa você deseja editar?");

            let mut task_index : String = String::new();

            io::stdin()
            .read_line(&mut task_index)
            .expect("could not read index");

            let index : usize = task_index.trim().parse().expect("invalid input");

            println!("Qual o novo conteúdo da tarefa?");

            let mut new_content : String = String::new();

            io::stdin()
            .read_line(&mut new_content)
            .expect("could not read index");

            let new_content = new_content.trim();

            tasks[index].content = new_content.to_owned();

        } else if operation == "5" {
            println!("Qual tarefa você deseja excluir?");

            let mut task_index : String = String::new();

            io::stdin()
            .read_line(&mut task_index)
            .expect("could not read index");

            let index : usize = task_index.trim().parse().expect("invalid input");

            tasks.remove(index);

            println!("{:#?}", tasks);


        } else if operation == "6" {
            break;
        } else {
            println!("Operação inválida");
        }

}


}


