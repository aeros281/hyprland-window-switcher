use hyprland::data::Client;
use hyprland::data::Clients;
use hyprland::prelude::*;
use rofi;
use std::collections::HashMap;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};

fn get_titles(clients: &Vec<Client>) -> HashMap<&String, i32> {
    clients
        .iter()
        .map(|Client { title, pid, .. }| (title, *pid))
        .filter(|(title, _)| !title.is_empty())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clients = Clients::get()?.to_vec();
    let titles = get_titles(&clients);
    let entries = Vec::from_iter(titles.keys());

    let choice = rofi::Rofi::new(&entries).run()?;
    println!("Choice: {}", choice);
    let pid: u32 = titles[&choice].try_into().unwrap();

    println!("Pid: {}", pid);
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::ProcessId(pid)))?;

    Ok(())
}
