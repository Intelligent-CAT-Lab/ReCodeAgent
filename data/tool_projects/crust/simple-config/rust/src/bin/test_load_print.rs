use Simple_Config::simple_config::*;
#[test]
fn run_print_test() {
    let expected = "font: \"JetBrainsMono Nerd Font\"\n\
                                   font.size: 14\n\
                                   zoom: 1.500000\n\
                                   line_numbers: true\n\
                                   ruler: false\n\
                                   bg.color: rgba(255, 255, 255, 255)";
    let src = "font: \"JetBrainsMono Nerd Font\"\n\
                              font.size: 14\n\
                              zoom: 1.5\n\
                              line_numbers: true\n\
                              ruler: false\n\
                              bg.color: rgba(255, 255, 255, 1)";

    let cfg = cfg_parse(src).unwrap();
    assert!(cfg.to_string() == expected);
}
#[test]
fn test(){
    assert!(cfg_parse_file("sample.cfg").is_ok());
    assert!(cfg_parse_file("sample.txt").is_err_and(|err| err.msg == "invalid file extension"));
    assert!(cfg_parse_file("").is_err_and(|err| err.msg == "invalid filename"));
    assert!(cfg_parse_file("sample2.cfg").is_err_and(|err| err.msg == "failed to open file"));
}
fn main() {}
