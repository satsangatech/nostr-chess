use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(Callback::noop())]
    pub onsubmit: Callback<web_sys::HtmlFormElement>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps {
        class,
        children,
        onsubmit,
    } = props;

    let onsubmit_handler = {
        let onsubmit = onsubmit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Prevent page reload
            let form = e.target_unchecked_into::<web_sys::HtmlFormElement>();
            onsubmit.emit(form);
        })
    };

    let classes = classes!(
        "space-y-2", // ShadCN-style vertical spacing
        class.clone(),
    );

    html! {
        <form class={classes} onsubmit={onsubmit_handler}>
            { for children.iter() }
        </form>
    }
}
