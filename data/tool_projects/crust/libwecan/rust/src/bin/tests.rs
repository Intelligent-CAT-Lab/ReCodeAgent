use libwecan::libwecan::*;

const PRECISION32: f32 = 0.00001;
const PRECISION64: f64 = 0.00001;

fn display_one(frame: &[u8]) {
    print!("\nencoded frame: ");
    for byte in frame {
        print!("{:02x} ", byte);
    }
    println!();
}

fn display_two(expected: &[u8], frame: &[u8]) {
    print!("\nexpected frame: ");
    for byte in expected {
        print!("{:02x} ", byte);
    }
    println!();
    print!("frame:          ");
    for byte in frame {
        print!("{:02x} ", byte);
    }
    println!();
}

fn cmp_float(f1: f32, f2: f32) -> bool {
    (f1 - PRECISION32 < f2) && (f1 + PRECISION32 as f32 > f2)
}

fn cmp_double(d1: f64, d2: f64) -> bool {
    (d1 - PRECISION64 < d2) && (d1 + PRECISION64 > d2)
}

fn asserteq(expected: &[u8], frame: &[u8]) -> bool {
    expected.iter().zip(frame.iter()).all(|(a, b)| a == b)
}

#[test]
fn run_tests() {
    // Prepare arrays (like the C arrays)
    let mut frame = [0u8; 8];
    let mut frame_fd0 = [0u8; 40];
    let mut frame_fd1 = [0u8; 56];
    let mut frame_fd2 = [0u8; 48];
    let mut frame_fd3 = [0u8; 64];
    let mut frame_fd4 = [0u8; 24];
    let mut expected_frame = [0u8; 8];

    let mut startbit: u16;
    let mut len: u8;
    let mut value_unsigned: u64;
    let mut expected_value_unsigned: u64;
    let mut value_signed: i64;
    let mut expected_value_signed: i64;
    let mut value_one_byte_unsigned: u64;
    let mut value_one_byte_signed: i64;
    let mut value_two_bytes_unsigned: u64;
    let mut value_two_bytes_signed: i64;
    let mut value_seven_bytes_unsigned: u64;
    let mut value_four_bytes_signed: i64;
    let mut uphy_value: u64;
    let mut sphy_value: i64;
    let mut fphy_value: f32;
    let mut dphy_value: f64;
    let mut factor: f64;
    let mut offset: f64;

    println!("===========================================================");
    println!("EXTRACT MOTOROLA");
    println!("===========================================================");

    // --- Step 1.1 (1 byte, unsigned)
    startbit = 7;
    len = 8;
    expected_value_unsigned = 255;
    frame[0] = 0xFF;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, MOTOROLA);
    println!("step 1.1");
    println!("expected_value_unsigned: {}", expected_value_unsigned);
    println!("current_value:           {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 1.2 (1 byte, signed)
    startbit = 15;
    len = 8;
    expected_value_signed = -3;
    frame[1] = 0xFD;
    value_signed = extract(&frame, startbit, len, SIGNED, MOTOROLA) as i64;
    println!("\nstep 1.2");
    println!("expected_value_signed:   {}", expected_value_signed);
    println!("current_value:           {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    // --- Step 1.3 (lsb middle of byte, unsigned)
    startbit = 27;
    len = 3;
    expected_value_unsigned = 7;
    frame[3] = 0x0E;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, MOTOROLA);
    println!("\nstep 1.3");
    println!("expected_value_unsigned:   {}", expected_value_unsigned);
    println!("current_value:             {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 1.4 (lsb start of byte, unsigned)
    startbit = 21;
    len = 6;
    expected_value_unsigned = 63;
    frame[2] = 0x3F;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, MOTOROLA);
    println!("\nstep 1.4");
    println!("expected_value_unsigned:   {}", expected_value_unsigned);
    println!("current_value:             {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 1.5 (lsb start of byte, signed)
    startbit = 35;
    len = 4;
    expected_value_signed = -5;
    frame[4] = 0x0B;
    value_signed = extract(&frame, startbit, len, SIGNED, MOTOROLA) as i64;
    println!("\nstep 1.5");
    println!("expected_value_signed:     {}", expected_value_signed);
    println!("current_value:             {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    // --- Step 2.1 (2 bytes, unsigned)
    startbit = 55;
    len = 16;
    expected_value_unsigned = 52651;
    frame[6] = 0xCD;
    frame[7] = 0xAB;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, MOTOROLA);
    println!("\nstep 2.1");
    println!("expected_value_unsigned:     {}", expected_value_unsigned);
    println!("current_value:               {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 2.2 (2 bytes, signed)
    startbit = 39;
    len = 16;
    expected_value_signed = -9;
    frame[4] = 0xFF;
    frame[5] = 0xF7;
    value_signed = extract(&frame, startbit, len, SIGNED, MOTOROLA) as i64;
    println!("\nstep 2.2");
    println!("expected_value_signed:       {}", expected_value_signed);
    println!("current_value:               {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    // --- Step 2.3 (lsb middle of byte, unsigned)
    startbit = 26;
    len = 9;
    expected_value_unsigned = 511;
    frame[3] = 0x07;
    frame[4] = 0xFC;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, MOTOROLA);
    println!("\nstep 2.3");
    println!("expected_value_unsigned:       {}", expected_value_unsigned);
    println!("current_value:                 {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 2.4 (lsb start of byte, unsigned)
    startbit = 29;
    len = 14;
    expected_value_unsigned = 16383;
    frame[3] = 0x3F;
    frame[4] = 0xFF;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, MOTOROLA);
    println!("\nstep 2.4");
    println!("expected_value_unsigned:       {}", expected_value_unsigned);
    println!("current_value:                 {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 2.5 (lsb start of byte, signed)
    startbit = 18;
    len = 11;
    expected_value_signed = -789;
    frame[2] = 0x04;
    frame[3] = 0xEB;
    value_signed = extract(&frame, startbit, len, SIGNED, MOTOROLA) as i64;
    println!("\nstep 2.5");
    println!("expected_value_signed:         {}", expected_value_signed);
    println!("current_value:                 {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    // --- Step 2.6 (signal on 7 bytes, unsigned)
    startbit = 7;
    len = 56;
    expected_value_unsigned = 72057594037927935;
    for i in 0..7 {
        frame[i] = 0xFF;
    }
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, MOTOROLA);
    println!("\nstep 2.6");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 2.7 (signal on 4 bytes, signed)
    startbit = 39;
    len = 32;
    expected_value_signed = -2345634;
    frame[4] = 0xFF;
    frame[5] = 0xDC;
    frame[6] = 0x35;
    frame[7] = 0x5E;
    value_signed = extract(&frame, startbit, len, SIGNED, MOTOROLA) as i64;
    println!("\nstep 2.7");
    println!("expected_value_signed:         {}", expected_value_signed);
    println!("current_value:                 {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    println!("\n\n\n===========================================================");
    println!("EXTRACT INTEL");
    println!("===========================================================");

    // --- Step 3.1 (1 byte, unsigned, Intel)
    startbit = 0;
    len = 8;
    expected_value_unsigned = 255;
    frame[0] = 0xFF;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, INTEL);
    println!("\nstep 3.1");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 3.2 (1 byte, signed, Intel)
    startbit = 40;
    len = 8;
    expected_value_signed = -33;
    frame[5] = 0xDF;
    value_signed = extract(&frame, startbit, len, SIGNED, INTEL) as i64;
    println!("\nstep 3.2");
    println!("expected_value_signed:         {}", expected_value_signed);
    println!("current_value:                 {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    // --- Step 3.3 (lsb middle of byte, unsigned, Intel)
    startbit = 17;
    len = 7;
    expected_value_unsigned = 47;
    frame[2] = 0x5E;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, INTEL);
    println!("\nstep 3.3");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 3.4 (lsb start of byte, unsigned, Intel)
    startbit = 48;
    len = 7;
    expected_value_unsigned = 118;
    frame[6] = 0x76;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, INTEL);
    println!("\nstep 3.4");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 3.5 (lsb start of byte, signed, Intel)
    startbit = 32;
    len = 8;
    expected_value_signed = -45;
    frame[4] = 0xD3;
    value_signed = extract(&frame, startbit, len, SIGNED, INTEL) as i64;
    println!("\nstep 3.5");
    println!("expected_value_signed:         {}", expected_value_signed);
    println!("current_value:                 {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    println!("\n===========================================================");
    println!("EXTRACT INTEL 2 BYTES");
    println!("===========================================================");

    // --- Step 4.1 (2 bytes, unsigned, Intel)
    startbit = 24;
    len = 16;
    expected_value_unsigned = 53754;
    frame[3] = 0xFA;
    frame[4] = 0xD1;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, INTEL);
    println!("\nstep 4.1");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 4.2 (2 bytes, signed, Intel)
    startbit = 48;
    len = 16;
    expected_value_signed = -999;
    frame[6] = 0x19;
    frame[7] = 0xFC;
    value_signed = extract(&frame, startbit, len, SIGNED, INTEL) as i64;
    println!("\nstep 4.2");
    println!("expected_value_signed:         {}", expected_value_signed);
    println!("current_value:                   {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    // --- Step 4.3 (lsb middle of byte, unsigned, Intel)
    startbit = 2;
    len = 12;
    expected_value_unsigned = 3387;
    frame[0] = 0xEC;
    frame[1] = 0x34;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, INTEL);
    println!("\nstep 4.3");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 4.4 (lsb start of byte, unsigned, Intel)
    startbit = 16;
    len = 11;
    expected_value_unsigned = 885;
    frame[2] = 0x75;
    frame[3] = 0x03;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, INTEL);
    println!("\nstep 4.4");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 4.5 (lsb start of byte, signed, Intel)
    startbit = 40;
    len = 16;
    expected_value_signed = -6666;
    frame[5] = 0xF6;
    frame[6] = 0xE5;
    value_signed = extract(&frame, startbit, len, SIGNED, INTEL) as i64;
    println!("\nstep 4.5");
    println!("expected_value_signed:         {}", expected_value_signed);
    println!("current_value:                   {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    // --- Step 4.6 (signal on 7 bytes, unsigned, Intel)
    startbit = 0;
    len = 56;
    expected_value_unsigned = 48413335211474859;
    frame[0] = 0xAB;
    frame[1] = 0xFF;
    frame[2] = 0xAB;
    frame[3] = 0xFF;
    frame[4] = 0xAB;
    frame[5] = 0xFF;
    frame[6] = 0xAB;
    value_unsigned = extract(&frame, startbit, len, UNSIGNED, INTEL);
    println!("\nstep 4.6");
    println!(
        "expected_value_unsigned:         {}",
        expected_value_unsigned
    );
    println!("current_value:                   {}", value_unsigned);
    assert_eq!(expected_value_unsigned, value_unsigned);

    // --- Step 4.7 (signal on 4 bytes, signed, Intel)
    startbit = 0;
    len = 32;
    expected_value_signed = -1666666;
    frame[0] = 0x96;
    frame[1] = 0x91;
    frame[2] = 0xE6;
    frame[3] = 0xFF;
    value_signed = extract(&frame, startbit, len, SIGNED, INTEL) as i64;
    println!("\nstep 4.7");
    println!("expected_value_signed:         {}", expected_value_signed);
    println!("current_value:                   {}", value_signed);
    assert_eq!(expected_value_signed, value_signed);

    println!("\n\n\n===========================================================");
    println!("INSERT MOTOROLA");
    println!("===========================================================");

    // --- Step 5.1 (1 byte, unsigned, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_unsigned = 6;
    startbit = 31;
    len = 8;
    expected_frame[3] = 0x06;
    insert(&mut frame, startbit, len, value_one_byte_unsigned, MOTOROLA);
    println!("\nstep 5.1");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 5.2 (1 byte, signed, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_signed = -15;
    startbit = 31;
    len = 8;
    expected_frame[3] = 0xF1;
    insert(
        &mut frame,
        startbit,
        len,
        value_one_byte_signed as u64,
        MOTOROLA,
    );
    println!("\nstep 5.2");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 5.3 (lsb middle of byte, unsigned, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_unsigned = 63;
    startbit = 7;
    len = 6;
    expected_frame[0] = 0xFC;
    insert(&mut frame, startbit, len, value_one_byte_unsigned, MOTOROLA);
    println!("\nstep 5.3");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 5.4 (lsb start of byte, unsigned, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_unsigned = 113;
    startbit = 47;
    len = 8;
    expected_frame[5] = 0x71;
    insert(&mut frame, startbit, len, value_one_byte_unsigned, MOTOROLA);
    println!("\nstep 5.4");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 5.5 (lsb start of byte, signed, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_signed = -113;
    startbit = 23;
    len = 8;
    expected_frame[2] = 0x8F;
    insert(
        &mut frame,
        startbit,
        len,
        value_one_byte_signed as u64,
        MOTOROLA,
    );
    println!("\nstep 5.5");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    println!("\n===========================================================");
    println!("INSERT MOTOROLA 2 BYTES");
    println!("===========================================================");

    // --- Step 6.1 (2 bytes, unsigned, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_unsigned = 30126;
    startbit = 55;
    len = 16;
    expected_frame[6] = 0x75;
    expected_frame[7] = 0xAE;
    insert(
        &mut frame,
        startbit,
        len,
        value_two_bytes_unsigned,
        MOTOROLA,
    );
    println!("\nstep 6.1");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 6.2 (2 bytes, signed, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_signed = -59595;
    startbit = 39;
    len = 16;
    expected_frame[4] = 0x17;
    expected_frame[5] = 0x35;
    insert(
        &mut frame,
        startbit,
        len,
        value_two_bytes_signed as u64,
        MOTOROLA,
    );
    println!("\nstep 6.2");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 6.3 (lsb middle of byte, unsigned, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_unsigned = 189;
    startbit = 21;
    len = 9;
    expected_frame[2] = 0x17;
    expected_frame[3] = 0xA0;
    insert(
        &mut frame,
        startbit,
        len,
        value_two_bytes_unsigned,
        MOTOROLA,
    );
    println!("\nstep 6.3");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 6.4 (lsb start of byte, unsigned, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_unsigned = 1390;
    startbit = 34;
    len = 11;
    expected_frame[4] = 0x05;
    expected_frame[5] = 0x6E;
    insert(
        &mut frame,
        startbit,
        len,
        value_two_bytes_unsigned,
        MOTOROLA,
    );
    println!("\nstep 6.4");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 6.5 (lsb start of byte, signed, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_signed = -24244;
    startbit = 7;
    len = 16;
    expected_frame[0] = 0xA1;
    expected_frame[1] = 0x4C;
    insert(
        &mut frame,
        startbit,
        len,
        value_two_bytes_signed as u64,
        MOTOROLA,
    );
    println!("\nstep 6.5");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 6.6 (signal on 7 bytes, unsigned, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_seven_bytes_unsigned = 48413335211474859;
    startbit = 7;
    len = 56;
    expected_frame[0] = 0xAB;
    expected_frame[1] = 0xFF;
    expected_frame[2] = 0xAB;
    expected_frame[3] = 0xFF;
    expected_frame[4] = 0xAB;
    expected_frame[5] = 0xFF;
    expected_frame[6] = 0xAB;
    insert(
        &mut frame,
        startbit,
        len,
        value_seven_bytes_unsigned,
        MOTOROLA,
    );
    println!("\nstep 6.6");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 6.7 (signal on 4 bytes, signed, Motorola)
    frame.fill(0);
    expected_frame.fill(0);
    value_four_bytes_signed = -489;
    startbit = 39;
    len = 32;
    expected_frame[4] = 0xFF;
    expected_frame[5] = 0xFF;
    expected_frame[6] = 0xFE;
    expected_frame[7] = 0x17;
    insert(
        &mut frame,
        startbit,
        len,
        value_four_bytes_signed as u64,
        MOTOROLA,
    );
    println!("\nstep 6.7");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    println!("\n\n\n===========================================================");
    println!("INSERT INTEL");
    println!("===========================================================");

    // --- Step 7.1 (1 byte, unsigned, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_unsigned = 240;
    startbit = 16;
    len = 8;
    expected_frame[2] = 0xF0;
    insert(&mut frame, startbit, len, value_one_byte_unsigned, INTEL);
    println!("\nstep 7.1");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 7.2 (1 byte, signed, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_signed = -202;
    startbit = 32;
    len = 8;
    expected_frame[4] = 0x36;
    insert(
        &mut frame,
        startbit,
        len,
        value_one_byte_signed as u64,
        INTEL,
    );
    println!("\nstep 7.2");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 7.3 (lsb middle of byte, unsigned, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_unsigned = 7;
    startbit = 29;
    len = 3;
    expected_frame[3] = 0xE0;
    insert(&mut frame, startbit, len, value_one_byte_unsigned, INTEL);
    println!("\nstep 7.3");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 7.4 (lsb start of byte, unsigned, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_unsigned = 23;
    startbit = 56;
    len = 5;
    expected_frame[7] = 0x17;
    insert(&mut frame, startbit, len, value_one_byte_unsigned, INTEL);
    println!("\nstep 7.4");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 7.5 (lsb start of byte, signed, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_one_byte_signed = -199;
    startbit = 40;
    len = 8;
    expected_frame[5] = 0x39;
    insert(
        &mut frame,
        startbit,
        len,
        value_one_byte_signed as u64,
        INTEL,
    );
    println!("\nstep 7.5");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    println!("\n===========================================================");
    println!("INSERT INTEL 2 BYTES");
    println!("===========================================================");

    // --- Step 8.1 (2 bytes, unsigned, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_unsigned = 52077;
    startbit = 16;
    len = 16;
    expected_frame[2] = 0x6D;
    expected_frame[3] = 0xCB;
    insert(&mut frame, startbit, len, value_two_bytes_unsigned, INTEL);
    println!("\nstep 8.1");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 8.2 (2 bytes, signed, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_signed = -48666;
    startbit = 32;
    len = 16;
    expected_frame[4] = 0xE6;
    expected_frame[5] = 0x41;
    insert(
        &mut frame,
        startbit,
        len,
        value_two_bytes_signed as u64,
        INTEL,
    );
    println!("\nstep 8.2");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 8.3 (lsb middle of byte, unsigned, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_unsigned = 1707;
    startbit = 44;
    len = 11;
    expected_frame[5] = 0xB0;
    expected_frame[6] = 0x6A;
    insert(&mut frame, startbit, len, value_two_bytes_unsigned, INTEL);
    println!("\nstep 8.3");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 8.4 (lsb start of byte, unsigned, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_unsigned = 1023;
    startbit = 8;
    len = 10;
    expected_frame[1] = 0xFF;
    expected_frame[2] = 0x03;
    insert(&mut frame, startbit, len, value_two_bytes_unsigned, INTEL);
    println!("\nstep 8.4");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 8.5 (lsb start of byte, signed, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_two_bytes_signed = -59821;
    startbit = 48;
    len = 16;
    expected_frame[6] = 0x53;
    expected_frame[7] = 0x16;
    insert(
        &mut frame,
        startbit,
        len,
        value_two_bytes_signed as u64,
        INTEL,
    );
    println!("\nstep 8.5");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 8.6 (signal on 7 bytes, unsigned, Intel)
    frame.fill(0);
    expected_frame.fill(0);
    value_seven_bytes_unsigned = 48413335211474859;
    startbit = 0;
    len = 56;
    expected_frame[0] = 0xAB;
    expected_frame[1] = 0xFF;
    expected_frame[2] = 0xAB;
    expected_frame[3] = 0xFF;
    expected_frame[4] = 0xAB;
    expected_frame[5] = 0xFF;
    expected_frame[6] = 0xAB;
    insert(&mut frame, startbit, len, value_seven_bytes_unsigned, INTEL);
    println!("\nstep 8.6");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    // --- Step 8.7 (signal on 4 bytes, signed, Intel)
    // Note: In the original C code this step used MOTOROLA ordering.
    frame.fill(0);
    expected_frame.fill(0);
    value_four_bytes_signed = -1339;
    startbit = 7;
    len = 32;
    expected_frame[0] = 0xFF;
    expected_frame[1] = 0xFF;
    expected_frame[2] = 0xFA;
    expected_frame[3] = 0xC5;
    insert(
        &mut frame,
        startbit,
        len,
        value_four_bytes_signed as u64,
        MOTOROLA,
    );
    println!("\nstep 8.7");
    display_two(&expected_frame, &frame);
    assert!(asserteq(&expected_frame, &frame));

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE MOTOROLA double value");
    println!("===========================================================");

    frame.fill(0);
    dphy_value = 66.66666;
    startbit = 7;
    len = 32;
    factor = 0.0000001;
    offset = 0.0;
    println!("\nstep 9.1\n");
    println!("value to be encoded: {:.5}", dphy_value);
    encode_double(
        &mut frame, dphy_value, startbit, len, MOTOROLA, factor, offset,
    );
    display_one(&frame);
    println!(
        "\ndecoded value:       {:.5}",
        decode_double(&frame, startbit, len, MOTOROLA, factor, offset)
    );
    assert!(cmp_double(
        decode_double(&frame, startbit, len, MOTOROLA, factor, offset),
        dphy_value
    ));

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE MOTOROLA double negative value");
    println!("===========================================================");

    frame.fill(0);
    dphy_value = -50.6164129;
    startbit = 7;
    len = 32;
    factor = 0.0000001;
    offset = 0.0;
    println!("\nstep 9.2\n");
    println!("value to be encoded: {:.7}", dphy_value);
    encode_double(
        &mut frame, dphy_value, startbit, len, MOTOROLA, factor, offset,
    );
    display_one(&frame);
    println!(
        "\ndecoded value:       {:.7}",
        decode_double(&frame, startbit, len, MOTOROLA, factor, offset)
    );
    assert!(cmp_double(
        decode_double(&frame, startbit, len, MOTOROLA, factor, offset),
        dphy_value
    ));

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE MOTOROLA unsigned int value");
    println!("===========================================================");

    frame.fill(0);
    uphy_value = 666666666;
    startbit = 7;
    len = 32;
    factor = 1.0;
    offset = 0.0;
    println!("\nstep 9.3\n");
    println!("value to be encoded: {}", uphy_value);
    encode_uint64_t(
        &mut frame, uphy_value, startbit, len, MOTOROLA, factor, offset,
    );
    display_one(&frame);
    println!(
        "\ndecoded value:       {}",
        decode_uint64_t(&frame, startbit, len, MOTOROLA, factor, offset)
    );
    assert_eq!(
        uphy_value,
        decode_uint64_t(&frame, startbit, len, MOTOROLA, factor, offset)
    );

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE INTEL double value");
    println!("===========================================================");

    frame.fill(0);
    dphy_value = 8.4939123;
    startbit = 0;
    len = 32;
    factor = 0.0000001;
    offset = 0.0;
    println!("\nstep 9.4\n");
    println!("value to be encoded: {:.7}", dphy_value);
    encode_double(&mut frame, dphy_value, startbit, len, INTEL, factor, offset);
    display_one(&frame);
    println!(
        "\ndecoded value:       {:.7}",
        decode_double(&frame, startbit, len, INTEL, factor, offset)
    );
    assert!(cmp_double(
        decode_double(&frame, startbit, len, INTEL, factor, offset),
        dphy_value
    ));

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE INTEL double negative value");
    println!("===========================================================");

    frame.fill(0);
    dphy_value = -7.7979897;
    startbit = 0;
    len = 32;
    factor = 0.0000001;
    offset = 0.0;
    println!("\nstep 9.5\n");
    println!("value to be encoded: {:.7}", dphy_value);
    encode_double(&mut frame, dphy_value, startbit, len, INTEL, factor, offset);
    display_one(&frame);
    println!(
        "\ndecoded value:       {:.7}",
        decode_double(&frame, startbit, len, INTEL, factor, offset)
    );
    assert!(cmp_double(
        decode_double(&frame, startbit, len, INTEL, factor, offset),
        dphy_value
    ));

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE INTEL unsigned int value");
    println!("===========================================================");

    frame.fill(0);
    uphy_value = 999999999;
    startbit = 0;
    len = 32;
    factor = 1.0;
    offset = 0.0;
    println!("\nstep 9.6\n");
    println!("value to be encoded: {}", uphy_value);
    encode_uint64_t(&mut frame, uphy_value, startbit, len, INTEL, factor, offset);
    display_one(&frame);
    println!(
        "\ndecoded value:       {}",
        decode_uint64_t(&frame, startbit, len, INTEL, factor, offset)
    );
    assert_eq!(
        uphy_value,
        decode_uint64_t(&frame, startbit, len, INTEL, factor, offset)
    );

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE INTEL signed int negative value");
    println!("===========================================================");

    frame.fill(0);
    sphy_value = -1029384756;
    startbit = 0;
    len = 32;
    factor = 1.0;
    offset = 0.0;
    println!("\nstep 9.7\n");
    println!("value to be encoded: {}", sphy_value);
    encode_int64_t(&mut frame, sphy_value, startbit, len, INTEL, factor, offset);
    display_one(&frame);
    println!(
        "\ndecoded value:       {}",
        decode_int64_t(&frame, startbit, len, INTEL, factor, offset)
    );
    assert_eq!(
        sphy_value,
        decode_int64_t(&frame, startbit, len, INTEL, factor, offset)
    );

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE MOTOROLA float negative value");
    println!("===========================================================");

    frame.fill(0);
    fphy_value = -2938.345666;
    startbit = 7;
    len = 40;
    factor = 0.0000001;
    offset = 0.0;
    println!("\nstep 9.8\n");
    println!("value to be encoded: {:.6}", fphy_value);
    encode_float(
        &mut frame, fphy_value, startbit, len, MOTOROLA, factor, offset,
    );
    display_one(&frame);
    println!(
        "\ndecoded value:       {:.6}",
        decode_float(&frame, startbit, len, MOTOROLA, factor, offset)
    );
    assert!(cmp_float(
        decode_float(&frame, startbit, len, MOTOROLA, factor, offset),
        fphy_value
    ));

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE INTEL unsigned int value FDFRAME");
    println!("===========================================================");

    frame_fd0.fill(0);
    uphy_value = 999999999;
    startbit = 288;
    len = 32;
    factor = 1.0;
    offset = 0.0;
    println!("\nstep 9.9\n");
    println!("value to be encoded: {}", uphy_value);
    encode_uint64_t(
        &mut frame_fd0,
        uphy_value,
        startbit,
        len,
        INTEL,
        factor,
        offset,
    );
    display_one(&frame_fd0);
    println!(
        "\ndecoded value:       {}",
        decode_uint64_t(&frame_fd0, startbit, len, INTEL, factor, offset)
    );
    assert_eq!(
        uphy_value,
        decode_uint64_t(&frame_fd0, startbit, len, INTEL, factor, offset)
    );

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE MOTOROLA signed int value FDFRAME");
    println!("===========================================================");

    frame_fd1.fill(0);
    sphy_value = -7777;
    startbit = 431;
    len = 16;
    factor = 1.0;
    offset = 0.0;
    println!("\nstep 10.0\n");
    println!("value to be encoded: {}", sphy_value);
    encode_int64_t(
        &mut frame_fd1,
        sphy_value,
        startbit,
        len,
        MOTOROLA,
        factor,
        offset,
    );
    display_one(&frame_fd1);
    println!(
        "\ndecoded value:       {}",
        decode_int64_t(&frame_fd1, startbit, len, MOTOROLA, factor, offset)
    );
    assert_eq!(
        sphy_value,
        decode_int64_t(&frame_fd1, startbit, len, MOTOROLA, factor, offset)
    );

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE INTEL signed negative int value FDFRAME");
    println!("===========================================================");

    frame_fd2.fill(0);
    sphy_value = -1029384756;
    startbit = 184;
    len = 32;
    factor = 1.0;
    offset = 0.0;
    println!("\nstep 10.1\n");
    println!("value to be encoded: {}", sphy_value);
    encode_int64_t(
        &mut frame_fd2,
        sphy_value,
        startbit,
        len,
        INTEL,
        factor,
        offset,
    );
    display_one(&frame_fd2);
    println!(
        "\ndecoded value:       {}",
        decode_int64_t(&frame_fd2, startbit, len, INTEL, factor, offset)
    );
    assert_eq!(
        sphy_value,
        decode_int64_t(&frame_fd2, startbit, len, INTEL, factor, offset)
    );

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE MOTOROLA float value FDFRAME");
    println!("===========================================================");

    frame_fd3.fill(0);
    fphy_value = 8.49391;
    startbit = 383;
    len = 32;
    factor = 0.0000001;
    offset = 0.0;
    println!("\nstep 10.2\n");
    println!("value to be encoded: {}", fphy_value);
    encode_float(
        &mut frame_fd3,
        fphy_value,
        startbit,
        len,
        MOTOROLA,
        factor,
        offset,
    );
    display_one(&frame_fd3);
    println!(
        "\ndecoded value:       {}",
        decode_float(&frame_fd3, startbit, len, MOTOROLA, factor, offset)
    );
    assert!(cmp_float(
        decode_float(&frame_fd3, startbit, len, MOTOROLA, factor, offset),
        fphy_value
    ));

    println!("\n\n\n===========================================================");
    println!("ENCODE DECODE INTEL double negative value FDFRAME");
    println!("===========================================================");

    frame_fd4.fill(0);
    dphy_value = -7.7979897;
    startbit = 32;
    len = 32;
    factor = 0.0000001;
    offset = 0.0;
    println!("\nstep 10.3\n");
    println!("value to be encoded: {:.7}", dphy_value);
    encode_double(
        &mut frame_fd4,
        dphy_value,
        startbit,
        len,
        INTEL,
        factor,
        offset,
    );
    display_one(&frame_fd4);
    println!(
        "\ndecoded value:       {:.7}",
        decode_double(&frame_fd4, startbit, len, INTEL, factor, offset)
    );
    assert!(cmp_double(
        decode_double(&frame_fd4, startbit, len, INTEL, factor, offset),
        dphy_value
    ));
}
pub fn main(){}