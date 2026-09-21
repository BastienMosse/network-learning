use crate::events::{StepHandle, StepReceiver};
use crate::simulation::SimulationManager;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;


pub struct StepController {
    receiver: Option<StepReceiver>,
    pending: Option<StepHandle>,
}

impl StepController {
    pub fn new(receiver: StepReceiver) -> Self {
        Self {
            receiver: Some(receiver),
            pending: None,
        }
    }

    fn clear_pending(&mut self) {
        if let Some(handle) = self.pending.take() {
            handle.resume();
        }
    }
}


#[derive(Serialize)]
pub struct StepInfo {
    pub device: String,
    pub event: String,
    pub tables: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct SimStepEvent {
    pub device: String,
    pub event: String,
    pub tables: Vec<String>,
}

#[derive(Serialize)]
pub struct AddIfaceResult {
    pub id: usize,
    pub mac: String,
}


#[tauri::command]
pub async fn sim_create_device(
    state: State<'_, Mutex<SimulationManager>>,
    name: String,
    device_type: String,
) -> Result<(), String> {
    state.lock().await.create_device(&name, &device_type)
}


#[tauri::command]
pub async fn sim_rename_device(
    state: State<'_, Mutex<SimulationManager>>,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    state.lock().await.rename_device(&old_name, &new_name)
}


#[tauri::command]
pub async fn sim_remove_device(
    state: State<'_, Mutex<SimulationManager>>,
    name: String,
) -> Result<(), String> {
    state.lock().await.remove_device(&name)
}


#[tauri::command]
pub async fn sim_add_interface(
    state: State<'_, Mutex<SimulationManager>>,
    device_name: String,
    iface_name: String,
    ip: String,
    mask: u8,
    mtu: u16,
) -> Result<AddIfaceResult, String> {
    let (id, mac) = state.lock().await.add_interface(&device_name, &iface_name, &ip, mask, mtu).await?;
    let mac_str = mac.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(":");
    Ok(AddIfaceResult { id, mac: mac_str })
}


#[tauri::command]
pub async fn sim_edit_interface(
    state: State<'_, Mutex<SimulationManager>>,
    device_name: String,
    old_iface_name: String,
    new_name: Option<String>,
    ip: Option<String>,
    mask: Option<u8>,
    mtu: Option<u16>,
) -> Result<(), String> {
    state.lock().await.edit_interface(
        &device_name,
        &old_iface_name,
        new_name.as_deref(),
        ip.as_deref(),
        mask,
        mtu,
    ).await
}


#[tauri::command]
pub async fn sim_link_interfaces(
    state: State<'_, Mutex<SimulationManager>>,
    dev_a: String,
    iface_a: String,
    dev_b: String,
    iface_b: String,
) -> Result<(), String> {
    state.lock().await.link_interfaces(&dev_a, &iface_a, &dev_b, &iface_b).await
}


#[tauri::command]
pub async fn sim_start(
    state: State<'_, Mutex<SimulationManager>>,
    app: AppHandle,
) -> Result<(), String> {
    let mut mgr = state.lock().await;
    let mut log_rx = mgr.subscribe();
    mgr.start()?;
    drop(mgr);

    tokio::spawn(async move {
        while let Ok(step) = log_rx.recv().await {
            if !step.kind.is_log_visible() {
                continue;
            }
            let payload = SimStepEvent {
                device: step.device,
                event: step.kind.to_string(),
                tables: step.tables.iter().map(|t| t.to_string()).collect(),
            };
            let _ = app.emit("sim-step", payload);
        }
    });

    Ok(())
}


#[tauri::command]
pub async fn sim_exec(
    state: State<'_, Mutex<SimulationManager>>,
    device_name: String,
    input: String,
) -> Result<String, String> {
    state.lock().await.exec(&device_name, &input).await
}


#[tauri::command]
pub async fn sim_stop(
    state: State<'_, Mutex<SimulationManager>>,
    step_ctrl: State<'_, Mutex<StepController>>,
) -> Result<(), String> {
    step_ctrl.lock().await.clear_pending();
    state.lock().await.stop();
    Ok(())
}


#[tauri::command]
pub async fn sim_reset(
    state: State<'_, Mutex<SimulationManager>>,
    step_ctrl: State<'_, Mutex<StepController>>,
) -> Result<(), String> {
    step_ctrl.lock().await.clear_pending();
    state.lock().await.reset();
    Ok(())
}


#[tauri::command]
pub async fn sim_is_running(
    state: State<'_, Mutex<SimulationManager>>,
) -> Result<bool, String> {
    Ok(state.lock().await.is_running())
}


#[tauri::command]
pub fn sim_max_interfaces(device_type: String) -> Result<usize, String> {
    let max = SimulationManager::max_interfaces(&device_type);
    if max == 0 {
        return Err(format!("type inconnu: {device_type}"));
    }
    Ok(max)
}


#[tauri::command]
pub async fn sim_set_step_mode(
    state: State<'_, Mutex<SimulationManager>>,
    enabled: bool,
) -> Result<(), String> {
    state.lock().await.set_step_mode(enabled);
    Ok(())
}


#[tauri::command]
pub async fn sim_next_step(
    step_ctrl: State<'_, Mutex<StepController>>,
) -> Result<StepInfo, String> {
    {
        let mut ctrl = step_ctrl.lock().await;
        ctrl.clear_pending();
    }

    let mut receiver = {
        let mut ctrl = step_ctrl.lock().await;
        ctrl.receiver.take().ok_or("step mode non initialisé")?
    };

    let result = receiver.next_step().await;

    let mut ctrl = step_ctrl.lock().await;
    ctrl.receiver = Some(receiver);

    match result {
        Some(handle) => {
            let info = StepInfo {
                device: handle.step.device.clone(),
                event: handle.step.kind.to_string(),
                tables: handle.step.tables.iter().map(|t| t.to_string()).collect(),
            };
            ctrl.pending = Some(handle);
            Ok(info)
        }
        None => Err("simulation terminée".into()),
    }
}
