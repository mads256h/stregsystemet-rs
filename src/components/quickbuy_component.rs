use maud::{html, Markup};

pub fn quickbuy_component() -> Markup {
    html! {
        div class="quickbuy-container" {
            form autocomplete="off" action="/buy/" method="post" {
                p {
                    label for="quickbuy" { "Quickbuy" }
                    input tabindex="1" type="text" name="quickbuy" id="quickbuy" autofocus;
                    input tabindex="2" type="submit" value="Køb!";
                }
            }
        }
    }
}
