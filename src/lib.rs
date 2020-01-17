pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub fn create_thermostat_state(name: String) -> messages::ThermostatState {
    let mut state = messages::ThermostatState::default();
    state.name = name;
    state
}
