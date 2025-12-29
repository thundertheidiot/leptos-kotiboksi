use crate::css::ClassName;
use leptos::logging::error;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_interval;
use leptos_use::UseIntervalReturn;

#[component]
fn RadioAudio(
    volume: ReadSignal<f64>,
    playing: ReadSignal<bool>,
    visible: ReadSignal<bool>,
) -> impl IntoView {
    let audio_ref = NodeRef::new();

    let stream_url = if cfg!(debug_assertions) {
	"https://kotiboksi.xyz/radio.ogg"
    } else {
	"/radio.ogg"
    };

    let audio = view! {
        <audio node_ref=audio_ref prop:volume=move || volume() / 100.0>
            <source src=stream_url type="audio/ogg" />
        </audio>
    };

    Effect::new(move |_| {
        if let Some(audio) = audio_ref.get() {
            if visible() {
                if playing() {
                    let _ = audio.play();
                } else {
                    let _ = audio.pause();
                }
            } else {
                let _ = audio.pause();
            }
        } else {
            error!("failed to grab radio audio by reference");
        }
    });

    audio
}

#[server(GetNowPlaying, "/api")]
pub async fn get_now_playing() -> Result<String, ServerFnError> {
    use reqwest;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Source {
	title: String
    }
    #[derive(Deserialize)]
    struct Stats {
	source: Source
    }
    #[derive(Deserialize)]
    struct Status {
	icestats: Stats
    }

    let status_url = if cfg!(debug_assertions) {
	"https://kotiboksi.xyz/status-json.xsl"
    } else {
	// icecast on server
	"http://127.0.0.1:8000/status-json.xsl"
    };

    let status = reqwest::get(status_url)
        .await?
	.json::<Status>()
	.await?;

    Ok(status.icestats.source.title)
}

#[component]
pub fn NowPlaying(playing: ReadSignal<bool>) -> impl IntoView {
    let UseIntervalReturn {
        counter,
        pause,
        resume,
        ..
    } = use_interval(5000);

    let (title, set_title) = signal::<Result<String, String>>(Ok("Loading...".into()));

    Effect::new(move || match playing() {
        true => resume(),
        false => pause(),
    });

    Effect::new(move || {
        let _tick = counter();

	if !cfg!(target_arch = "wasm32") {
	    return;
	}

	spawn_local(async move {
	    set_title(get_now_playing().await.map_err(|e| e.to_string()));
	});
    });

    view! {
        <p>
            {move || {
                match title() {
                    Ok(v) => {
                        view! {
                            "Now playing: "
                            <em>{v}</em>
                        }
                            .into_any()
                    }
                    Err(e) => {
                        view! {
                            "Unable to fetch song: "
                            <strong>{e}</strong>
                        }
                            .into_any()
                    }
                }
            }}
        </p>
        <p>
            "Your browser can sometimes be behind on playback, so the title doesn't always match up."
        </p>
    }
}

#[component]
pub fn Radio() -> impl IntoView {
    let (visible, set_visible) = signal(false);
    let (playing, set_playing) = signal(false);
    let (volume, set_volume) = signal(50.0);

    view! {
        <button class=ClassName::NAVBUTTON on:click=move |_| { set_visible.update(|n| *n = !*n) }>
            {move || if visible() { "Hide Radio" } else { "Show Radio" }}
        </button>

        <div
            class=ClassName::RADIO_CONTROLS
            style=move || { if visible() { "display:block" } else { "display:none" } }
        >

            <br />

            <RadioAudio volume=volume playing=playing visible=visible />
            <h2>"Radio"</h2>

            <button
                class=ClassName::NAVBUTTON
                on:click=move |_| { set_playing.update(|n| *n = !*n) }
            >
                {move || { if playing() { "Pause" } else { "Play" } }}
            </button>

            <br />

            <input
                class=ClassName::SLIDER
                type="range"
                min="0"
                max="100"
                on:input=move |ev| {
                    set_volume(event_target_value(&ev).parse::<f64>().unwrap_or(50.0));
                }
            />

            <p>"Volume: " {volume} "%"</p>

            <NowPlaying playing=playing />

            <p>
                "Link: "
                <a href="https://kotiboksi.xyz/radio.ogg">"https://kotiboksi.xyz/radio.ogg"</a>
            </p>

            <p>
                "The radio has " <a href="https://en.wikipedia.org/wiki/Module_file">mod music</a>
                " from the " <a href="http://modarchive.org/">"mod archive"</a>
                ", and other random places."
            </p>
        </div>
    }
}
