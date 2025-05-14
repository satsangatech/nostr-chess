use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Routable)]
pub enum AnnotatorRoute {
    #[at("/")]
    Home,
    #[at("/keys")]
    KeySettings,
    #[at("/network")]
    RelaySettings,
}

#[function_component(AnnotatorRouter)]
pub fn annotator_router() -> Html {
    html! {
        <Switch<AnnotatorRoute> render = { move |switch: AnnotatorRoute| {
            match switch {
                AnnotatorRoute::Home => html! { <crate::HomePage /> },
                AnnotatorRoute::KeySettings => html! { <h1>{ "Key Settings" }</h1> },
                AnnotatorRoute::RelaySettings => html! { <h1>{ "Relay Settings" }</h1> },
            }}}
        />

    }
}
