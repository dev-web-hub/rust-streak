use std::fs::OpenOptions;
use std::io::Write;
use arboard::Clipboard;

fn main() {
    // Access clipboard
    let mut clipboard = Clipboard::new().expect("❌ Could not access clipboard");

    // Try to read clipboard text
    if let Ok(content) = clipboard.get_text() {
        // Open or create data.txt and append
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data.txt")
            .expect("❌ Could not open data.txt");

        // Write with a separator
        writeln!(file, "\n---\n{}", content).expect("❌ Could not write to data.txt");

        println!("✅ Clipboard saved to data.txt");
    } else {
        println!("⚠️ No text found in clipboard.");
    }
}
