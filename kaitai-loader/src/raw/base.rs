/// Partially generated using schemafy.

pub mod endian;
pub mod scalar;
pub mod switch;
pub mod doc;

pub use self::endian::Endian;
pub use self::scalar::{AnyNonNullScalar, AnyScalar, StringOrBoolean, StringOrNumber};
pub use self::switch::Switch;
pub use self::doc::{Doc, DocRef};

pub type MimeType = String;
pub type IsoIdentifier = String;
pub type LocIdentifier = String;
pub type MediaWikiPageName = String;
pub type PronomIdentifier = String;
pub type WikidataIdentifier = String;
pub type RfcIdentifier = StringOrNumber;
pub type Identifier = String;
