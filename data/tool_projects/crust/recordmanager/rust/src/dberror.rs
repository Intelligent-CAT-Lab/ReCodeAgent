/* Module-wide constants */
pub const PAGE_SIZE: i32 = 4096;
/* Return code definitions */
#[derive(Debug, PartialEq, Eq)]
pub enum RC {
    Ok = 0,
    FileNotFound = 1,
    FileHandleNotInit = 2,
    WriteFailed = 3,
    ReadNonExistingPage = 4,
    // Record Manager Errors
    RmCompareValueOfDifferentDatatype = 200,
    RmExprResultIsNotBoolean = 201,
    RmBooleanExprArgIsNotBoolean = 202,
    RmNoMoreTuples = 203,
    RmNoPrintForDatatype = 204,
    RmUnknownDatatype = 205,
    // Index Manager Errors
    ImKeyNotFound = 300,
    ImKeyAlreadyExists = 301,
    ImNToLarge = 302,
    ImNoMoreEntries = 303,
    // General Errors
    MemoryAllocationFail = 401,
    BufferpoolInUse = 402,
    CloseFailed = 403,
    Error = 404,
    BufferpoolFull = 405,
    ReadFailed = 406,
    InvalidHeader = 407,
    SeekFailed = 408,
    DestroyFailed = 409,
    RecordNotFound = 410,
    GeneralError = 411,
    ShutdownWithoutInit = 420,
    LoggingSetupFailure = 430,
}
pub fn throw(rc: RC, message: &str) -> i32{
    unimplemented!()
}
pub fn check(code: i32){
    unimplemented!()
}
pub fn print_error(error: RC) {
    unimplemented!()
}
pub fn error_message(error: RC) -> String {
    unimplemented!()
}