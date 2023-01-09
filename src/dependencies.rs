// rustup toolchain ? rustup update

// Component dependencies
// fmt: rustup component add rustfmt -> cargo fmt
// clippy: rustup component add clippy -> cargo clippy

// Binary dependencies
// git

pub const RUSTUP_COMPONENT_LIST: &[str] = ["clippy", "rustfmt"];
pub const CARGO_SUBCOMMAND_LIST: &[str] = ["fmt", "outdated", "audit"];
pub const SYSTEM_BINARY_LIST: &[str] = ["git"];

pub fn update_and_install_dependencies() {
    update();
    install_rustup_component();
    install_cargo_subcommand();
}

fn update() {
    // rustup self update
    // rustup update
    // cargo bin update
}
fn install_rustup_component() {}
fn install_cargo_subcommand() {}
