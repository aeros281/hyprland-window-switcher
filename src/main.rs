use hyprland::data::{Client, Clients, WorkspaceBasic};
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::prelude::*;
use rofi;
use std::collections::HashMap;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
