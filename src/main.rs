mod components;
mod routes;
pub mod app;

use crate::{routes::{switch, Route}, app::App};
use rust_i18n::i18n;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::components::{footer::Footer, navbar::Navbar, aside::Aside};

i18n!("locales");

#[function_component]
fn Main() -> Html {
    html! {
    <>
        <BrowserRouter>
            <div class="antialiased bg-gray-50 dark:bg-gray-900 flex flex-col min-h-screen">
                <Navbar />
                <Aside />
                <main class="p-4 md:ml-64 h-auto pt-20 flex-grow">
                    <Switch<Route> render={switch} />
                </main>


                <Footer />
            </div>
        </BrowserRouter>
    </>
    }
}

fn main() {
    rust_i18n::set_locale("en");
    yew::Renderer::<Main>::new().render();
}
