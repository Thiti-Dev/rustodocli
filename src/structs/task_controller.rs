use chrono::*; // import every shit -> like struct-> trait
use prettytable::{Table, Row, Cell,format};

#[allow(dead_code)] #[derive(Debug)] 
pub struct Task{
    pub name: String,
    pub done: bool,
    pub created_at: DateTime<Local>
}

impl Task {
    pub fn new(task_name: &str) -> Self{
        Task{
            name: task_name.to_string(),
            created_at: chrono::offset::Local::now(),
            done: false
        }
    }
}

pub struct TaskController{
    pub tasks: Vec<Task>
}

impl TaskController {
    pub fn init() -> Self{
        TaskController{
            tasks: Vec::new()
        }
    }
    /*pub fn create(&mut self,task_name: &str) -> Result<(),&str>{
        let task = Task{
            name: task_name.to_string(),
            created_at: chrono::offset::Local::now()
        };
        self.tasks.push(task);
        Ok(())
    }*/
    pub fn create(&mut self,task_name: &str){
        let task = Task::new(task_name);
        self.tasks.push(task);
    }
    pub fn remove(&mut self,index: u8) -> Result<(),&str>{
        //checking if len is less than the given index
        let total_tasks = self.tasks.len();
        if total_tasks <= 0 { return Err("The given index number isn't valid") }
        if total_tasks-1 < index.into(){
            return Err("The given index number isn't valid")
        }
        // ─────────────────────────────────────────────────────────────────
        self.tasks.remove(index.into());
        Ok(())
    }
    pub fn toggle(&mut self,index: u8) -> Result<(),&str>{
        //checking if len is less than the given index
        let total_tasks = self.tasks.len();
        if total_tasks <= 0 { return Err("The given index number isn't valid") }
        if total_tasks-1 < index.into(){
            return Err("The given index number isn't valid")
        }
        // ─────────────────────────────────────────────────────────────────
        self.tasks[index as usize].done = !self.tasks[index as usize].done;
        Ok(())
    }
    pub fn total(&self) -> usize{
            self.tasks.len()
    }
    pub fn _print_all_native(&self){
        //use iter over into_iter because the value cannot move out (shared)
        println!(
            "{0: <10} | {1: <30} | {2: <10} ",
            "task_name", "created_at", "is_done"
        );
        for task in self.tasks.iter(){
            println!("{0: <10} | {1: <30} | {2: <10}", task.name, task.created_at, "false");
        }
    }
    pub fn print_all_pretty_table(&self){
        // The length of the task-name is dynamic so we should use this method instead of the native one
        let mut table = Table::new();
        //table.add_row(row!["No.","task_name", "created_at", "is_done"]);

        if self.total() > 0{
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["No.","task_name", "created_at", "is_done"]);
            for (index,task) in self.tasks.iter().enumerate(){
                let rendered_status_str:&str = if task.done {"yes"} else {"no"};
                table.add_row(row![index,task.name, task.created_at, rendered_status_str]);
            }
        }else{
            // else if tasks are empty
            table.add_row(row!["Tasks are empty at the moment. . . go create one"]);
        }
        table.printstd();
    }
}