use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub required: bool,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let SwitchProps {
        checked,
        disabled,
        onchange,
        class,
        required,
    } = (*props).clone();

    let toggle = {
        Callback::from(move |_| {
            onchange.emit(!checked);
        })
    };

    let classes = classes!(
        "relative",
        "inline-flex",
        "items-center",
        "h-6",
        "w-11",
        "cursor-pointer",
        "rounded-full",
        "transition-colors",
        "bg-input",
        "ring-offset-background",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:opacity-50",
        "disabled:cursor-not-allowed",
        if checked { "bg-primary" } else { "bg-input" },
        class
    );

    let circle_class = classes!(
        "inline-block",
        "h-4",
        "w-4",
        "transform",
        "rounded-full",
        "bg-background",
        "transition-transform",
        if checked {
            "translate-x-6"
        } else {
            "translate-x-1"
        },
    );

    html! {
        <button type="button" class={classes} onclick={toggle} {disabled} {required}>
            <span class={circle_class} />
        </button>
    }
}
