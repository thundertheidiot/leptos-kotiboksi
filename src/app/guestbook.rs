use leptos_router::ActionForm;
use leptos::CollectView;
use leptos::View;
use leptos::SignalGet;
use leptos::create_resource;
use leptos::create_server_action;
use leptos::Suspense;
use crate::css::ClassName;
use leptos::IntoView;
use leptos::view;
use leptos::component;
use leptos::server;
use leptos::ServerFnError;

#[server(GetGuestbookEntries, "/api", "GetJson")]
pub async fn get_guestbook_entries() -> Result<Vec<(String, String)>, ServerFnError> {
    use crate::db::db;

    let mut conn = db().await?;

    let entries = sqlx::query!("SELECT name, message FROM guestbook")
        .fetch_all(&mut conn)
        .await?;

    Ok(entries.into_iter().filter_map(|e| {
	if let (Some(name), Some(message)) = (e.name, e.message) {
	    Some((name, message))
	} else {
	    None
	}
    }).rev().collect::<Vec<(String, String)>>())
}

#[server(AddGuestbookEntry, "/api")]
pub async fn add_guestbook_entry(name: String, message: String) -> Result<(), ServerFnError> {
    use crate::db::db;

    let mut conn = db().await?;

    match sqlx::query!(
	"INSERT INTO guestbook (name, message) VALUES ($1, $2)",
	name,
	message
    ).execute(&mut conn)
	.await {
	    Ok(_row) => Ok(()),
	    Err(e) => Err(ServerFnError::ServerError(e.to_string()))
	}
}

#[component]
pub fn Guestbook() -> impl IntoView {
    let add_entry = create_server_action::<AddGuestbookEntry>();

    let entries = create_resource(
	move || add_entry.version().get(),
	    move |_| get_guestbook_entries(),
	    );

    view! {
        <div class=ClassName::GUESTBOOK_CONTAINER>
            <div class=ClassName::TEXTPART>
                {move || {
                    view! {
                        <h2>"Sign the guestbook!"</h2>
                        <ActionForm action=add_entry>

                            <input
                                class=ClassName::GUESTBOOK_TEXTINPUT
                                type="text"
                                name="name"
                                placeholder="Name (max=50)"
                                maxlength="50"
                                required
                            />
                            <input
                                class=ClassName::GUESTBOOK_TEXTINPUT
                                type="text"
                                name="message"
                                placeholder="Message (max=1000)"
                                maxlength="1000"
                                required
                            />
                            <input
                                class=ClassName::GUESTBOOK_SUBMITINPUT
                                type="submit"
                                value="Submit"
                            />
                        </ActionForm>
                    }
                }}

            </div>
            <br/>

            <Suspense fallback=move || {
                view! {
                    <div class=ClassName::TEXTPART>
                        <p>"Loading entries..."</p>
                    </div>
                }
            }>
                {move || {
                    match entries.get() {
                        Some(Ok(entries)) => {
                            if entries.is_empty() {
                                view! {
                                    <div class=ClassName::TEXTPART>
                                        <p>"Be the first one to write something here!"</p>
                                    </div>
                                    <br/>
                                }
                                    .into()
                            } else {
                                entries
                                    .into_iter()
                                    .map(|(name, message)| {
                                        view! {
                                            <div class=ClassName::TEXTPART>
                                                <a>"Name:"</a>
                                                " "
                                                {name}
                                                <br/>
                                                <a>"Message:"</a>
                                                " "
                                                {message}
                                                <br/>
                                            </div>
                                            <br/>
                                        }
                                    })
                                    .collect_view()
                            }
                        }
                        _ => View::Text(view! { "uh oh, something went wrong" }),
                    }
                }}

            </Suspense>

        </div>
    }
}
