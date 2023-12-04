///Send email messages 
pub mod email_client;

///Abstraction to handle user creation and deletion via some mean
///of verification from the user
pub mod user_communicator; 

pub use email_client::*;
pub use user_communicator::*;
