use mixedradix_macros::mixedradix;

mixedradix! {
  #[bits(5)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Controller {
    pub field button_a: 2, // 0 or 1 (2 states)
    pub field joy_x: 3,    // 0, 1, or 2 (3 states)
    pub field joy_y: 5,    // 0 to 4 (5 states)
  }

  #[bits(64)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Controller2 {
    pub field a: 4294967296,
    pub field b: 2147483648,
    pub field c: 2,
  }

  #[bits(64)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Max64BitPack {
    pub field a: 65536, // States: 2^16 (0..=65535) -> Fits in u16
    pub field b: 65536, // States: 2^16
    pub field c: 65536, // States: 2^16
    pub field d: 65536, // States: 2^16
  }
}

#[test]
fn test_u32_u31_u1() {
  let state = Controller2 {
    a: 65535,
    b: 10,
    c: 1,
  };

  let packed: u64 = state.serialize();

  let unpacked = Controller2::deserialize(packed);
  assert_eq!(state, unpacked);
}

#[test]
fn test_generic_u64() {
  let state = Max64BitPack {
    a: 65535,
    b: 10,
    c: 65535,
    d: 65535,
  };

  let packed: u64 = state.serialize();

  let unpacked = Max64BitPack::deserialize(packed);
  assert_eq!(state, unpacked);
}

#[test]
fn test_zero_values() {
  let state = Controller {
    button_a: 0,
    joy_x: 0,
    joy_y: 0,
  };

  let packed = state.serialize();
  assert_eq!(packed, 0);

  let unpacked = Controller::deserialize(packed);
  assert_eq!(state, unpacked);
}
