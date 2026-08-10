// src/main.rs
use warp::Filter;

#[tokio::main]
async fn main() {
    // 1. Serve every file in /public exactly as-is
    let static_files = warp::fs::dir("../target/dx/ore-app/release/web/public");

    // 2. SPA fallback: any unknown route → index.html
    let spa_fallback = warp::any().and(warp::fs::file(
        "../target/dx/ore-app/release/web/public/index.html",
    ));

    // 3. Combine them: static first, then fallback
    let routes = static_files.or(spa_fallback);

    println!("Ore Supply web UI → http://0.0.0.0:8080");
    println!(
        "Serving folder: {}",
        std::fs::canonicalize("../target/dx/ore-app/release/web/public")
            .unwrap()
            .display()
    );

    // 4. Bind
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
