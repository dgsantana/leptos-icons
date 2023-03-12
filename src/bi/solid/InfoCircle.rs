#[cfg(feature = "BiSolidInfoCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidInfoCircle")]
/// *This icon requires the feature* `BiSolidInfoCircle` *to be enabled*.
#[component]
pub fn InfoCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z" /></svg>
   }
}