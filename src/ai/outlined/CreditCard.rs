#[cfg(feature = "AiOutlinedCreditCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedCreditCard")]
/// *This icon requires the feature* `AiOutlinedCreditCard` *to be enabled*.
#[component]
pub fn CreditCard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M928 160H96c-17.7 0-32 14.3-32 32v640c0 17.7 14.3 32 32 32h832c17.7 0 32-14.3 32-32V192c0-17.7-14.3-32-32-32zm-792 72h752v120H136V232zm752 560H136V440h752v352zm-237-64h165c4.4 0 8-3.6 8-8v-72c0-4.4-3.6-8-8-8H651c-4.4 0-8 3.6-8 8v72c0 4.4 3.6 8 8 8z" /></svg>
   }
}