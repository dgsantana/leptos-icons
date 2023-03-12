#[cfg(feature = "FiTwitch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiTwitch")]
/// *This icon requires the feature* `FiTwitch` *to be enabled*.
#[component]
pub fn Twitch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7" /></svg>
   }
}