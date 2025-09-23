use std::{env, process};

use libvcd::vcd::*;

pub fn print_vcd(vcd: &VCD) {
    println!(
        "{{\n\tdate=\"{}\",\n\tversion=\"{}\",\n\ttimescale= {{\n\t\tunit=\"{}\",\n\t\tscale=\"{}\"\n\t}},\n\tsignal= {{",
        String::from_utf8_lossy(&vcd.date),
        String::from_utf8_lossy(&vcd.version),
        String::from_utf8_lossy(&vcd.timescale.unit),
        vcd.timescale.scale
    );

    for signal in &vcd.signals {
        println!(
            "\t\t{}= {{\n\t\t\tsize={},\n\t\t\tchanges= {{",
            String::from_utf8_lossy(&signal.name),
            signal.size
        );

        for value_change in &signal.value_changes {
            println!(
                "\t\t\t\t{{\n\t\t\t\t\ttimestamp={},\n\t\t\t\t\tvalue={}\n\t\t\t\t}},",
                value_change.timestamp,
                String::from_utf8_lossy(&value_change.value)
            );
        }

        println!("\t\t\t}},\n\t\t}},");
    }
    println!("\t}}\n}}");
}

#[test]
fn test_ram_vcd() {
    // 1. Read the VCD (adjust path as needed).
    let vcd_file_path = "src/bin/assets/ram.vcd";
    let vcd = VCD::read_from_path(vcd_file_path)
        .expect("Failed to read the 'ram.vcd' file");

    // 2. Assert the date field matches your VCD content
    let date_str = String::from_utf8_lossy(&vcd.date);
    assert_eq!(
        date_str, 
        "Fri Jul 15 15:17:36 2022", 
        "Expected date to match 'Fri Jul 15 15:17:36 2022', got '{}'",
        date_str
    );

    // 3. Assert the version field
    let version_str = String::from_utf8_lossy(&vcd.version);
    assert_eq!(
        version_str, 
        "Icarus Verilog", 
        "Expected version to match 'Icarus Verilog', got '{}'",
        version_str
    );

    // 4. Assert timescale fields
    let unit_str = String::from_utf8_lossy(&vcd.timescale.unit);
    assert_eq!(
        unit_str, 
        "s",
        "Expected timescale unit to be 's', got '{}'",
        unit_str
    );
    assert_eq!(
        vcd.timescale.scale, 
        1, 
        "Expected timescale scale=1, got '{}'",
        vcd.timescale.scale
    );

    // 5. Check that we have some expected signals by name.
    //    For example, from your VCD, we see "matched [8:0]" was declared.
    //    The library’s `get_signal_by_name` will only work if the parser
    //    sets signal.name == "matched [8:0]". If not sure, print them all first.
    let matched_signal_name = "matched [8:0]";
    let matched_signal = vcd
        .get_signal_by_name(matched_signal_name)
        .unwrap_or_else(|| panic!("Expected to find signal '{}'", matched_signal_name));

    // Confirm size is 9 bits
    assert_eq!(
        matched_signal.size, 
        9,
        "Expected signal '{}' to have size=9 bits, found '{}'",
        matched_signal_name,
        matched_signal.size
    );

    // 6. Check a specific timestamp for a known value in the 'matched [8:0]' signal.
    //    From your raw VCD snippet, at #25 we see `b10010 !` which might be 5 bits.
    //    Some libraries zero-extend (e.g., "00010010"), others store as "10010".
    //    Let's assume it’s literally "10010" for this test:
    let timestamp_25 = 25;
    let val_25 = matched_signal
        .get_value_at_timestamp(timestamp_25)
        .unwrap_or_else(|| {
            panic!("No value found for '{}' at timestamp {}", matched_signal_name, timestamp_25)
        });
    let val_25_str = String::from_utf8_lossy(val_25);
    assert_eq!(
        val_25_str, 
        "10010",
        "At t=#25, expected '{}' to be '10010', got '{}'",
        matched_signal_name,
        val_25_str
    );

    // 7. If you want to test other signals like 'clock', 'reset', or 'mask [7:0]',
    //    simply replicate the pattern:
    let clock_signal = vcd
        .get_signal_by_name("clock")
        .expect("Should find 'clock' in the VCD");

    // For instance, check that at t=#5, 'clock' is '1'
    let timestamp_5 = 5;
    let clock_val_5 = clock_signal
        .get_value_at_timestamp(timestamp_5)
        .expect("No 'clock' value at #5");
    let clock_val_5_str = String::from_utf8_lossy(clock_val_5);
    assert_eq!(
        clock_val_5_str, 
        "1",
        "Expected 'clock' at t=#5 to be '1', got '{}'",
        clock_val_5_str
    );

    // 8. Optionally, print the parsed VCD for debug info
    print_vcd(&vcd);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() % 2 == 1 {
        eprintln!("Usage: test <vcd-file> [signal-name timestamp] ...");
        process::exit(1);
    }

    let vcd_file_path = &args[1];
    let vcd = match VCD::read_from_path(vcd_file_path) {
        Ok(vcd) => vcd,
        Err(_) => {
            eprintln!("Could not read the VCD");
            process::exit(1);
        }
    };

    print_vcd(&vcd);
    println!();

    for i in (2..args.len()).step_by(2) {
        let signal_name = &args[i];
        let timestamp: Timestamp = match args[i + 1].parse() {
            Ok(ts) => ts,
            Err(_) => {
                eprintln!("Invalid timestamp: {}", args[i + 1]);
                process::exit(1);
            }
        };

        if let Some(signal) = vcd.get_signal_by_name(signal_name) {
            if let Some(value) = signal.get_value_at_timestamp(timestamp) {
                println!(
                    "{} at {} equals {}",
                    signal_name,
                    timestamp,
                    String::from_utf8_lossy(value)
                );
            } else {
                println!("{} at {} not found in VCD", signal_name, timestamp);
            }
        } else {
            println!("Signal '{}' not found in VCD", signal_name);
        }
    }
}
