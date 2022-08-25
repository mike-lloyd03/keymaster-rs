use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    pub msg: String,
    pub confirm_action: Callback<MouseEvent>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let classes = use_state(|| classes!("modal", "fade", "show"));
    let style = use_state(|| "display: block;");

    let onclose = {
        let classes = classes.clone();
        // let style = style.clone();
        Callback::from(move |_: MouseEvent| {
            classes.set(classes!("modal", "fade"));
            // style.set("display: none;");
        })
    };

    // let onconfirm = {
    //     let confirm_action = props.confirm_action.clone();
    //     let onclose = onclose.clone();
    //     Callback::from(move |_: MouseEvent| {
    //         confirm_action;
    //         // onclose;
    //     })
    // };

    html! {
    <div class={(*classes).clone()} tabindex="-1" style={(*style).clone()}>
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
          <h5 class="modal-title">{props.title.clone()}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
          </div>
          <div class="modal-body">
          <p>{props.msg.clone()}</p>
          </div>
          <div class="modal-footer">
          <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" onclick={onclose}>{"Close"}</button>
          <button type="button" class="btn btn-danger" onclick={props.confirm_action.clone()}>{"Confirm"}</button>
          </div>
        </div>
      </div>
    </div>
    }
}
