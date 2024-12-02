use std::io::{self, Write};
use chrono::prelude::*;

fn saludos() {
    let saludo = "## BIENVENIDO AL REGISTRO DE TAREAS ##";
    let michi = "#";
    let repeat_michi = michi.repeat(saludo.len());

    println!();
    println!("{}", repeat_michi);
    println!("{}", saludo);
    println!("{}", repeat_michi);
}

fn nombre_tarea() -> String {
    let mut tarea = String::new();

    loop {
        print!("Add task: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut tarea)
            .unwrap();

        if tarea.trim().is_empty() {
            println!("Task not can be empty. Please train again.")
        } else {
            return tarea;
        }
    }
}

fn main() {
    saludos();
    // Iniciamos loop
    loop {
        // iniciamos tarea y el tiempo de inicio.
        let task = nombre_tarea();
        let start_task= Local::now();

        // Imprimimos inicio y tarea.
        println!("TAREA EN CURSO:\t{}", task.trim());
        println!("INICIO:\t\t{}", start_task);
        println!("FIN:\t\tEN CURSO");

        // Esperamos enter para finalizar tareas
        println!("Press enter (new task) or 'q' (exit) ...");
        let mut enter_key = String::new();
        io::stdin()
            .read_line(&mut enter_key)
            .unwrap();
        
        if enter_key == "\n" {
            let end_task = Local::now();
            println!("FIN:\t\t{}", end_task);
        }

        if enter_key.trim() == "q" {
            let end_task = Local::now();
            println!("FIN:\t\t{}", end_task);
            println!("Good bye");
            break;
        }
    }
}
