#[cfg(feature = "BiRegularBookmarkMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBookmarkMinus")]
/// *This icon requires the feature* `BiRegularBookmarkMinus` *to be enabled*.
#[component]
pub fn BookmarkMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 9h8v2H8z" /><path d="M20 22V4c0-1.103-.897-2-2-2H6c-1.103 0-2 .897-2 2v18l8-4.572L20 22zM6 10V4h12v14.553l-6-3.428-6 3.428V10z" /></svg>
   }
}