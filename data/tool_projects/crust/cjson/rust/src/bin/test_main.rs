use std::fs::File;
use std::io::Read;
use std::path::Path;

use cJSON::cjson::{parse, CJson};

/// The Rust equivalent of the C `doit` function.
/// It parses the input text, prints an error if parsing fails,
/// otherwise prints out the re–serialized JSON.
fn doit(text: &str) {
    match parse(text, false) {
        Ok(json) => {
            let out = json.print_formatted(); // formatted “pretty” output
            println!("{}", out);
        }
        Err(e) => {
            // In cJSON, we might have used cJSON_GetErrorPtr(). In our Rust version,
            // the error object provides context (pos, etc.). We'll print the error:
            println!("Error: {}", e);
        }
    }
}

/// The Rust equivalent of reading from a file, then calling `doit`.
/// This is commented out in the original C code, so we show it but do not call it by default.
fn dofile(filename: &str) {
    let path = Path::new(filename);
    let mut file = File::open(path).expect("Failed to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");
    doit(&data);
}

/// A helper struct mirroring the `struct record` in the C code.
#[derive(Debug)]
struct Record {
    precision: &'static str,
    lat: f64,
    lon: f64,
    address: &'static str,
    city: &'static str,
    state: &'static str,
    zip: &'static str,
    country: &'static str,
}

/// Replicates the logic of C's `create_objects` function:
/// building a variety of JSON objects and arrays and printing them.
fn create_objects() {
    // The "days of the week" array:
    let strings = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // A 3×3 matrix of ints:
    let numbers = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];

    // A "gallery" item:
    let ids = [116, 943, 234, 38793];

    // Our array of "records":
    let fields = [
        Record {
            precision: "zip",
            lat: 37.7668,
            lon: -122.3959,
            address: "",
            city: "SAN FRANCISCO",
            state: "CA",
            zip: "94107",
            country: "US",
        },
        Record {
            precision: "zip",
            lat: 37.371991,
            lon: -122.026020,
            address: "",
            city: "SUNNYVALE",
            state: "CA",
            zip: "94085",
            country: "US",
        },
    ];

    // 1) The "Video" datatype example:
    {
        let mut root = CJson::create_object();
        root.add_item_to_object("name", CJson::create_string(r#"Jack ("Bee") Nimble"#))
            .unwrap();

        let mut fmt = CJson::create_object();
        fmt.add_item_to_object("type", CJson::create_string("rect"))
            .unwrap();
        fmt.add_item_to_object("width", CJson::create_number(1920.0))
            .unwrap();
        fmt.add_item_to_object("height", CJson::create_number(1080.0))
            .unwrap();
        fmt.add_item_to_object("interlace", CJson::create_bool(false))
            .unwrap();
        fmt.add_item_to_object("frame rate", CJson::create_number(24.0))
            .unwrap();

        root.add_item_to_object("format", fmt).unwrap();

        let out = root.print_formatted();
        println!("{}", out);
    }

    // 2) The "days of the week" array
    {
        // cJSON_CreateStringArray(strings, 7)
        let arr = CJson::Array(strings.iter().map(|&s| CJson::create_string(s)).collect());
        let out = arr.print_formatted();
        println!("{}", out);
    }

    // 3) The matrix
    {
        // root = cJSON_CreateArray();
        let mut root = CJson::create_array();
        for row in &numbers {
            let row_arr = CJson::Array(
                row.iter()
                    .map(|&n| CJson::create_number(n as f64))
                    .collect(),
            );
            root.add_item_to_array(row_arr).unwrap();
        }
        let out = root.print_formatted();
        println!("{}", out);
    }

    // 4) The "gallery" item
    {
        let mut root = CJson::create_object();
        let mut img = CJson::create_object();

        img.add_item_to_object("Width", CJson::create_number(800.0))
            .unwrap();
        img.add_item_to_object("Height", CJson::create_number(600.0))
            .unwrap();
        img.add_item_to_object("Title", CJson::create_string("View from 15th Floor"))
            .unwrap();

        // thumbnail
        let mut thm = CJson::create_object();
        thm.add_item_to_object(
            "Url",
            CJson::create_string("http:/*www.example.com/image/481989943"),
        )
        .unwrap();
        thm.add_item_to_object("Height", CJson::create_number(125.0))
            .unwrap();
        thm.add_item_to_object("Width", CJson::create_string("100"))
            .unwrap();

        img.add_item_to_object("Thumbnail", thm).unwrap();

        // IDs array
        let ids_arr = CJson::Array(
            ids.iter()
                .map(|&id| CJson::create_number(id as f64))
                .collect(),
        );
        img.add_item_to_object("IDs", ids_arr).unwrap();

        root.add_item_to_object("Image", img).unwrap();
        let out = root.print_formatted();
        println!("{}", out);
    }

    // 5) The array of "records"
    {
        let mut root = CJson::create_array();
        for r in &fields {
            let mut fld = CJson::create_object();
            fld.add_item_to_object("precision", CJson::create_string(r.precision))
                .unwrap();
            fld.add_item_to_object("Latitude", CJson::create_number(r.lat))
                .unwrap();
            fld.add_item_to_object("Longitude", CJson::create_number(r.lon))
                .unwrap();
            fld.add_item_to_object("Address", CJson::create_string(r.address))
                .unwrap();
            fld.add_item_to_object("City", CJson::create_string(r.city))
                .unwrap();
            fld.add_item_to_object("State", CJson::create_string(r.state))
                .unwrap();
            fld.add_item_to_object("Zip", CJson::create_string(r.zip))
                .unwrap();
            fld.add_item_to_object("Country", CJson::create_string(r.country))
                .unwrap();

            root.add_item_to_array(fld).unwrap();
        }
        let out = root.print_formatted();
        println!("{}", out);
    }
}

fn main() {
    // The sample JSON text blocks:
    let text1 = r#"{
"name": "Jack (\"Bee\") Nimble", 
"format": {"type": "rect", "width": 1920, "height": 1080, "interlace": false,"frame rate": 24}
}"#;

    let text2 = r#"["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]"#;

    let text3 = r#"[
    [0, -1, 0],
    [1, 0, 0],
    [0, 0, 1]
    ]"#;

    let text4 = r#"{
	"Image": {
		"Width":  800,
		"Height": 600,
		"Title":  "View from 15th Floor",
		"Thumbnail": {
			"Url":    "http:/*www.example.com/image/481989943",
			"Height": 125,
			"Width":  "100"
		},
		"IDs": [116, 943, 234, 38793]
	}
}"#;

    let text5 = r#"[
	{
		"precision": "zip",
		"Latitude":  37.7668,
		"Longitude": -122.3959,
		"Address":   "",
		"City":      "SAN FRANCISCO",
		"State":     "CA",
		"Zip":       "94107",
		"Country":   "US"
	},
	{
		"precision": "zip",
		"Latitude":  37.371991,
		"Longitude": -122.02602,
		"Address":   "",
		"City":      "SUNNYVALE",
		"State":     "CA",
		"Zip":       "94085",
		"Country":   "US"
	}
]"#;

    // Process each JSON text block
    doit(text1);
    doit(text2);
    doit(text3);
    doit(text4);
    doit(text5);

    // The C code commented these out:
    // dofile("../../tests/test1");
    // dofile("../../tests/test2");
    // dofile("../../tests/test3");
    // dofile("../../tests/test4");
    // dofile("../../tests/test5");

    // Build and print objects concisely:
    create_objects();
}
