#[cfg(feature = "BiRegularLaptop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLaptop")]
/// *This icon requires the feature* `BiRegularLaptop` *to be enabled*.
#[component]
pub fn Laptop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 17.722c.595-.347 1-.985 1-1.722V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v11c0 .736.405 1.375 1 1.722V18H2v2h20v-2h-2v-.278zM5 16V5h14l.002 11H5z" /></svg>
   }
}