use std::io;

fn main() {
    let list = vec!["Add Task", "View Tasks", "Complete Task", "Exit"];

    let mut todo_list: Vec<String> = Vec::new();

    println!("Welcome to Rust To-Do List!");

    loop {
        for (i, task) in list.iter().enumerate() {
            println!("{:?} {:?}", i + 1, task);
        }

        println!("Enter you choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Fail to read input");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                return;
            }
        };

        match choice {
            1 => {
                println!("Enter task:");
                let mut new_task = String::new();
                io::stdin()
                    .read_line(&mut new_task)
                    .expect("Fail to read input");
                todo_list.push(new_task.trim().to_string());
            }
            2 => {
                if todo_list.is_empty() {
                    println!("No task yet!");
                }
                for (i, item) in todo_list.iter().enumerate() {
                    println!("{:?} {:?}", i + 1, item);
                }
            }
            3 => {
                println!("Enter task number to complete:");
                let mut task_id = String::new();
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Fail to read input");

                let task_id: usize = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };

                todo_list.remove(task_id - 1);
                println!("Task completed!");
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}
