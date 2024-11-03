// ------ INPUT HANDLING ------

use crate::widgets::tabstate::TabState;
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn next(tab_state: &Arc<Mutex<TabState>>) {
    let mut state = tab_state.lock().await;
    state.index = (state.index + 1) % 2;
}

pub async fn previous(tab_state: &Arc<Mutex<TabState>>) {
    let mut state = tab_state.lock().await;
    state.index = if state.index == 0 { 1 } else { state.index - 1 };
}

// pub fn next(tab_state: &mut TabState) {
//     tab_state.index = (tab_state.index + 1) % 2;
// }

// pub fn previous(tab_state: &mut TabState) {
//     if tab_state.index > 0 {
//         tab_state.index -= 1;
//     }
// }

/// Move the selected item up in the list
pub fn scroll_up(tab_state: &mut TabState) {
    let i = match tab_state.list_state.selected() {
        Some(i) => {
            if i == 0 {
                tab_state.species.len() - 1 // Wrap around to the bottom
            } else {
                i - 1
            }
        }
        None => 0,
    };
    tab_state.list_state.select(Some(i));
}

/// Move the selected item down in the list
pub fn scroll_down(tab_state: &mut TabState) {
    let i = match tab_state.list_state.selected() {
        Some(i) => {
            if i >= tab_state.species.len() - 1 {
                0 // Wrap around to the top
            } else {
                i + 1
            }
        }
        None => 0,
    };
    tab_state.list_state.select(Some(i));
}
