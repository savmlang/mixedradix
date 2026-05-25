# Mixed Radix Macros

Mixed Radix System Macros

This crate provides Mixed Radix Macros to create, edit and use macros for Mixed Radix system.

## Example

### Defining a mixed radix field

```rust
use mixedradix::mixedradix;
mixedradix::mixedradix! {
  #[bits(5)] //<- Specifies how many bits to use to represent the data
  #[derive(Debug, Clone, Copy, PartialEq, Eq)] //<- Any attributes you would like to give to the structure
  #[my_attr(...)]
  pub struct Controller {
    // Define your fields with
    // <visbility> field <field_name>: <total_possible_states>
    pub button_a: 2, // 0 or 1 (2 states)
    pub joy_x: 3,    // 0, 1, or 2 (3 states)
    pub joy_y: 5,    // 0 to 4 (5 states)
  }
}
```

### Usage

```rust
use mixedradix::MixedRadixStructure;
let state = Controller {
  button_a: 0,
  joy_x: 0,
  joy_y: 0,
};

// Quickly export to MixedRadix
let controller_state: u8 = state.serialize();

// Safer Version
if let Some(safe_compressed) = state.try_serialize() {
  println!("Packed into integer value: {}", safe_compressed);
}

// Construct the controller back again!
// On Debug mode, it panics on overflow
let state_back = Controller::deserialize(controller_state);

// Safer Fallible deserializer
// Returns None if total exceeds maximum possible states
if let Some(safe_unpacked) = Controller::try_deserialize(compressed) {
  assert_eq!(state, safe_unpacked);
}
```
