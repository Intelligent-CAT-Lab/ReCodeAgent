// Import the leftpad module
use leftpad::leftpad::leftpad;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leftpad() {
        let mut buf = [0u8; 10];

        assert_eq!(leftpad("test", "*", 4, &mut buf), 4);
        assert_eq!(std::str::from_utf8(&buf[..4]).unwrap(), "test");

        assert_eq!(leftpad("test", "*", 9, &mut buf), 9);
        assert_eq!(std::str::from_utf8(&buf[..9]).unwrap(), "*****test");

        assert_eq!(leftpad("test", "*", 2, &mut buf), 4);
        assert_eq!(std::str::from_utf8(&buf[..4]).unwrap(), "test");

        assert_eq!(leftpad("test", " .", 9, &mut buf), 9);
        assert_eq!(std::str::from_utf8(&buf[..9]).unwrap(), " . . test");

        assert_eq!(leftpad("test", None.unwrap_or(""), 9, &mut buf), 9);
        assert_eq!(std::str::from_utf8(&buf[..9]).unwrap(), "     test");

        assert_eq!(leftpad(None.unwrap_or(""), "*", 9, &mut buf), 9);
        assert_eq!(std::str::from_utf8(&buf[..9]).unwrap(), "*********");

        assert_eq!(leftpad("test", "*", 11, &mut buf), 9);
        assert_eq!(std::str::from_utf8(&buf[..9]).unwrap(), "*******te");

        assert_eq!(leftpad("test", " ", 2, &mut []), 4);
        assert_eq!(leftpad("test", " ", 4, &mut []), 4);
        assert_eq!(leftpad("test", " ", 6, &mut []), 6);
        assert_eq!(leftpad("test", " ", 2, &mut []), 4);
        assert_eq!(leftpad("test", " ", 4, &mut []), 4);
        assert_eq!(leftpad("test", " ", 6, &mut []), 6);
    }
}

fn main() {}
