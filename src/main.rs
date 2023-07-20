use clap::Parser;
use cli::{Cli, MonitorIndex};
use dispatcher::DispatchResult;

mod cli;
mod dispatcher;

fn main() -> DispatchResult {
    match cli::Cli::parse() {
        Cli::SwitchWindowByTitle => dispatcher::switch_window_by_titles(),
        Cli::SwitchWorkspace => dispatcher::switch_workspaces(),
        Cli::CycleWindowWithSameClass => Ok(dispatcher::cycle_window_with_same_class()?),
        Cli::MoveWorkspaceToMonitor(MonitorIndex { index }) => {
            Ok(dispatcher::switch_monitor_by_index(index)?)
        }
        Cli::FocusMonitor(MonitorIndex { index }) => dispatcher::focus_monitor_by_index(index),
    }
}
