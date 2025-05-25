use crate::contexts::user_metadata::{use_publish_metadata, use_user_metadata_ctx};
use crate::models::NostrMetadata;
use crate::router::AnnotatorRoute;
use nostr_minions::widgets::upload_thing::ImageUploadInput;
use shady_minions::ui::{Button, ButtonType, Card, CardContent, CardHeader, CardTitle, Input};
use yew::prelude::*;
use yew_router::prelude::*;

/// Profile page component that directly interacts with relay events
#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    html! {
        <ProfilePageContent />
    }
}

#[function_component(ProfilePageContent)]
fn profile_page_content() -> Html {
    // Get language context and navigator
    let language_ctx = crate::contexts::language::use_language_ctx().clone();
    let navigator = use_navigator().expect("Navigator not available");

    // Getting metadata from the central UserMetadataStore
    let user_metadata_store = use_user_metadata_ctx();
    let profile = user_metadata_store.get_metadata().unwrap_or_default();

    let publish_metadata = use_publish_metadata();

    let key_ctx = nostr_minions::key_manager::use_nostr_id_ctx();

    // Form state - initialize with current values from profile
    let name = use_state(|| profile.name.clone());
    let about = use_state(|| profile.about.clone().unwrap_or_default());
    let picture_url = use_state(|| profile.picture.clone());

    let onsubmit = {
        let name = name.clone();
        let about = about.clone();
        let picture_url = picture_url.clone();
        let navigator = navigator.clone();
        let language_ctx = language_ctx.clone();
        let publish_metadata = publish_metadata.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let name = (*name).clone();
            let about = (*about).clone();
            let picture_url = (*picture_url).clone();
            let navigator = navigator.clone();
            let language_ctx = language_ctx.clone();

            // Creating the updated metadata object
            let updated_metadata = NostrMetadata::new(
                name.clone(),
                if about.is_empty() {
                    None
                } else {
                    Some(about.clone())
                },
                picture_url.clone(),
            );

            web_sys::console::log_1(
                &format!(
                    "Updating profile: name={}, about={:?}, picture={:?}",
                    updated_metadata.name, updated_metadata.about, updated_metadata.picture
                )
                .into(),
            );

            publish_metadata.emit(updated_metadata);

            // Loging profile update success
            web_sys::console::log_1(
                &format!(
                    "Profile successfully updated: {}",
                    language_ctx.t("profile_updated")
                )
                .into(),
            );

            // Navigating back after a short delay to allow relay to process
            gloo::timers::callback::Timeout::new(1000, move || {
                navigator.push(&AnnotatorRoute::Home);
            })
            .forget();
        })
    };

    // Input handlers that take String directly since Input component expects String callbacks
    let oninput_name = {
        let name = name.clone();
        Callback::from(move |value: String| {
            name.set(value);
        })
    };

    let oninput_about = {
        let about = about.clone();
        Callback::from(move |value: String| {
            about.set(value);
        })
    };

    let oninput_picture = {
        let picture_url = picture_url.clone();
        Callback::from(move |value: String| {
            if value.is_empty() {
                picture_url.set(None);
            } else {
                picture_url.set(Some(value));
            }
        })
    };

    let oncancel = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AnnotatorRoute::Home);
        })
    };

    html! {
        <div class="container px-3 xs:px-4 py-6">
            <header class="flex justify-between items-center mb-6">
            <Button
                    onclick={Callback::from(move |_| navigator.back())}
                    variant={shady_minions::ui::ButtonVariant::Outline}
                >
                    <lucide_yew::ArrowLeft class="size-4 mr-2" />
                    { language_ctx.t("common_back") }
                </Button>
            </header>

            <Card class="max-w-md mx-auto">
                <CardHeader>
                    <CardTitle>
                        { language_ctx.t("edit_profile") }
                    </CardTitle>
                </CardHeader>

                <CardContent>
                    <form {onsubmit} class="space-y-4">
                        <div class="flex flex-col space-y-2">
                            <label class="text-sm font-medium">
                                { language_ctx.t("profile_name") }
                            </label>
                            <Input
                                value={(*name).clone()}
                                oninput={oninput_name}
                                required={true}
                                placeholder={ language_ctx.t("profile_name_placeholder") }
                            />
                        </div>

                        <div class="flex flex-col space-y-2">
                            <label class="text-sm font-medium">
                                { language_ctx.t("profile_about") }
                            </label>
                            <Input
                                value={(*about).clone()}
                                oninput={oninput_about}
                                placeholder={ language_ctx.t("profile_about_placeholder") }
                            />
                        </div>

                        <div class="flex flex-col space-y-2">
                            <label class="text-sm font-medium">
                                { language_ctx.t("profile_picture_url") }
                            </label>
                            {if let Some(identity) = key_ctx.get_identity() {
                                html! {
                                    <ImageUploadInput
                                        url_handle={picture_url.clone()}
                                        nostr_keys={identity.clone()}
                                        classes={classes!("w-full", "h-32")}
                                        image_classes={classes!("w-full", "h-32", "object-cover")}
                                        input_id={"profile-picture-upload"}
                                        server_pubkey={"npub1fljn6rv3dnxnjl472a2l8xwngsvesjjmf53f38keuc3v8u2e9taq9q7j0m"}
                                    />
                                }
                            } else {
                                html! {
                                    <Input
                                        value={(*picture_url).clone().unwrap_or_default()}
                                        oninput={oninput_picture}
                                        placeholder={ language_ctx.t("profile_picture_url") }
                                    />
                                }
                            }}
                        </div>

                        <div class="flex justify-between pt-4 border-t">
                            <Button
                                onclick={oncancel}
                                variant={shady_minions::ui::ButtonVariant::Outline}
                            >
                                { language_ctx.t("common_cancel") }
                            </Button>

                            <Button r#type={ButtonType::Submit}>
                                { language_ctx.t("common_save") }
                            </Button>
                        </div>
                    </form>
                </CardContent>
            </Card>
        </div>
    }
}
