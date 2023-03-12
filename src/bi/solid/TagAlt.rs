#[cfg(feature = "BiSolidTagAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTagAlt")]
/// *This icon requires the feature* `BiSolidTagAlt` *to be enabled*.
#[component]
pub fn TagAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21.868 11.504-4-7A1 1 0 0 0 17 4H3a1 1 0 0 0-.868 1.496L5.849 12l-3.717 6.504A1 1 0 0 0 3 20h14a1 1 0 0 0 .868-.504l4-7a.998.998 0 0 0 0-.992z" /></svg>
   }
}