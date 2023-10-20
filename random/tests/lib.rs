use random::Random;

#[test]
fn test_next_int64() {
    let seed = [1u8, 2u8, 3u8, 4u8, 1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());
    assert_eq!(8, random.size());
    assert_eq!(0x0102030401020304, random.next_int::<u64>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int32() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());
    assert_eq!(0x01020304, random.next_int::<u32>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int16() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());
    assert_eq!(0x0102, random.next_int::<u16>());
    assert_eq!(0x0304, random.next_int::<u16>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int8() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());

    assert_eq!(1u8, random.next_int::<u8>());
    assert_eq!(2u8, random.next_int::<u8>());
    assert_eq!(3u8, random.next_int::<u8>());
    assert_eq!(1, random.size());
}

#[test]
fn test_next_bool() {
    let seed = [1u8, 2u8];
    let mut random = Random::new(&seed.to_vec());

    assert_eq!(true, random.next_bool());
    assert_eq!(false, random.next_bool());
    assert_eq!(0, random.size());
}

#[test]
fn test_roll() {
    let seed = [1u8, 2u8, 3u8, 4u8, 1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());

    assert_eq!(1, random.roll::<u8>(5));
    assert_eq!(0, random.roll::<u8>(2));
    assert_eq!(16, random.roll::<u16>(21));
    assert_eq!(60, random.roll::<u32>(100));
    assert_eq!(0, random.size());
}

#[test]
fn test_in_range() {
    let seed = [0, 1, 2];
    let mut random = Random::new(&seed.to_vec());
    assert_eq!(6, random.in_range::<u8>(6, 5));
    assert_eq!(6, random.in_range::<u8>(6, 6));
    assert_eq!(6, random.in_range::<u8>(6, 7));
    assert_eq!(6, random.in_range::<u8>(6, 8));
    assert_eq!(7, random.in_range::<u8>(6, 8));
    assert_eq!(6, random.in_range::<u8>(6, 8));

    assert_eq!(0, random.size());
}

#[test]
fn test_roll_uniformity() {
    let seed = [1u8, 240u8, 255u8, 199u8];
    let mut random = Random::new(&seed.to_vec());

    assert_eq!(1, random.roll::<u8>(200));
    assert_eq!(199, random.roll::<u8>(200));
    assert_eq!(0, random.size());
}

#[test]
fn test_roll_uniformity_edge_cases() {
    let seed = [255, 240, 199, 200, 3, 255, 255, 255, 255];
    let mut random = Random::new(&seed.to_vec());

    assert_eq!(240, random.roll::<u8>(255));
    assert_eq!(0, random.roll::<u8>(1));
    assert_eq!(199, random.roll::<u8>(200));
    assert_eq!(3, random.roll::<u8>(100));
    assert_eq!(1, random.roll::<u8>(2));
    assert_eq!(3, random.roll::<u8>(4));
    assert_eq!(15, random.roll::<u8>(16));
    assert_eq!(127, random.roll::<u8>(128));
    assert_eq!(0, random.size());
}

#[test]
fn test_rotate() {
    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut random = Random::new(&seed.to_vec());

    assert_eq!(0x01020304, random.next_int::<u32>());
    assert_eq!(0x05060708, random.next_int::<u32>());
    assert_eq!(0x090A0B0C, random.next_int::<u32>());
    assert_eq!(0, random.size());
    assert_eq!(0, random.rotations());
    println!("{:?}", random.seed());

    // rotation by 11 w/o Lehman
    // // 0101 00000100 00000011 00000010
    // assert_eq!(84148994, random.next_int::<u32>());

    // just a long deterministic sequence of numbers
    assert_eq!(0x9F560A78, random.next_int::<u32>());
    assert_eq!(0xCD873BB4, random.next_int::<u32>());
    assert_eq!(0xFBB86C3C, random.next_int::<u32>());
    assert_eq!(1, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(0xF9BF308C, random.next_int::<u32>());
    assert_eq!(0x89621E84, random.next_int::<u32>());
    assert_eq!(0x85800B08, random.next_int::<u32>());
    assert_eq!(2, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(0xEE72E9BC, random.next_int::<u32>());
    assert_eq!(0xFB37CC78, random.next_int::<u32>());
    assert_eq!(0x9A09A34, random.next_int::<u32>());
    assert_eq!(3, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(0xA0D69508, random.next_int::<u32>());
    assert_eq!(0x57534E0C, random.next_int::<u32>());
    assert_eq!(0x53F45104, random.next_int::<u32>());
    assert_eq!(4, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(0, random.size());
}


