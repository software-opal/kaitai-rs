
/// used to give a more detailed description of a user-defined type. In most languages, it will be
/// used as a docstring compatible with tools like Javadoc, Doxygen, JSDoc, etc.
pub type Doc = String;
/// used to provide reference to original documentation (if the ksy file is actually an
/// implementation of some documented format).
///
/// Contains:
/// 1. URL as text,
/// 2. arbitrary string, or
/// 3. URL as text + space + arbitrary string
pub type DocRef = Vec<String>;
