#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use pythonathon::{app::*, fileserv::file_and_error_handler};
    // use clap::Parser;
    use tokio::net::TcpListener;
    // use dotenvy;
    use sea_orm::Database;
    //
    // #[derive(Parser)]
    // struct Args {
    //     // Database URL
    //     #[arg(short, long, default_value = "sqlite:./sqlite.db?mode=rwc")]
    //     database_url: String,
    //
    //     // Database Name
    //     #[arg(short, long, default_value = "pythonathon")]
    //     database_name: String,
    // }

    // #[derive(Clone)]
    // struct AppState {
    //     conn: DatabaseConnection,
    // }

    // dotenvy::dotenv().ok();
    // let args = Args::parse();

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    // We don't use an address for the lambda function
    #[allow(unused_variables)]
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Connect to the database
    // let db = Database::connect(args.database_url).await.unwrap();
    let conn = Database::connect("sqlite:./sqlite.db?mode=rwc")
        .await
        .expect("Database connection failed");

    // build our application with a route
    let additional_context = move || {
        provide_context(conn.clone());
    };

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes_with_context(&leptos_options, routes, additional_context, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // In development, we use the Hyper server
    #[cfg(debug_assertions)]
    {
        log::info!("listening on http://{}", &addr);
        let listener = TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }

    // In release, we use the lambda_http crate
    #[cfg(not(debug_assertions))]
    {
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);

        lambda_http::run(app).await.unwrap();
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
