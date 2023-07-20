use clap::Args;
use clap::Parser;

#[derive(Parser)]
pub enum Cli {
    SwitchWindowByTitle,
    SwitchWorkspace,
    CycleWindowWithSameClass,
    MoveWorkspaceToMonitor(MonitorIndex),
    FocusMonitor(MonitorIndex),
}

#[derive(Args)]
pub struct MonitorIndex {
    pub index: usize,
}
