#[cfg(feature = "BiSolidArrowToTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowToTop")]
/// *This icon requires the feature* `BiSolidArrowToTop` *to be enabled*.
#[component]
pub fn ArrowToTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 4h12v2H6zm5 10v6h2v-6h5l-6-6-6 6z" /></svg>
   }
}