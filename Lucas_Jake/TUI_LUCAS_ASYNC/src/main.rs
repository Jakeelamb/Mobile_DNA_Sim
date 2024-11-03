// ---- Used for main program -----
mod terminal; // Module for handling terminal operations
mod logo; // Module for getting the ASCII logo
mod widgets; // Module for the widgets (tabs, etc.)
// mod widgets {
//     pub mod tabs; // Make the tabs module public
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::run_app() // Call the terminal application
}

// -- End program --