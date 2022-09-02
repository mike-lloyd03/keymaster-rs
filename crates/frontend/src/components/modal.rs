use crate::theme::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub msg: String,
    pub confirm_action: Callback<MouseEvent>,
    pub show_modal: UseStateHandle<bool>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let onclick_cancel = {
        let show = props.show_modal.clone();
        Callback::once(move |_: MouseEvent| {
            show.set(false);
        })
    };

    let cl_base = classes!(
        "overflow-y-auto",
        "overflow-x-hidden",
        "fixed",
        "top-0",
        "right-0",
        "left-0",
        "z-50",
        "md:inset-0",
        "md:h-full",
    );

    html! {
        <div
            class={
                match (*props.show_modal).clone() {
                    true => cl_base,
                    false => classes!(cl_base, "hidden"),
                }
            }
            tabindex="-1"
        >

            <div class="relative py-20 w-full max-w-lg h-full mx-auto">
                <div class="relative rounded-lg shadow bg-gray-700">
                    <div class="p-6 text-center">
                        <svg aria-hidden="true" class="mx-auto mb-4 w-14 h-14 text-gray-400 dark:text-gray-200" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                        <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">{props.msg.clone()}</h3>
                        <button type="button" class={classes!(BTN, BTN_DANGER)} onclick={props.confirm_action.clone()}>{"Confirm"}</button>
                        <button type="button" class={classes!(BTN, BTN_SECONDARY_OUTLINE)} data-bs-dismiss="modal" onclick={onclick_cancel}>{"Cancel"}</button>
                    </div>
                </div>
            </div>

        </div>
    }
}
