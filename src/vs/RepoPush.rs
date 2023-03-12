#[cfg(feature = "VsRepoPush")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRepoPush")]
/// *This icon requires the feature* `VsRepoPush` *to be enabled*.
#[component]
pub fn RepoPush(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M13.5 1H3.74A1.74 1.74 0 0 0 2 2.75v9.5A1.74 1.74 0 0 0 3.74 14H7v-1H3.74a.74.74 0 0 1-.74-.75v-.5a.74.74 0 0 1 .74-.75H7v-1H4V2h9v8h-3v1h3v2h-3v1h3.5l.5-.5v-12l-.5-.5zM3 2.73a.75.75 0 0 0 0 .02v7.42-7.44zM6 3H5v1h1V3zm-.62 5.65l.71.7 1.92-1.92V15h1V7.328l2.03 2.022.7-.7-2.82-2.83h-.71L5.38 8.65zM5 5h1v1H5V5z" /></svg>
   }
}