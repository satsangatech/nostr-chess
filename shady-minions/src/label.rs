use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub r#for: String,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let classes = classes!(
        "text-sm",
        "font-medium",
        "leading-none",
        "peer-disabled:cursor-not-allowed",
        "peer-disabled:opacity-70",
        "select-none",         // Prevent text selection
        "pointer-events-none", // Prevent clicking / hovering
        props.class.clone()
    );

    html! {
        <label
            class={classes}
            for={props.r#for.clone()}
        >
            {props.children.clone()}
        </label>
    }
}
