use NandC::nand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(nand::nand(true, true), false);
        assert_eq!(nand::nand(true, false), true);
        assert_eq!(nand::nand(false, true), true);
        assert_eq!(nand::nand(false, false), true);
    }

    #[test]
    fn test_not() {
        assert_eq!(nand::not(true), false);
        assert_eq!(nand::not(false), true);
    }

    #[test]
    fn test_or() {
        assert_eq!(nand::or(true, true), true);
        assert_eq!(nand::or(true, false), true);
        assert_eq!(nand::or(false, true), true);
        assert_eq!(nand::or(false, false), false);
    }

    #[test]
    fn test_and() {
        assert_eq!(nand::and(true, true), true);
        assert_eq!(nand::and(true, false), false);
        assert_eq!(nand::and(false, true), false);
        assert_eq!(nand::and(false, false), false);
    }

    #[test]
    fn test_xor() {
        assert_eq!(nand::xor(true, true), false);
        assert_eq!(nand::xor(true, false), true);
        assert_eq!(nand::xor(false, true), true);
        assert_eq!(nand::xor(false, false), false);
    }

    #[test]
    fn test_add_bit() {
        let mut carry = false;
        assert_eq!(nand::add_bit(false, false, false, &mut carry), false);
        assert_eq!(carry, false);

        assert_eq!(nand::add_bit(true, false, false, &mut carry), true);
        assert_eq!(carry, false);

        assert_eq!(nand::add_bit(true, true, false, &mut carry), false);
        assert_eq!(carry, true);

        assert_eq!(nand::add_bit(true, true, true, &mut carry), true);
        assert_eq!(carry, true);
    }

    #[test]
    fn test_sub_bit() {
        let mut carry = false;
        assert_eq!(nand::sub_bit(false, false, false, &mut carry), false);
        assert_eq!(carry, false);

        assert_eq!(nand::sub_bit(true, false, false, &mut carry), true);
        assert_eq!(carry, false);

        assert_eq!(nand::sub_bit(true, true, false, &mut carry), false);
        assert_eq!(carry, false);

        assert_eq!(nand::sub_bit(false, true, false, &mut carry), true);
        assert_eq!(carry, true);
    }

    #[test]
    fn test_add_u4() {
        assert_eq!(nand::add_u4(0b0000, 0b0000), 0b0000);
        assert_eq!(nand::add_u4(0b0001, 0b0001), 0b0010);
        assert_eq!(nand::add_u4(0b1111, 0b0001), 0b0000); // Overflow
    }

    #[test]
    fn test_sub_u4() {
        assert_eq!(nand::sub_u4(0b0000, 0b0000), 0b0000);
        assert_eq!(nand::sub_u4(0b0010, 0b0001), 0b0001);
        assert_eq!(nand::sub_u4(0b0000, 0b0001), 0b1111); // Underflow
    }

    #[test]
    fn test_bll() {
        assert_eq!(nand::bll(true), "true");
        assert_eq!(nand::bll(false), "false");
    }

    #[test]
    fn test_print_add_bit() {
        // This function prints output, so we can't directly test it with assertions.
        // We can manually verify the output by running the test.
        nand::print_add_bit(true, true, false);
    }
}

fn main() {}