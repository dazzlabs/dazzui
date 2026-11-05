use dazzui_gtk4::{Theme, gtk};
use dazzui_gtk4_example::{app, conformance};
fn main() -> gtk::glib::ExitCode {
    let args: Vec<_> = std::env::args().collect();
    if args.iter().any(|a| a == "--conformance") {
        if let Err(error) = conformance::run() {
            eprintln!("{error}");
            return gtk::glib::ExitCode::FAILURE;
        }
        return gtk::glib::ExitCode::SUCCESS;
    }
    let option = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let theme = option("--theme").map(|name| Theme::new(&name)).transpose();
    match theme {
        Ok(theme) => app::run(
            option("--story"),
            theme.unwrap_or_default(),
            args.iter().any(|arg| arg == "--new-instance"),
        ),
        Err(error) => {
            eprintln!("{error}");
            gtk::glib::ExitCode::FAILURE
        }
    }
}
