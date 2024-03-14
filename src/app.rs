use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

/// The root component of the application.
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // let theme = create_rw_signal(Theme::dark());

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pythonathon.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            // .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "ssr")]
fn db() -> sea_orm::DatabaseConnection {
    use_context::<sea_orm::DatabaseConnection>().unwrap()
}

#[server(GetPuzzle, "/puzzle/get")]
pub async fn get_puzzle(id: i32) -> Result<String, ServerFnError> {
    use entity::puzzle::Entity as Puzzle;
    use sea_orm::EntityTrait;

    let conn = db();

    let Ok(maybe_puzzle) = Puzzle::find_by_id(id).one(&conn).await else {
        return Err(ServerFnError::new("Failed to fetch requested puzzle"));
    };
    let Some(puzzle) = maybe_puzzle else {
        return Ok("Blank puzzle".to_string());
    };
    Ok(puzzle.text)
}

#[server(SetPuzzle, "/puzzle/set")]
pub async fn set_puzzle(id: i32, text: String) -> Result<(), ServerFnError> {
    use entity::puzzle;
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

    let conn = db();

    let p: puzzle::ActiveModel = match puzzle::Entity::find_by_id(id)
        .one(&conn)
        .await
        .map_err(|_| ServerFnError::new("Failed to fetch requested puzzle"))?
    {
        Some(record) => {
            let mut _p: puzzle::ActiveModel = record.into();
            _p.set(entity::puzzle::Column::Text, text.into());
            _p
        }
        None => puzzle::ActiveModel {
            text: Set(text),
            ..Default::default()
        },
    };
    p.save(&conn)
        .await
        .map_err(|_| ServerFnError::new("Failed to save puzzle"))
        .map(|_| ())
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let puzzle_text = create_rw_signal("".to_string());

    let load_puzzle = create_action(move |_: &()| async move {
        puzzle_text.set(get_puzzle(1).await.unwrap_or("".to_string()));
    });
    load_puzzle.dispatch(());

    let save_puzzle = create_action(|input: &String| {
        let input = input.clone();
        async move { set_puzzle(1, input).await }
    });

    view! {
        <h1>"Here's the puzzle!"</h1>
        { move ||
            match load_puzzle.pending().get() {
                true => view! {
                    <p>"Loading..."</p>
                }.into_view(),
                false => view! {
                    <input type="text"
                        on:input=move |ev| { puzzle_text.set(event_target_value(&ev)) }
                        prop:value=puzzle_text
                    />
                    <button on:click=move |_| {
                        save_puzzle.dispatch(puzzle_text.get());
                    }>"Save Puzzle"</button>
                }.into_view()
            }
        }
    }
}
