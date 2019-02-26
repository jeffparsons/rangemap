extern crate skeptic;

fn main() {
    // Generate doc tests for `README.md`.
    skeptic::generate_doc_tests(&["README.md"]);
}
