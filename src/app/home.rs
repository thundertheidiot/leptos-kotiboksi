use crate::css::ClassName;
use leptos::IntoView;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class=ClassName::TEXTPART>
            <h1>"About me:"</h1>

            <p>
                "I like tinkering with computers, they are my only real hobby. "
                "I've used (GNU/)Linux since 2017, I currently use "
                <a href="https://nixos.org">"NixOS"</a>
                ", which is a pretty niche distro that allows you to declaratively configure pretty much everything. "
                <a href="https://gitlab.com/thundertheidiot/nixdots">"Here"</a>
                " is my slowly growing monstrocity of a config."
            </p>

            <h2>"Languages:"</h2>

            <p>
                "Currently my favourite language for general purpose programming is Rust. "
                "I have a basic understanding of C and C++, but I prefer Rust."
                "I also like trying \"weird\" languages like Haskell or "
                <a href="https://janet-lang.org">"Janet"</a>
                ", but I haven't really used them outside of small utilities."
            </p>

            <p>
                "I also really like working with the Nix language, and I don't hate Emacs Lisp either."
            </p>

        </div>

        <br/>

        <div class=ClassName::TEXTPART>
            <h1>"This website:"</h1>

            <p>
                "The website is made with the "
                <a href="https://leptos-rs.github.io/leptos/">"leptos framework"</a>
                ", because I wanted to make something with Rust. "
                "There isn't really anything here that i couldn't make with "
                <em>"insert javascript framework"</em> ", i just like working with Rust."
            </p>

            <p>
                " The font is IBM VGA 8x16, and you can find it "
                <a href="https://int10h.org/oldschool-pc-fonts/">"here"</a> "."
            </p>
        </div>

        <br/>

        <div class=ClassName::TEXTPART>
            <h1>"Other stuff:"</h1>

            <p>
                "I am running a " <a href="https://chat.kotiboksi.xyz">"matrix"</a>
                " and xmpp server on here in addition to this website. Feel free to sign up to either, registrations are open."
            </p>

            <h2>"Contact:"</h2>

            <p style="margin:0px">"You can find me on"</p>
            <p>
                "XMPP: " <a>"thunder@gooptyland.xyz"</a> <br/> "Matrix: "
                <a>"@thunder:kotiboksi.xyz"</a> <br/> "Discord: " <a>"thundertheidiot"</a>
                " (I would prefer the above options)"
            </p>
        </div>
    }
}
