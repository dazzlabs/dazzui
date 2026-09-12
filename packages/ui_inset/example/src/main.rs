use inset::{DefaultEmbedder, IntoWidget, Shell, run_app};
fn main() {
    let embedder = DefaultEmbedder::default();
    #[cfg(not(target_arch = "wasm32"))]
    let embedder = embedder.implicit_view(Some(inset::ImplicitViewConfig {
        title: "DazzUI — Inset".into(),
        logical_size: [1000.0, 720.0],
    }));
    embedder.run(|platform| {
        Shell::new(platform, |app| {
            let gallery = dazzui_inset_playground::Gallery::new(app);
            run_app(
                app,
                dazzui_inset_playground::host(gallery).into_widget(),
            );
        })
    });
}
