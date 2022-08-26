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
    let show_classes = classes!("modal", "fade", "show");
    let show_style = "display: block;";
    let hide_classes = classes!("modal", "fade");
    let hide_style = "display: none;";

    let onclick_cancel = {
        let show = props.show_modal.clone();
        Callback::once(move |_: MouseEvent| {
            show.set(false);
        })
    };

    html! {
    <div class={
            match (*props.show_modal).clone() {
                true => show_classes.clone(),
                false => hide_classes.clone(),
            }
        }
        style={
            match (*props.show_modal).clone() {
                true => show_style.clone(),
                false => hide_style.clone(),
            }
        }
        tabindex="-1"
    >
      <div class="modal-dialog">
        <div class="modal-content bg-dark text-light">
          <div class="modal-header">
          <h5 class="modal-title">{props.title.clone()}</h5>
          </div>
          <div class="modal-body">
          <p>{props.msg.clone()}</p>
          </div>
          <div class="modal-footer">
          <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" onclick={onclick_cancel}>{"Cancel"}</button>
          <button type="button" class="btn btn-danger" onclick={props.confirm_action.clone()}>{"Confirm"}</button>
          </div>
        </div>
      </div>
    </div>
    }
}
