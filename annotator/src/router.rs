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
    #[at("/profile")]
    Profile,
    #[at("/review")]
    Review,
}

#[function_component(AnnotatorRouter)]
pub fn annotator_router() -> Html {
    html! {
        <Switch<AnnotatorRoute> render = { move |switch: AnnotatorRoute| {
            match switch {
                AnnotatorRoute::Home => html! { <crate::HomePage /> },
                AnnotatorRoute::KeySettings => html! { <crate::KeyRecoveryPage /> },
                AnnotatorRoute::RelaySettings => html! { <crate::RelayManagementPage /> },
                AnnotatorRoute::Profile => html! { <crate::ProfilePage /> },
                AnnotatorRoute::Review => html! { <crate::ReviewPage /> },
            }}}
        />

    }
}
