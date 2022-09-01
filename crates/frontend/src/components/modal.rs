use crate::theme::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub msg: String,
    pub confirm_action: Callback<MouseEvent>,
    pub show_modal: UseStateHandle<bool>,
}
// <div id="popup-modal" tabindex="-1" class="hidden overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 md:inset-0 h-modal md:h-full">
//     <div class="relative p-4 w-full max-w-md h-full md:h-auto">
//         <div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
//             <div class="p-6 text-center">
//                 <svg aria-hidden="true" class="mx-auto mb-4 w-14 h-14 text-gray-400 dark:text-gray-200" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
//                 <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">Are you sure you want to delete this product?</h3>
//                 <button data-modal-toggle="popup-modal" type="button" class="text-white bg-red-600 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 dark:focus:ring-red-800 font-medium rounded-lg text-sm inline-flex items-center px-5 py-2.5 text-center mr-2">
//                     Yes, I'm sure
//                 </button>
//                 <button data-modal-toggle="popup-modal" type="button" class="text-gray-500 bg-white hover:bg-gray-100 focus:ring-4 focus:outline-none focus:ring-gray-200 rounded-lg border border-gray-200 text-sm font-medium px-5 py-2.5 hover:text-gray-900 focus:z-10 dark:bg-gray-700 dark:text-gray-300 dark:border-gray-500 dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-gray-600">No, cancel</button>
//             </div>
//         </div>
//     </div>
// </div>

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
