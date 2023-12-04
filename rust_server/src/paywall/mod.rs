///Main abstraction of the paywall module. `PaywallServer` is a trait that
///contains all the necessary functions to handle paywalled content in an abstract way.
///Right now, an `std::collections::HashMap` is the implementation of choice but other,
///more efficient data structures could be implemented as well in the future.
pub mod paywall_server;

///Factory method to create a paywall server from its key components
///(i.e. recursive filereader, object manipulator, paywall extractor)
pub mod paywall_server_factory;

///Unifying factory method to create a paywall for quarto HTML pages; this is not necessarily
///the only use case and other types of content, e.g. documents or videos could also be 
///put behind a paywall in the future. Only a few changes and/or additions should be necessary
///to accomplish the latter.
pub mod quarto_paywall;

///Simple filereader to recursively read and load all files from a designated directory 
///(e.g. the quarto export directory)
pub mod recursive_filereader;

///Abstractions to manipulate data and assign the output to a `SessionConditionalObject`, e.g.
///to serve variations of the same object based on different values of a user session
///(e.g. their privileges)
pub mod session_conditional_manipulation;

///Stores variations of the same object and returns a variation based on given properties of the
///user session
pub mod session_conditional_object;

pub use paywall_server::*;
pub use paywall_server_factory::*;
pub use quarto_paywall::*;
pub use recursive_filereader::*;
pub use session_conditional_manipulation::*;
pub use session_conditional_object::*;
