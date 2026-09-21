pub mod devices;
pub mod events;
pub mod utils;
pub mod net;
pub mod simulation;
pub mod commands;

use simulation::SimulationManager;
use commands::*;

pub fn run() {
    let (manager, step_receiver) = SimulationManager::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(tokio::sync::Mutex::new(manager))
        .manage(tokio::sync::Mutex::new(StepController::new(step_receiver)))
        .invoke_handler(tauri::generate_handler![
            sim_create_device,
            sim_rename_device,
            sim_remove_device,
            sim_add_interface,
            sim_edit_interface,
            sim_link_interfaces,
            sim_start,
            sim_exec,
            sim_stop,
            sim_reset,
            sim_is_running,
            sim_max_interfaces,
            sim_set_step_mode,
            sim_next_step,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
