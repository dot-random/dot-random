use random::Random;

#[test]
fn test_next_int64() {
    let seed = [1u8, 2u8, 3u8, 4u8, 1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed);
    assert_eq!(8, random.size());
    assert_eq!(289077004467372545, random.next_int::<u64>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int32() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed);
    // 100 00000011 00000010 00000001
    assert_eq!(67305985, random.next_int::<u32>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int16() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed);
    // 100 00000011 00000010 00000001
    assert_eq!(513, random.next_int::<u16>());
    assert_eq!(1027, random.next_int::<u16>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int8() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed);
    // 100 00000011 00000010 00000001
    assert_eq!(1u8, random.next_int::<u8>());
    assert_eq!(2u8, random.next_int::<u8>());
    assert_eq!(3u8, random.next_int::<u8>());
    assert_eq!(1, random.size());
}

#[test]
fn test_next_bool() {
    let seed = [1u8, 2u8];
    let mut random = Random::new(&seed);
    // 100 00000011 00000010 00000001
    assert_eq!(true, random.next_bool());
    assert_eq!(false, random.next_bool());
    assert_eq!(0, random.size());
}

#[test]
fn test_roll() {
    let seed = [1u8, 2u8, 3u8, 4u8, 1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed);
    assert_eq!(1, random.roll::<u8>(5));
    assert_eq!(0, random.roll::<u8>(2));
    assert_eq!(19, random.roll::<u16>(24));
    assert_eq!(85, random.roll::<u32>(100));
    assert_eq!(0, random.size());
}