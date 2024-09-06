mod copy_and_paste;
mod cut_and_paste;

use copy_and_paste::CopyAndPaste;
use cut_and_paste::CutAndPaste;

fn main() {
    let cp = CopyAndPaste {
        sequence: String::from("Copy and Paste"),
    };

    let result = cp.print();
    println!("Result: {}", result);

    let cut = CutAndPaste {
        sequence: String::from("Cut and Paste"),
    };

    let result = cut.print();
    println!("Result: {}", result);
}
