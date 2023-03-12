#[cfg(feature = "CgMathEqual")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgMathEqual")]
/// *This icon requires the feature* `CgMathEqual` *to be enabled*.
#[component]
pub fn MathEqual(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 9C4.44772 9 4 9.44771 4 10C4 10.5523 4.44772 11 5 11H19C19.5523 11 20 10.5523 20 10C20 9.44771 19.5523 9 19 9H5Z" fill="currentColor" /><path d="M5 13C4.44772 13 4 13.4477 4 14C4 14.5523 4.44772 15 5 15H19C19.5523 15 20 14.5523 20 14C20 13.4477 19.5523 13 19 13H5Z" fill="currentColor" /></svg>
   }
}