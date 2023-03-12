#[cfg(feature = "BiRegularChevronDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularChevronDown")]
/// *This icon requires the feature* `BiRegularChevronDown` *to be enabled*.
#[component]
pub fn ChevronDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.293 9.293 12 13.586 7.707 9.293l-1.414 1.414L12 16.414l5.707-5.707z" /></svg>
   }
}