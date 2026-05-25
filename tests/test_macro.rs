use mixedradix::{MixedRadixStructure, mixedradix};

mixedradix! {
  #[bits(5)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Controller {
    pub button_a: 2, // 0 or 1 (2 states)
    pub joy_x: 3,    // 0, 1, or 2 (3 states)
    pub joy_y: 5,    // 0 to 4 (5 states)
  }

  #[bits(64)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Controller2 {
    pub a: 4294967296,
    pub b: 2147483648,
    pub c: 2,
  }

  #[bits(64)]
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Max64BitPack {
    pub a: 65536, // States: 2^16 (0..=65535) -> Fits in u16
    pub b: 65536, // States: 2^16
    pub c: 65536, // States: 2^16
    pub d: 65536, // States: 2^16
  }
}

#[test]
fn test_u32_u31_u1() {
  let state = Controller2 {
    a: 65535,
    b: 10,
    c: 1,
  };

  let packed: u64 = state.bits();

  let unpacked = Controller2::from_bits(packed);
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

  let packed: u64 = state.bits();

  let unpacked = Max64BitPack::from_bits(packed);
  assert_eq!(state, unpacked);
}

#[test]
fn test_zero_values() {
  let state = Controller {
    button_a: 0,
    joy_x: 0,
    joy_y: 0,
  };

  let packed = state.bits();
  assert_eq!(packed, 0);

  let unpacked = Controller::from_bits(packed);
  assert_eq!(state, unpacked);
}
