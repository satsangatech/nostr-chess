use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ButtonVariant {
    #[default]
    Normal,
    Outline,
    Destructive,
    Disabled,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ButtonSize {
    Small,
    Large,
    #[default]
    Regular,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ButtonType {
    Submit,
    #[default]
    Button,
    Reset,
}
impl std::fmt::Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Submit => write!(f, "submit"),
            Self::Button => write!(f, "button"),
            Self::Reset => write!(f, "reset"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub r#type: ButtonType,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(Callback::noop())]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let variant_class = match props.variant {
        ButtonVariant::Outline => [
            "border-2",
            "border-secondary",
            "bg-background",
            "text-secondary",
            "hover:bg-muted",
            "hover:text-muted-foreground",
        ]
        .as_slice(),
        ButtonVariant::Destructive => [
            "bg-destructive",
            "text-destructive-foreground",
            "hover:bg-destructive/80",
        ]
        .as_slice(),
        ButtonVariant::Normal => [
            "bg-primary",
            "text-primary-foreground",
            "hover:bg-primary/80",
        ]
        .as_slice(),
        ButtonVariant::Disabled => &[
            "bg-muted",
            "text-muted-foreground",
            "hover:bg-muted/80",
            "pointer-events-none",
        ],
    };

    let size_class = match props.size {
        ButtonSize::Small => ["h-9", "rounded-md", "px-3"].as_slice(),
        ButtonSize::Large => ["h-11", "rounded-md", "px-8"].as_slice(),
        ButtonSize::Regular => ["h-10", "px-4", "py-2", "rounded-md"].as_slice(),
    };

    let classes = classes!(
        "inline-flex",
        "items-center",
        "justify-center",
        "whitespace-nowrap",
        "cursor-pointer",
        "font-medium",
        "transition-colors",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:opacity-50",
        "disabled:pointer-events-none",
        "ring-offset-background",
        variant_class,
        size_class,
        props.class.clone()
    );

    html! {
        <button
            disabled={props.disabled}
            class={classes}
            type={props.r#type.to_string()}
            onclick={props.onclick.clone()}
        >
            {props.children.clone()}
        </button>
    }
}
