use clap::Parser;
use hyprland::data::{Client, Clients, WorkspaceBasic};
use hyprland::dispatch::{
    Dispatch, DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial,
};
use hyprland::prelude::*;
use rofi;
use std::collections::HashMap;

mod cli;

fn get_titles(clients: &Vec<Client>) -> HashMap<&String, i32> {
    clients
        .iter()
        .filter(|Client { title, .. }| !title.is_empty())
        .filter(
            |Client {
                 workspace: WorkspaceBasic { id, .. },
                 ..
             }| *id != -99, // id -99 is special workspace
        )
        .map(|Client { title, pid, .. }| (title, *pid))
        .collect()
}

fn switch_window_by_titles() -> Result<(), Box<dyn std::error::Error>> {
    let clients = Clients::get()?.to_vec();
    println!("{clients:?}");
    let titles = get_titles(&clients);
    let entries = Vec::from_iter(titles.keys());

    let choice = rofi::Rofi::new(&entries).run()?;
    println!("Choice: {}", choice);
    let pid: u32 = titles[&choice].try_into().unwrap();

    println!("Pid: {}", pid);
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::ProcessId(pid)))?;

    Ok(())
}

fn switch_workspaces() -> Result<(), Box<dyn std::error::Error>> {
    let workspaces: Vec<String> = (1..11).into_iter().map(|val| val.to_string()).collect();

    let choice = rofi::Rofi::new(&workspaces).run()?;
    println!("Choice: {}", choice);
    let workspace_id: i32 = choice.parse::<i32>().unwrap();

    Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        workspace_id,
    )))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli::Cli::parse() {
        cli::Cli::SwitchWindowByTitle => switch_window_by_titles(),
        cli::Cli::SwitchWorkspace => switch_workspaces(),
    }
}
