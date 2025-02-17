use leptos::SignalSet;
use leptos::create_signal;
use leptos::server_fn::ServerFn;
use leptos::create_node_ref;
use leptos::Action;
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

// Check build.rs
include!(concat!(env!("OUT_DIR"), "/secret.rs"));

#[server(GetGuestbookEntries, "/api", "GetJson")]
pub async fn get_guestbook_entries(secret: String) -> Result<Vec<(String, String)>, ServerFnError> {
    if secret != SECRET {
	return Err(ServerFnError::ServerError("haha skill issue".to_string()));
    }

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

#[server(AddGuestbookEntryArgs, "/api")]
pub async fn add_guestbook_entry_args(name: String, message: String) -> Result<(), ServerFnError> {
    todo!()
}

fn add_guestbook_entry_action() -> Action<AddGuestbookEntryArgs, Result<(), ServerFnError>> {
    #[cfg(feature = "ssr")]
    let action_function = |args: &AddGuestbookEntryArgs| AddGuestbookEntry::run_body(
	AddGuestbookEntry {
	    secret: SECRET.to_string(),
	    name: args.name.clone(),
	    message: args.message.clone(),
	}

    );

    // When not on the server send a fetch to request the fn call.
    #[cfg(not(feature = "ssr"))]
    let action_function = |args: &AddGuestbookEntryArgs| AddGuestbookEntry::run_on_client(
	AddGuestbookEntry {
	    secret: SECRET.to_string(),
	    name: args.name.clone(),
	    message: args.message.clone(),
	}
    );

    Action::new(action_function).using_server_fn()
}


#[server(AddGuestbookEntry, "/api")]
pub async fn add_guestbook_entry(secret: String, name: String, message: String) -> Result<(), ServerFnError> {
    if secret != SECRET {
	return Err(ServerFnError::ServerError("LIGMA BALLS".to_string()));
    }

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
    let add_entry = add_guestbook_entry_action();
    let (submitted, set_submit) = create_signal(false);

    let entries = create_resource(
	move || add_entry.version().get(),
	move |_| {
	    get_guestbook_entries(SECRET.to_string())
	},
    );

    let form_ref = create_node_ref::<leptos::html::Form>();

    view! {
        <div class=ClassName::GUESTBOOK_CONTAINER>
            <div class=ClassName::TEXTPART>
                {move || {
                    if submitted() {
                        view! { <p>"Thanks!"</p> }.into_view()
                    } else {
                        view! {
                            <h2>"Sign the guestbook!"</h2>
                            <ActionForm
                                action=add_entry
                                node_ref=form_ref

                                on:submit=move |_| {
                                    set_submit.set(true);
                                }
                            >

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
                            .into_view()
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
