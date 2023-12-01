pub mod database;
pub mod envvars;
pub mod errors;
pub mod models;

///This module holds the key logic for a pay-per-article paywall
///In here, things have become quite abstract but with the (hopeful) advantage
///of being fairly extensible for the future. 
///
///Some ideas including paywalling other file-formats, quarto books and possibly
///also a way to implement a subscription based paywall
///
///In general, this module consists of 3 modules:
///- A recursive filereader: Reads through a complete folder and loads filepaths
///     and filenames for all given filetypes
///- A file manipulator: Manipulates the files by user provided functions and allows
///     to assign the output of each function to a session based property. 
///     E.g. we read all HTML quarto outputs, remove paywalled content and add respective
///     notifications to the document, based on the user privileges (logged out, logged in,
///     paid for article).
///- A paywall extraction function: Extracts the properties of the paywalled item (identifier,
///     price, link) from the object that was loaded from the filereader. Currently, this is
///     just done for html/text but we could potentially also implement that for videos or pdfs
pub mod paywall;
pub mod price;
pub mod purchase;
pub mod routes;
pub mod security;
pub mod templates;
pub mod user_communication;
pub mod utils;
