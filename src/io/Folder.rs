#[cfg(feature = "IoFolder")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFolder")]
/// *This icon requires the feature* `IoFolder` *to be enabled*.
#[component]
pub fn Folder(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M496,152a56,56,0,0,0-56-56H220.11a23.89,23.89,0,0,1-13.31-4L179,73.41A55.77,55.77,0,0,0,147.89,64H72a56,56,0,0,0-56,56v48a8,8,0,0,0,8,8H488a8,8,0,0,0,8-8Z" /><path d="M16,392a56,56,0,0,0,56,56H440a56,56,0,0,0,56-56V216a8,8,0,0,0-8-8H24a8,8,0,0,0-8,8Z" /></svg>
   }
}