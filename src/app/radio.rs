use crate::css::ClassName;
use leptos::component;
use leptos::create_effect;
use leptos::create_node_ref;
use leptos::create_signal;
use leptos::event_target_value;
use leptos::html::Audio;
use leptos::logging::error;
use leptos::view;
use leptos::HtmlElement;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::SignalUpdate;

#[component]
fn RadioAudio(
    volume: ReadSignal<f64>,
    playing: ReadSignal<bool>,
    visible: ReadSignal<bool>,
) -> impl IntoView {
    let audio_ref = create_node_ref::<leptos::html::Audio>();

    let audio: HtmlElement<Audio> = view! {
        <audio node_ref=audio_ref prop:volume=move || volume() / 100.0>
            <source src="https://kotiboksi.xyz/radio.ogg"/>
        </audio>
    };

    create_effect(move |_| {
        let audio = match audio_ref.get() {
            Some(v) => v,
            None => {
                error!("failed to grab radio audio");
                return;
            }
        };

        if visible() {
            if playing() {
                let _ = audio.play();
            } else {
                let _ = audio.pause();
            }
        } else {
            let _ = audio.pause();
        }
    });

    audio
}

#[component]
pub fn Radio() -> impl IntoView {
    let (visible, set_visible) = create_signal(false);
    let (playing, set_playing) = create_signal(false);
    let (volume, set_volume) = create_signal(50.0);

    view! {
        <button class=ClassName::NAVBUTTON on:click=move |_| { set_visible.update(|n| *n = !*n) }>
            {move || if visible() { "Hide Radio" } else { "Show Radio" }}
        </button>

        <div
            class=ClassName::RADIO_CONTROLS
            style=move || { if visible() { "display:block" } else { "display:none" } }
        >
            <RadioAudio volume=volume playing=playing visible=visible/>
            <h2>"Radio"</h2>

            <button
                class=ClassName::NAVBUTTON
                on:click=move |_| { set_playing.update(|n| *n = !*n) }
            >
                {move || { if playing() { "Pause" } else { "Play" } }}
            </button>

            <br/>

            <input
                class=ClassName::SLIDER
                type="range"
                min="0"
                max="100"
                on:input=move |ev| {
                    set_volume(event_target_value(&ev).parse::<f64>().unwrap_or(50.0))
                }
            />

            <p>"Volume: " {move || volume()} "%"</p>

            <p>
                "Link: "
                <a href="https://kotiboksi.xyz/radio.ogg">"https://kotiboksi.xyz/radio.ogg"</a>
            </p>

            <p>
                "Currently the radio has "
                <a href="https://en.wikipedia.org/wiki/Module_file">mod music</a> " from the "
                <a href="http://modarchive.org/">"mod archive"</a> ", and other random places."
            </p>
        </div>
    }
}
