# Protobuf home automation experiment

This project is mostly about sending protobufs over the wire.


## Premise

I am wondering about controlling a home using a physical network of small
computers. Each of these computers (referred to as thermostats) would be able
to sense, in a minimal example, room temperature and radiator temperature (this
has some bad assumptions such as one type of radiator, rooms without any
physical attributes [size, topology, insulation]). The thermostats would also
be able to turn on and off the radiator.

The eventual goal of this would be a network-connected internet-disconnected
system that would allow anyone to efficiently, and comfortably control the
space.

## Demo

```
cargo run --bin server
cargo run --bin fake_thermostat
```

## Ideas 

### Project

- Optimize for power consumption.
- Simulate less busy rooms, rooms that get colder faster
- Record sent messages. Display trends.
- Web based UI

### Implementation

- Run a bunch of simulated thermostats in a network.
- Test that thermostat can be upgraded in a backwards compatible way.
- Simulate in a simulated faulty network.
- Add retries and exp back-off
- Try out protobufs for transmission

