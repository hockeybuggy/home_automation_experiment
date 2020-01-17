pub mod items {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub fn create_thermostat_state(name: String) -> items::ThermostatState {
    let mut state = items::ThermostatState::default();
    state.name = name;
    state
}
