// function names

pub const GET: &str = "_w.g"; // GET(Id) returns the value at memory slot Id
pub const DEL: &str = "_w.d"; // DEL(Id) removes the value at memory slot Id
pub const SET: &str = "_w.s"; // SET(Id, Value) sets the value at memory slot Id
pub const REP: &str = "_w.r"; // REP(Id, Value) sends the value back as id:json(value)
