use crate::contexts::user_metadata::use_user_metadata_ctx;
use yew::prelude::*;

/// A profile card component that uses the UserMetadataProvider context
#[function_component(UserProfileCard)]
pub fn user_profile_card() -> Html {
    // Get contexts needed
    let language_ctx = crate::contexts::language::use_language_ctx();
    // let navigator = use_navigator().expect("Navigator not available");
    let user_metadata_store = use_user_metadata_ctx();

    // Getting metadata from the central store - no local state needed
    let metadata = user_metadata_store.get_metadata();

    // On edit button click
    // let onclick = {
    //     let navigator = navigator.clone();
    //     Callback::from(move |_| {
    //         navigator.push(&AnnotatorRoute::Profile);
    //     })
    // };

    html! {
        <div class="rounded-lg shadow-sm space-y-1">
            <div class="mb-2 xs:mb-3">
                <h3 class="text-base xs:text-lg font-semibold text-muted mb-0.5 xs:mb-1">
                    { language_ctx.t("profile_title") }
                </h3>
                <p class="text-xs xs:text-sm text-muted-foreground">
                    { language_ctx.t("profile_description") }
                </p>
            </div>

            <div class="flex items-center gap-3 mb-3">
                <div class="w-12 h-12 xs:w-14 xs:h-14 rounded-full overflow-hidden border border-slate-200 flex-shrink-0">
                    <img
                        src={metadata.as_ref().and_then(|p| p.picture.clone())
                            .unwrap_or_else(|| "/public/assets/img/default-avatar.png".to_string())}
                        alt="Profile"
                        class="w-full h-full object-cover"
                    />
                </div>
                <div class="flex-grow min-w-0">
                    <p class="font-medium text-sm xs:text-base truncate text-muted">
                        {metadata.as_ref().map(|p| p.name.clone()).unwrap_or_else(|| language_ctx.t("anonymous_user"))}
                    </p>
                    <p class="text-xs xs:text-sm text-muted-foreground truncate italic">
                        {metadata.as_ref().and_then(|p| p.about.clone()).unwrap_or_else(|| language_ctx.t("no_bio"))}
                    </p>
                </div>
            </div>

            // <Button
            //     {onclick}
            //     variant={shady_minions::ui::ButtonVariant::Outline}
            //     class="w-full text-sm xs:text-base"
            // >
            //     <lucide_yew::Pen class="size-4 mr-2" />
            //     { language_ctx.t("edit_profile") }
            // </Button>
        </div>
    }
}
