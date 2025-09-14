use crate::css::ClassName;

use leptos::either::Either;
use leptos::prelude::*;

// Check build.rs
// include!(concat!(env!("OUT_DIR"), "/secret.rs"));

#[server]
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

// #[server(AddGuestbookEntryArgs, "/api")]
// pub async fn add_guestbook_entry_args(name: String, message: String) -> Result<(), ServerFnError> {
//     todo!()
// }

// fn add_guestbook_entry_action() -> Action<AddGuestbookEntryArgs, Result<(), ServerFnError>> {
//     #[cfg(feature = "ssr")]
//     let action_function = |args: &AddGuestbookEntryArgs| AddGuestbookEntry::run_body(
// 	AddGuestbookEntry {
// 	    secret: SECRET.to_string(),
// 	    name: args.name.clone(),
// 	    message: args.message.clone(),
// 	}

//     );

//     // When not on the server send a fetch to request the fn call.
//     #[cfg(not(feature = "ssr"))]
//     let action_function = |args: &AddGuestbookEntryArgs| AddGuestbookEntry::run_on_client(
// 	AddGuestbookEntry {
// 	    secret: SECRET.to_string(),
// 	    name: args.name.clone(),
// 	    message: args.message.clone(),
// 	}
//     );

//     Action::new(action_function)
// }


#[server]
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

// no real security here, should probably disable this
#[component]
pub fn Guestbook() -> impl IntoView {
    let add_entry = ServerAction::<AddGuestbookEntry>::new();
    let (submitted, set_submit) = signal(false);

    let entries = Resource::new(
	move || add_entry.version().get(),
	move |_| {
	    get_guestbook_entries()
	}
    );

    let entry_suspense = move || {
	Suspend::new(async move {
	    entries.await.map(|entries| {
		if entries.is_empty() {
		    Either::Left(view! { <p>"no"</p> })
		} else {
		    Either::Right(
			entries.iter().map(|(name, message)| {
			    view! {
                    <div class=ClassName::TEXTPART>
                        <a>"Name: "</a>
                        {name.clone()}
                        <br />
                        <a>"Message: "</a>
                        {message.clone()}
                        <br />
				    </div>
				    <br/>
                }
			}).collect::<Vec<_>>()
		    )
		}
	    })
	})
    };

    view! {
        <div class=ClassName::GUESTBOOK_CONTAINER>
            <div class=ClassName::TEXTPART>
                <Show when=move || !submitted() fallback=|| view! { <p>"Thanks!"</p> }.into_view()>
                    <h2>"Sign the guestbook!"</h2>
                    <ActionForm
                        action=add_entry

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

            </Show>

	    <br/>

                <Transition fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>{entry_suspense}</Transition>

            </div>
            <br />
        </div>
    }
}
