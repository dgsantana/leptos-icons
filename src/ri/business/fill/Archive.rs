#[cfg(feature = "RiBusinessFillArchive")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessFillArchive")]
/// *This icon requires the feature* `RiBusinessFillArchive` *to be enabled*.
#[component]
pub fn Archive(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 10h18v10.004c0 .55-.445.996-.993.996H3.993A.994.994 0 0 1 3 20.004V10zm6 2v2h6v-2H9zM2 4c0-.552.455-1 .992-1h18.016c.548 0 .992.444.992 1v4H2V4z" /></g></svg>
   }
}