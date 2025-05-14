use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub rows: u32,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
}

#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let onchange = {
        let onchange = props.onchange.clone();
        Callback::from(move |e: Event| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            onchange.emit(textarea.value());
        })
    };

    let classes = classes!(
        "flex",
        "w-full",
        "rounded-md",
        "border",
        "border-input",
        "bg-background",
        "px-3",
        "py-2",
        "text-sm",
        "text-foreground",
        "placeholder:text-muted-foreground",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:opacity-50",
        "disabled:cursor-not-allowed",
        "ring-offset-background",
        props.class.clone()
    );

    html! {
        <textarea
            id={props.id.clone()}
            name={props.name.clone()}
            class={classes}
            value={props.value.clone()}
            onchange={onchange}
            placeholder={props.placeholder.clone()}
            rows={props.rows.to_string()}
            disabled={props.disabled}
            required={props.required}
        />
    }
}
