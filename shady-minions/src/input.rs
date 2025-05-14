use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    #[default]
    Text,
    Time,
    Url,
    Week,
}
impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Button => write!(f, "button"),
            Self::Checkbox => write!(f, "checkbox"),
            Self::Color => write!(f, "color"),
            Self::Date => write!(f, "date"),
            Self::DatetimeLocal => write!(f, "datetime-local"),
            Self::Email => write!(f, "email"),
            Self::File => write!(f, "file"),
            Self::Hidden => write!(f, "hidden"),
            Self::Image => write!(f, "image"),
            Self::Month => write!(f, "month"),
            Self::Number => write!(f, "number"),
            Self::Password => write!(f, "password"),
            Self::Radio => write!(f, "radio"),
            Self::Range => write!(f, "range"),
            Self::Reset => write!(f, "reset"),
            Self::Search => write!(f, "search"),
            Self::Submit => write!(f, "submit"),
            Self::Tel => write!(f, "tel"),
            Self::Text => write!(f, "text"),
            Self::Time => write!(f, "time"),
            Self::Url => write!(f, "url"),
            Self::Week => write!(f, "week"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub r#type: InputType,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or_default]
    pub min: String,
    #[prop_or_default]
    pub max: String,
    #[prop_or_default]
    pub step: String,
    #[prop_or(Callback::noop())]
    pub onchange: Callback<String>,
    #[prop_or(Callback::noop())]
    pub oninput: Callback<String>,
    #[prop_or(Callback::noop())]
    pub onclick: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let has_text_size = props.class.contains("text-sm")
        || props.class.contains("text-base")
        || props.class.contains("text-lg")
        || props.class.contains("text-xl")
        || props.class.contains("text-2xl")
        || props.class.contains("text-3xl")
        || props.class.contains("text-4xl")
        || props.class.contains("text-5xl")
        || props.class.contains("text-6xl");
    let classes = classes!(
        "flex",
        "h-10",
        "w-full",
        "rounded-md",
        "border",
        "border-input",
        "bg-background",
        if has_text_size { "" } else { "text-sm" },
        "px-3",
        "py-2",
        "text-foreground",
        "placeholder:text-muted-foreground",
        "ring-offset-background",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:cursor-not-allowed",
        "disabled:opacity-50",
        props.class.clone()
    );
    let onchange = {
        let callback = props.onchange.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };
    let oninput = {
        let callback = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };
    let onclick = {
        let callback = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            callback.emit(input.value());
        })
    };

    html! {
        <input
            class={classes}
            id={props.id.clone()}
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            type={props.r#type.to_string()}
            value={props.value.clone()}
            min={props.min.clone()}
            max={props.max.clone()}
            step={props.step.clone()}
            {onchange}
            {oninput}
            {onclick}
            disabled={props.disabled}
            required={props.required}
        />
    }
}
