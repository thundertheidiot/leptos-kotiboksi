use crate::css::ClassName;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <div class=ClassName::TEXTPART>

            <h1>"Hi"</h1>

            <p>
            "I'm thunder, welcome to my website "
            </p>

        <h2>"About"</h2>

        <p>
        "I like computers and tinkering with them, i don't have much else going on in my life."
        </p>

                <p>
                    "I've been using (GNU/)Linux since 2017, i currently use "
                    <a href="https://nixos.org">"NixOS"</a>
                    ", which is a pretty niche distro that allows you to declaratively configure pretty much everything using nix. "
            <a href="https://github.com/thundertheidiot/nixdots">"Here"</a>

                    " is my slowly growing monstrocity of a config."
                </p>

            <p>
            "My favourite language for general purpose programming is Rust. "
            "I have a basic understanding of C and C++, but I'm not a huge fan. "
            </p>

            <p>
            "I also like trying more obscure languages languages like Haskell"
            ", but i haven't really used them outside of small utility programs, some of which can be found embedded in my nix configuration. "
            "I also like Nix (the language)."
            </p>

            </div>

            <br />

            <div class=ClassName::TEXTPART>
                <h2>"This website"</h2>

                <p>
                    "This site was made with the "
                    <a href="https://leptos-rs.github.io/leptos/">"leptos framework"</a>
            ", because i wanted to program in Rust. "
        "All of this could easily be created with basic html, a sprinkle of js for the radio and a tiny bit of php for the guestbook, "
        "in fact that is what i used previously."
                </p>

        <p>
            "Thanks to leptos' server side rendering feature, there aren't really any downsides to using it here, "
        "even with WASM disabled most of the page (everything \"important\") works. "
        "The WASM blob is very small compared to other modern websites."
        </p>

                <p>
                    "The pixel font is IBM VGA 8x16, and you can find it "
                    <a href="https://int10h.org/oldschool-pc-fonts/">"here"</a> "."
                </p>

            </div>

            <br />

            <div class=ClassName::TEXTPART>

            <a href="https://cyber.dabamos.de/88x31"><img src="/static/img/buttons/88x31.gif" alt="the buttons" /></a>
            " "
            <a href="https://nixos.org"><img src="/static/img/buttons/nixos.png" alt="nixos" /></a>
            " "
            <a href="https://cartercoding.com"><img src="/static/img/buttons/carter.webp" alt="carer i am carer" /></a>
            " "
            <a href="https://corru.observer"><img src="/static/img/buttons/corru-observer.gif" alt="corru.observer" /></a>

        <br style="margin: 10px;"/>

        <img src="/static/img/buttons/gnu-linux.gif" alt="gnu/linux" />
            " "
            <img src="/static/img/buttons/emacs.gif" alt="emacs" />
            " "
            <img src="/static/img/buttons/pride.gif" alt="pridenow" />
            " "
            <img src="/static/img/buttons/transnow2.gif" alt="transnow" />
            " "

    // when published
        // <img src="/static/img/buttons/agplv3-88x31.gif" alt="agplv3" />
        // " "

            </div>

        <br/>

            <div class=ClassName::LINKS>

        <div class=ClassName::NOPAD>
        <h2>"My links"</h2>
        <ul>
        <li><a href="https://github.com/thundertheidiot">"github"</a></li>
            </ul>

                <h2>"Contact"</h2>

                <p style="margin:0px">"You can find me on"</p>
                <p>
                    "XMPP: " <a>"thunder@kotiboksi.xyz"</a> <br /> "Discord: " <a>"thundertheidiot"</a>
            </p>

            </div>
        // <div class=ClassName::PROFILE>
        //     <img src="/static/img/profile.webp"/>
        // </div>
        </div>
        </div>
    }
}
