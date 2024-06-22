pub mod rups;

fn main() {
    rups::run::run().expect("Failed to run rups.");
}
