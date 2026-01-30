use std::fs;

pub fn get_ascii_logo() -> String {
    fs::read_to_string("src/img/logo2.txt")
        .expect("Failed to read logo2.txt")
        .trim()
        .to_string()
}

pub fn get_ascii_sim() -> String {
    fs::read_to_string("src/img/simulation.txt")
        .expect("Failed to read simulation.txt")
        .trim()
        .to_string()
}

// pub fn create_logo_block(is_home_tab: bool) -> Paragraph {
//     let content = if is_home_tab {
//         get_ascii_logo() // Home tab gets the logo
//     } else {
//         get_ascii_sim() // Simulation tab gets the simulation ASCII
//     };

//     // Create a Paragraph widget for the content
//     Paragraph::new(content)
//         .block(Block::default().title("Logo").borders(Borders::ALL)) // Title and borders
//         .style(Style::default().fg(Color::White)) // Style for the text
// }