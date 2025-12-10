use wspace::nameparts::parse;

// Test helper module to access internal nameString functionality
// We need to replicate the tests by using the public parse function
// and checking the results, since nameString is internal

#[test]
fn TestLooksCorporate() {
    // Testing via parse function since nameString is internal
    let res = parse("Sprockets Inc").unwrap();

    // If it looks corporate, parse returns empty first/last name
    assert_eq!(res.first_name, "", "Expected empty first_name for corporate name");
    assert_eq!(res.last_name, "", "Expected empty last_name for corporate name");
}

#[test]
fn TestSearchParts() {
    // Testing via parse function since nameString is internal
    let res = parse("Mr. James Polera").unwrap();

    // If searchParts finds "Mr" as salutation at position 0, it should be slotted
    assert_eq!(res.salutation, "Mr.", "Expected 'Mr.' in salutation field");
}

#[test]
fn TestClean() {
    // Testing via parse function since nameString is internal
    let res = parse("Mr. James Polera").unwrap();

    // cleaned() removes punctuation, so "Mr." becomes "Mr"
    // This is verified through proper parsing
    assert_eq!(res.salutation, "Mr.", "Expected 'Mr.' in salutation field");
    assert_eq!(res.first_name, "James", "Expected 'James' in first_name field");
}

#[test]
fn TestLocateSalutation() {
    // Testing via parse function since nameString is internal
    let res = parse("Mr. James Polera").unwrap();

    // find("salutation") should locate at position 0
    assert_eq!(res.salutation, "Mr.", "Expected 'Mr.' at position 0");
}

#[test]
fn TestHasComma() {
    // Testing via parse function since nameString is internal
    let res = parse("Polera, James").unwrap();

    // hasComma should detect comma and normalize to "James Polera"
    assert_eq!(res.first_name, "James", "Expected 'James' as first_name");
    assert_eq!(res.last_name, "Polera", "Expected 'Polera' as last_name");
}

#[test]
fn TestNormalize() {
    // Testing via parse function since nameString is internal
    let res = parse("Polera, James").unwrap();

    // normalize() should swap "Polera, James" to "James Polera"
    assert_eq!(res.first_name, "James", "Expected 'James' as first_name after normalization");
    assert_eq!(res.last_name, "Polera", "Expected 'Polera' as last_name after normalization");
}

#[test]
fn TestParseAllFields() {
    let res = parse("Mr. James J. Polera Jr. Esq.").unwrap();

    assert_eq!(res.salutation, "Mr.", "Expected 'Mr.'. Actual {}", res.salutation);
    assert_eq!(res.first_name, "James", "Expected 'James'. Actual {}", res.first_name);
    assert_eq!(res.middle_name, "J.", "Expected 'J.'. Actual {}", res.middle_name);
    assert_eq!(res.last_name, "Polera", "Expected 'Polera'. Actual {}", res.last_name);
    assert_eq!(res.generation, "Jr.", "Expected 'Jr.'. Actual {}", res.generation);
    assert_eq!(res.suffix, "Esq.", "Expected 'Esq.'. Actual {}", res.suffix);
}

#[test]
fn TestParseOnlySalutation() {
    let res = parse("Mr.").unwrap();

    assert_eq!(res.first_name, "", "Expected ''. Actual {}", res.first_name);
    assert_eq!(res.last_name, "", "Expected ''. Actual {}", res.last_name);
}

#[test]
fn TestParseFirstLast() {
    let res = parse("James Polera").unwrap();

    assert_eq!(res.first_name, "James", "Expected 'James'. Actual {}", res.first_name);
    assert_eq!(res.last_name, "Polera", "Expected 'Polera'. Actual {}", res.last_name);
}

#[test]
fn TestLastNamePrefix() {
    let res = parse("Otto von Bismark").unwrap();

    assert_eq!(res.first_name, "Otto", "Expected 'Otto'. Actual {}", res.first_name);
    assert_eq!(res.last_name, "von Bismark", "Expected 'von Bismark'. Actual {}", res.last_name);
}

#[test]
fn TestAliases() {
    let res = parse("James Polera a/k/a Batman").unwrap();

    assert_eq!(res.aliases[0].first_name, "Batman", "Expected 'Batman'. Actual: {}", res.aliases[0].first_name);
}

#[test]
fn TestNickname() {
    let res = parse("Philip Francis 'The Scooter' Rizzuto").unwrap();

    assert_eq!(res.nickname, "'The Scooter'", "Expected 'The Scooter'. Actual: {}", res.nickname);
}

#[test]
fn TestStripSupplemental() {
    let res = parse("Philip Francis 'The Scooter' Rizzuto, deceased").unwrap();

    assert_eq!(res.first_name, "Philip", "Expected 'Philip'. Actual: {}", res.first_name);
    assert_eq!(res.middle_name, "Francis", "Expected 'Francis'. Actual: {}", res.middle_name);
    assert_eq!(res.nickname, "'The Scooter'", "Expected 'The Scooter'. Actual: {}", res.nickname);
    assert_eq!(res.last_name, "Rizzuto", "Expected 'Rizzuto'. Actual: {}", res.last_name);
}

#[test]
fn TestLongPrefixedLastName() {
    let res = parse("Saleh ibn Tariq ibn Khalid al-Fulan").unwrap();

    assert_eq!(res.first_name, "Saleh", "Expected 'Saleh'. Actual: {}", res.first_name);
    assert_eq!(res.last_name, "ibn Tariq ibn Khalid al-Fulan",
               "Expected 'ibn Tariq ibn Khalid al-Fulan'. Actual: {}", res.last_name);
}

#[test]
fn TestMisplacedApostrophe() {
    let res = parse("John O' Hurley").unwrap();

    assert_eq!(res.first_name, "John", "Expected 'John'. Actual: {}", res.first_name);
    assert_eq!(res.last_name, "O'Hurley", "Expected 'O'Hurley'. Actual: {}", res.last_name);
}

#[test]
fn TestMultipleAKA() {
    let res = parse("Tony Stark a/k/a Ironman a/k/a Stark, Anthony a/k/a Anthony Edward \"Tony\" Stark").unwrap();

    assert_eq!(res.aliases.len(), 3, "Expected 3 aliases. Actual: {}", res.aliases.len());
    assert_eq!(res.first_name, "Tony", "Expected 'Tony'. Actual: {}", res.first_name);
    assert_eq!(res.last_name, "Stark", "Expected 'Stark'. Actual: {}", res.last_name);
}

#[test]
fn TestBuildFullName() {
    let res = parse("President George Herbert Walker Bush").unwrap();

    assert_eq!(res.full_name, "President George Herbert Walker Bush",
               "Expected 'President George Herbert Walker Bush'. Actual: {}", res.full_name);
}

#[test]
fn TestDottedAka() {
    let res = parse("James Polera a.k.a James K. Polera").unwrap();

    assert_eq!(res.aliases.len(), 1, "Expected 1 alias. Actual: {}", res.aliases.len());
}

#[test]
fn TestUnicodeCharsInName() {
    let res = parse("König Ludwig").unwrap();

    assert_eq!(res.first_name, "König", "Expected 'König'. Actual: {}", res.first_name);
}

#[test]
fn TestTabsInName() {
    let res = parse("Dr. James\tPolera\tEsq.").unwrap();

    assert_eq!(res.salutation, "Dr.", "Expected 'Dr.'. Actual: {}", res.salutation);
    assert_eq!(res.first_name, "James", "Expected 'James'. Actual: {}", res.first_name);
    assert_eq!(res.last_name, "Polera", "Expected 'Polera'. Actual: {}", res.last_name);
    assert_eq!(res.suffix, "Esq.", "Expected 'Esq.'. Actual: {}", res.suffix);
}

#[test]
fn TestObviouslyBadName() {
    // make sure we don't panic on a clearly bad name
    // In Rust, we don't need defer/recover, we just run the function
    // If it panics, the test will fail automatically
    let _ = parse("I am a Popsicle");
    // If we get here, no panic occurred - test passes
}

#[test]
fn TestLastNameSalutation() {
    // make sure we don't panic if the last name looks like a salutation
    let res = parse("Alan Hon").unwrap();

    assert_eq!(res.first_name, "Alan", "Expected 'Alan'. Actual: {}", res.first_name);
    assert_eq!(res.last_name, "Hon", "Expected 'Hon'. Actual: {}", res.last_name);
    assert_eq!(res.full_name, "Alan Hon", "Expected 'Alan Hon'. Actual: {}", res.full_name);
}

#[test]
fn TestLastNameNonName() {
    // make sure we don't panic if the last name looks like a nonname
    let res = parse("Jessica Aka").unwrap();

    assert_eq!(res.first_name, "Jessica", "Expected 'Jessica'. Actual: {}", res.first_name);
    assert_eq!(res.last_name, "Aka", "Expected 'Aka'. Actual: {}", res.last_name);
    assert_eq!(res.full_name, "Jessica Aka", "Expected 'Jessica Aka'. Actual: {}", res.full_name);
}

#[test]
fn TestNameEndsWithApostrophe() {
    // make sure we don't panic on a name ending with apostrophe
    let res = parse("James Polera'").unwrap();

    assert_eq!(res.first_name, "James", "Expected 'James'. Actual: {}", res.first_name);
    assert_eq!(res.last_name, "Polera", "Expected 'Polera'. Actual: {}", res.last_name);
}

#[test]
fn ExampleParse() {
    let res = parse("Thurston Howell III").unwrap();

    // Verify the expected output matches the Go example
    assert_eq!(res.first_name, "Thurston", "first_name should be Thurston");
    assert_eq!(res.last_name, "Howell", "last_name should be Howell");
    assert_eq!(res.generation, "III", "generation should be III");
}

#[test]
fn ExampleParse_second() {
    let res = parse("President George Herbert Walker Bush").unwrap();

    // Verify the expected output matches the Go example
    assert_eq!(res.salutation, "President", "salutation should be President");
    assert_eq!(res.first_name, "George", "first_name should be George");
    assert_eq!(res.middle_name, "Herbert Walker", "middle_name should be Herbert Walker");
    assert_eq!(res.last_name, "Bush", "last_name should be Bush");
}
