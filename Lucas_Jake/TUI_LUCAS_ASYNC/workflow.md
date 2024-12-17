
# Mobile DNA Sim workflow for TUI_LUCAS_ASYNC

1. main.rs:
    * calls terminal::run_app()
2. run_app (in terminal.rs):
    * Sets up the terminal
    * Starts a background thread for the simulation
    * Runs the main UI loop: renders the interface and handles user input
3. UI Updates:
    * Calls render_ui to update the screen based on the state.
    * helps with rendering to home_renderer.rs, sim_renderer.rs, etc.
4. Simulation Logic:
    * Runs in a background thread, updates the shared state (Tabstate), and 
      triggers UI redraws

---

## Widgets
* app_widgets.rs

* footer_renderer.rs

* home_renderer.rs

* input_handler.rs

* key_bindings_renderer.rs

* mod.rs - 

* run_simulation.rs

* sim_renderer.rs

* tabstate.rs


* utils.rs  - contains components that can be reused.
   1) create_block()
    
---

## Files Deleted (8pm Dec. 16th)
terminal_ideas.rs
terminal_working.rs
home.rs
run_simulation.rs
sim.rs
