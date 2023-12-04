///For interaction with different databases
pub mod database;

///For working with environment variables
pub mod envvars;

///Error types used throughout the code
pub mod errors;

///Data models used throughout the code; not necessarily only data models
///that are stored in the database, but rather common data structures that glue
///together different parts of the applicatin.
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

///Dedicated module to work with prices in different units and currencies
pub mod price;

///Everything related to purchasing access to a paywalled item. Right now, handles only stripe
///payments
pub mod purchase;

///Functions corresponding to the different server routes
pub mod routes;

///Everything around session management and passwords/hashing
pub mod security;

///Everything around HTML template rendering - currently used only for the user management
///frontend, i.e. login/logout modal, etc.
pub mod templates;

///Provides functionality to send messages to users, e.g. confirmation emails.
pub mod user_communication;

///Generic utility functions that can be used throughout the application
pub mod utils;
