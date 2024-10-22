from textual.app import App, ComposeResult
from textual.containers import Container
from textual.widgets import Static, Input, Button
import numpy as np
import pandas as pd
import random
import time

class SimulationApp(App):

    def compose(self) -> ComposeResult:
        """Create the UI layout."""
        yield Static("Species Data Entry")
        yield Input(placeholder="Genome size", id="genome_size")
        yield Input(placeholder="Average Exon Range", id="average_range")
        yield Input(placeholder="Number of simulation rounds", id="simulation_rounds")
        yield Input(placeholder="Number of active TEs", id="num_active_te")
        yield Button(label="Run Simulation", id="run_button")

    def on_button_pressed(self, event: Button.Pressed) -> None:
        """When the 'Run Simulation' button is pressed."""
        species_data = {
            'Genome_size': int(self.query_one("#genome_size").value),
            'Average Range': int(self.query_one("#average_range").value)
        }
        simulation_rounds = int(self.query_one("#simulation_rounds").value)
        num_active_te = int(self.query_one("#num_active_te").value)

        # Run simulation
        results_df = simulation(species_data, simulation_rounds, num_active_te)
        # Display results in TUI
        self.query_one("#run_button").update("Simulation Complete!")

def get_results_dictionary(species_data, simulation_rounds, genome_size, num_active_te, range_1_start, range_1_end, range_2_start, range_2_end):
    species_results = {
        'Species': 'Sample Species',
        'Beginning Genome Size': genome_size,
        'Exon Start Range': range_1_start,
        'Exon End Range': range_1_end,
        'Non-Coding Start Range': range_2_start,
        'Non-Coding End Range': range_2_end,
        'Active TEs': num_active_te,
        'TEs mobilized': 0,
        'TEs static': 0,
        'TEs in Exons': 0,
        'TEs in Non-Coding': 0,
        'Exon New Size': range_1_end - range_1_start + 1,
        'Non-Coding New Size': range_2_end - range_2_start + 1,
        'Total Genome Growth': 0,
        'Simulation Rounds': simulation_rounds,
        'Total Time': 0
    }
    return species_results

def simulation(species_data, simulation_rounds, num_active_te):
    te_lengths = np.random.normal(loc=5000, scale=2000, size=num_active_te)
    te_lengths = np.clip(te_lengths, 100, 10000).astype(int)

    results = get_results_dictionary(species_data, simulation_rounds, species_data['Genome_size'], num_active_te, 0, species_data['Average Range'], species_data['Average Range'] + 1, species_data['Genome_size'])
    
    # Simplified simulation logic
    for _ in range(simulation_rounds):
        for _ in range(num_active_te):
            if random.random() < 0.5:
                te_position = random.randint(0, species_data['Genome_size'])
                if te_position <= results['Exon End Range']:
                    results['TEs in Exons'] += 1
                else:
                    results['TEs in Non-Coding'] += 1
                results['TEs mobilized'] += 1
            else:
                results['TEs static'] += 1
    return pd.DataFrame([results])

# Run the TUI application
if __name__ == "__main__":
    app = SimulationApp()
    app.run()