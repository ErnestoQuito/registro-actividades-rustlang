use std::io::{self, Write};
use chrono::prelude::*;
use std::thread;
use std::time::Duration;

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

    let task = nombre_tarea();
    let start_task= Local::now();
    print!("INICIO={}={}", start_task, task.trim());
    io::stdout().flush().unwrap();

    thread::sleep(Duration::new(4, 0));

    let end_task = Local::now();
    println!(" - Fin: {}", end_task);
}
