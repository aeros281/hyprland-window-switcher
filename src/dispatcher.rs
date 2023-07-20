use hyprland::data::{Client, Clients, Monitors, WorkspaceBasic};
use hyprland::dispatch::{
    Dispatch, DispatchType, MonitorIdentifier, WindowIdentifier, WorkspaceIdentifierWithSpecial,
};
use hyprland::prelude::*;
use itertools::Itertools;
use rofi;
use std::collections::HashMap;

pub type DispatchResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn switch_window_by_titles() -> DispatchResult {
    let clients = Clients::get()?;
    let titles = get_titles(&clients);
    let entries = Vec::from_iter(titles.keys());

    let choice = rofi::Rofi::new(&entries).run()?;
    println!("Choice: {}", choice);
    let address = titles[&choice].clone();

    println!("Address: {:?}", address);
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(
        address,
    )))?;

    Ok(())
}

pub fn switch_monitor_by_index(index: usize) -> DispatchResult {
    let monitor_name = get_window_name_by_relative_index(index)?;
    Dispatch::call(DispatchType::MoveCurrentWorkspaceToMonitor(
        MonitorIdentifier::Name(&monitor_name),
    ))?;
    Ok(())
}

pub fn focus_monitor_by_index(index: usize) -> DispatchResult {
    let monitor_name = get_window_name_by_relative_index(index)?;
    Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Name(
        &monitor_name,
    )))?;
    Ok(())
}

pub fn switch_workspaces() -> DispatchResult {
    let mut workspace_hash = HashMap::new();
    workspace_hash.insert("01. Terminal".to_string(), 1);
    workspace_hash.insert("02. Browser (Firefox)".to_string(), 2);
    workspace_hash.insert("03. File manager".to_string(), 3);
    workspace_hash.insert("04. Media player".to_string(), 4);
    workspace_hash.insert("05. Git".to_string(), 5);
    workspace_hash.insert("06. none".to_string(), 6);
    workspace_hash.insert("07. none".to_string(), 7);
    workspace_hash.insert("08. none".to_string(), 8);
    workspace_hash.insert("09. Email".to_string(), 9);
    workspace_hash.insert("10. Communication (Chat, Slack)".to_string(), 10);
    let entries = Vec::from_iter(workspace_hash.keys().sorted());

    let choice = rofi::Rofi::new(&entries).run()?;
    println!("Choice: {}", choice);
    let workspace_id: i32 = workspace_hash[&choice];

    Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        workspace_id,
    )))?;
    Ok(())
}

pub fn cycle_window_with_same_class() -> DispatchResult {
    let Client {
        class: active_class,
        address: active_address,
        ..
    } = Client::get_active()?.unwrap();
    let clients = Clients::get()?
        .filter(|Client { class, .. }| *class == active_class)
        .collect::<Vec<Client>>();

    let mut next_index: usize = 0;
    for (index, c) in clients.iter().enumerate() {
        if format!("{:?}", c.address) == format!("{:?}", active_address) {
            next_index = index + 1;
        }
    }

    next_index = next_index % clients.len();
    let Client {
        address: next_address,
        ..
    } = clients.get(next_index).unwrap();

    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(
        next_address.clone(),
    )))?;
    Ok(())
}

fn get_titles(clients: &Clients) -> HashMap<&String, &hyprland::shared::Address> {
    clients
        .iter()
        .filter(|Client { title, .. }| !title.is_empty())
        .filter(
            |Client {
                 workspace: WorkspaceBasic { id, .. },
                 ..
             }| *id != -99, // id -99 is special workspace
        )
        .map(|Client { title, address, .. }| (title, address))
        .collect()
}

fn get_window_name_by_relative_index(index: usize) -> DispatchResult<String> {
    let mut monitors = Monitors::get()?.to_vec();
    monitors.sort_by_key(|m| m.x);
    let monitor = monitors.get_mut(index).unwrap();
    let monitor_name = std::mem::replace(&mut monitor.name, String::default());
    Ok(monitor_name)
}
