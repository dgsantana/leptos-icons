#[cfg(feature = "BiRegularCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCard")]
/// *This icon requires the feature* `BiRegularCard` *to be enabled*.
#[component]
pub fn Card(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M17.999 17c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2h-12c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h12zm-12-12h12l.002 10H5.999V5zm-2 14h16v2h-16z" /></svg>
   }
}