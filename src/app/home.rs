use crate::css::ClassName;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <div class=ClassName::TEXTPART>
                <img src="/static/img/profile.webp" class=ClassName::PROFILE />

                <h1>"Hiiii :3"</h1>

                <p>"I'm thunder, welcome to my stupid website"</p>

                <h2>"About"</h2>

                <p>
                    "I like tinkering with computers, both hardware and software. "
                    "I am now studying computer science at a "
                    <i>"\"university of applied sciences\""</i> "."
                </p>

                <p>
                    "I am a huge fan of NixOS and Emacs. " "You can find my very convoluted flake "
                    <a href="https://github.com/thundertheidiot/nixdots">"here"</a>
                    ", it is also powering this very server. "
                    <a href="https://github.com/thundertheidiot/emacs">"This"</a>
                    " is my Emacs config, packaged nicely with nix."
                </p>

                <p>
                    "My favourite general purpose language is Rust, "
                    "but I also like occasionally playing around with weirder stuff like Haskell. "
                    "I like the Nix language."
                </p>

            </div>

            <br />

            <div class=ClassName::TEXTPART>
                <h2>"This website"</h2>

                <p>
                    "This site was made with the "
                    <a href="https://leptos-rs.github.io/leptos/">"leptos framework"</a>
                    " because I'm stupid like that. "
                    "All of this could easily be created with basic html and a sprinkle of js for the radio."
                </p>

                <p>
                    "Thanks to leptos' server side rendering feature, there's no real downside, "
                    "even with WASM disabled most of the page works. "
                    "The WASM blob is very small compared to most modern websites."
                </p>

                <p>
                    "The pixel font is IBM VGA 8x16, and you can find it "
                    <a href="https://int10h.org/oldschool-pc-fonts/">"here"</a> "."
                </p>

            </div>

            <br />

            <div class=ClassName::TEXTPART>

                <a href="https://cyber.dabamos.de/88x31">
                    <img src="/static/img/buttons/88x31.gif" alt="the buttons" />
                </a>
                " "
                <a href="https://nixos.org">
                    <img src="/static/img/buttons/nixos.png" alt="nixos" />
                </a>
                " "
                <a href="https://cartercoding.com">
                    <img src="/static/img/buttons/carter.webp" alt="carer i am carer" />
                </a>
                " "
                <a href="https://corru.observer">
                    <img src="/static/img/buttons/corru-observer.gif" alt="corru.observer" />
                </a>

                <br style="margin: 10px;" />

                <img src="/static/img/buttons/gnu-linux.gif" alt="gnu/linux" />
                " "
                <img src="/static/img/buttons/emacs.gif" alt="emacs" />
                " "
                <img src="/static/img/buttons/pride.gif" alt="pridenow" />
                " "
                <img src="/static/img/buttons/transnow2.gif" alt="transnow" />
                " "

                <br style="margin: 10px;" />

                <a href="https://elissa.moe">
                    <img src="/static/img/buttons/elissa.png" alt="elissa (pingu)" />
                </a>
                " "
                <a href="https://isabelroses.com">
                    <img src="/static/img/buttons/isabelroses.gif" alt="isabel roses" />
                </a>
                " "
                <a href="https://robinwobin.dev">
                    <img src="/static/img/buttons/robin.gif" alt="robin" />
                </a>
                " "
                <a href="https://dbw.neocities.org">
                    <img src="/static/img/buttons/diza.png" alt="diza" width="88" height="31" />
                </a>
                " "
                <a href="https://tired.moe">
                    <img src="/static/img/buttons/tired.moe.gif" alt="tired.moe" />
                </a>
                " "
                <a href="https://adam.qpon/">
                    <img src="/static/img/buttons/adamperkowski.dev.gif" alt="adam" />
                </a>
                " "
                <a href="https://aprl.pet/">
                    <img src="/static/img/buttons/aprl.pet.png" alt="april!!!!" />
                </a>
                " "
                <a href="https://sketchni.uk/">
                    <img src="/static/img/buttons/sketchni.uk.png" alt="sketchni" />
                </a>
                " "
                <a href="https://tasky.nuxt.dev/">
                    <img src="/static/img/buttons/tasky.nuxt.dev.webp" alt="tasky" />
                </a>
                " "

            // when published
            // <img src="/static/img/buttons/agplv3-88x31.gif" alt="agplv3" />
            // " "
            </div>

            <br />

            <div class=ClassName::LINKS>

                <div class=ClassName::NOPAD>
                    <h2>"My links"</h2>
                    <ul>
                        <li>
                            <a href="https://github.com/thundertheidiot">"github"</a>
                        </li>
                        <li>
                            <a href="https://ch.tetr.io/u/thundertheidiot">"tetr.io"</a>
                        </li>
                    </ul>

                    <h2>"Contact"</h2>

                    <p style="margin:0px">"You can find me on"</p>
                    <p>
                        "XMPP: " <a>"thunder@kotiboksi.xyz"</a> <br /> "Discord: "
                        <a>"thundertheidiot"</a>
                    </p>

                </div>
            </div>
        </div>
    }
}
