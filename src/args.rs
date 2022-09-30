use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodustArgs {
    #[clap(subcommand)]
    pub command: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// List all tasks that are not done
    List(ListTask),

    /// Create a task taking the Name and a optional descripiton
    Create(CreateTask),

    /// Complete a task selected by id
    Complete(CompleteTask),

    /// Delete a selected task taking the task id
    Delete(DeleteTask),
}

#[derive(Debug, Args)]
pub struct ListTask {}

#[derive(Debug, Args)]
pub struct CreateTask {
    /// Title of the task
    pub title: String,

    /// Description of the task
    pub descripiton: Option<String>,
}

#[derive(Debug, Args)]
pub struct CompleteTask {
    /// Title of the task
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct DeleteTask {
    /// Title of the task
    pub id: u32,
}
