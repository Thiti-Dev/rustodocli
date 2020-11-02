use std::io::{stdin, stdout, Read, Write}; //bring the trait into scope
use std::{thread, time};
use std::process::Command;
mod structs {
    pub mod task_controller;
}

//Pretty-table
#[macro_use] extern crate prettytable;
// ────────────────────────────────────────────────────────────────────────────────


fn main() {
    clear_console();
    //pre-fab
    let mut task_contoller = structs::task_controller::TaskController::init();
    // ────────────────────────────────────────────────────────────────────────────────


    println!("rust-todo created by THITI-MAHAWANNAKIT");

    loop{
        clear_console();
        println!("Total: {} tasks\n1: View all to-do lists\n2: Append new task\n3: Toggle task status as done or undone\n4: Remove task\nor else: Exit Program", task_contoller.total());
        let mut operation_txt = String::new();
        print!("Whant operation do you want to do : ");
        let _ = stdout().flush(); // to make the print from above visible within the same line as the user_input
        stdin().read_line(&mut operation_txt).expect("Unable to get to input from the user");
        
        let operation_as_static_str: &str = &operation_txt[..].trim(); // need to be trimmed to remove the \r \n
        match operation_as_static_str {
            "1" => {
                task_contoller.print_all_pretty_table();
                pause();
            },
            "2" => {
                let task_name = get_input("describe your task (task-name)");
                task_contoller.create(task_name.as_str());
                print_then_pause(format!("Created task->{} successfully", task_name).as_str());
            },
            "3" => {
                let mut iter_state :bool = true;

                while iter_state{
                    clear_console();
                    task_contoller.print_all_pretty_table();
                    let task_index_str = get_input("Which task that you wanna toggle its status");
                    let task_index : u8 = task_index_str.parse().unwrap();

                    // If an error expected -> it will continue the loop
                    match task_contoller.toggle(task_index){
                        //Err(_) => iter_state=true,
                        Ok(_) => {
                            iter_state=false;
                            print_then_pause(format!("Sucessfully toggle task:{} status", task_index).as_str());
                        },
                        Err(_) => print_then_pause(format!("Task number {} isn't existed", task_index).as_str()),
                    }
                }
            },
            "4" => {
                let mut iter_state :bool = true;

                while iter_state{
                    clear_console();
                    task_contoller.print_all_pretty_table();
                    let task_index_str = get_input("Which task you want to remove");
                    let task_index : u8 = task_index_str.parse().unwrap();

                    // If an error expected -> it will continue the loop
                    match task_contoller.remove(task_index){
                        //Err(_) => iter_state=true,
                        Ok(_) => {
                            iter_state=false;
                            print_then_pause(format!("Sucessfully removed task:{}", task_index).as_str());
                        },
                        Err(_) => print_then_pause(format!("Task number {} isn't existed", task_index).as_str()),
                    }
                }
                
            },
            _ => return,
        }
    };
}

fn clear_console(){
    let mut console_clear = Command::new("cls");
    match console_clear.status() {
        Err(_) => {
           Command::new("clear").spawn().expect("clear command failed to execute");
           thread::sleep(time::Duration::from_millis(50)); // this is needed or the instructions wont ever be shown
        },
        _ => return,
    }
}

// Should be returning out String instead of &str
fn get_input(prepend_msg : &str) -> String{
    let mut operation_txt = String::new();
    print!("{} : ", prepend_msg);
    let _ = stdout().flush(); // to make the print from above visible within the same line as the user_input
    stdin().read_line(&mut operation_txt).expect("Unable to get to input from the user");
    let operation_as_static_str: &str = &operation_txt[..].trim();
    String::from(operation_as_static_str)
}

fn print_then_pause(msg: &str){
    println!("{}", msg);
    thread::sleep(time::Duration::from_millis(1000));
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press enter to continue using the program . . . ").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    stdin().read(&mut [0]).unwrap(); // doubled to prevent the flush-flow
}