use random::Random;

#[test]
fn test_next_int64() {
    let seed = [1u8, 2u8, 3u8, 4u8, 1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());
    assert_eq!(8, random.size());
    assert_eq!(289077004467372545, random.next_int::<u64>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int32() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());
    // 100 00000011 00000010 00000001
    assert_eq!(67305985, random.next_int::<u32>());
    assert_eq!(0, random.size());
}

#[test]
fn test_next_int16() {
    let seed = [1u8, 2u8, 3u8, 4u8];
    let mut random = Random::new(&seed.to_vec());
    // 100 00000011 00000010 00000001
    assert_eq!(513, random.next_int::<u16>());
    assert_eq!(1027, random.next_int::<u16>());
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
    assert_eq!(19, random.roll::<u16>(24));
    assert_eq!(85, random.roll::<u32>(100));
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

    // 0100 00000011 00000010 00000001
    assert_eq!(67305985, random.next_int::<u32>());
    // 1000 00000111 00000110 00000101
    assert_eq!(134678021, random.next_int::<u32>());
    // 1100 00001011 00001010 00001001
    assert_eq!(202050057, random.next_int::<u32>());
    assert_eq!(0, random.size());
    assert_eq!(0, random.rotations());
    println!("{:?}", random.seed());

    // rotation by 11 w/o Lehman
    // // 0101 00000100 00000011 00000010
    // assert_eq!(84148994, random.next_int::<u32>());

    // just a long deterministic sequence of numbers
    assert_eq!(2760890918, random.next_int::<u32>());
    assert_eq!(3586852952, random.next_int::<u32>());
    assert_eq!(1929852809, random.next_int::<u32>());
    assert_eq!(1, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(2251998094, random.next_int::<u32>());
    assert_eq!(2368227722, random.next_int::<u32>());
    assert_eq!(2424807888, random.next_int::<u32>());
    assert_eq!(2, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(1867678831, random.next_int::<u32>());
    assert_eq!(1444310146, random.next_int::<u32>());
    assert_eq!(573580184, random.next_int::<u32>());
    assert_eq!(3, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(2382462553, random.next_int::<u32>());
    assert_eq!(1939739702, random.next_int::<u32>());
    assert_eq!(3370680583, random.next_int::<u32>());
    assert_eq!(4, random.rotations());
    println!("{:?}", random.seed());

    assert_eq!(0, random.size());
}


