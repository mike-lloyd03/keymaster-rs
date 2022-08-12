use yew::prelude::*;

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    html! {
        <td>{ props.value.clone() }</td>
    }
}

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub heading: String,
    pub value: String,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
    html! {
        <tr>{ for props.children.iter() }</tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub children: ChildrenWithProps<Cell>,
}

pub struct Table;

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let headings: Vec<String> = ctx
            .props()
            .children
            .iter()
            .next()
            .unwrap()
            .props
            .children
            .iter()
            .map(|cell| cell.props.heading.clone())
            .collect();

        html! {
            <div style="text-align: center">
                <h2>{ ctx.props().title.clone() }</h2>
                    <table class="table table-striped table-hover table-bordered table-dark">
                        <thead class="table-dark">
                            <tr>
                                { for headings.iter().map(|heading| html!{<th>{ heading.clone() }</th>}) }
                            </tr>
                        </thead>
                        <tbody>
                            { for ctx.props().children.iter()}
                        </tbody>
                    </table>
            </div>
        }
    }
}

// #[function_component(Table)]
// pub fn table(props: &TableProps) -> Html {
//     let headings: Vec<String> = props
//         .children
//         .iter()
//         .next()
//         .unwrap()
//         .props
//         .children
//         .iter()
//         .map(|cell| cell.props.heading.clone())
//         .collect();

//     html! {
//         <div style="text-align: center">
//             <h2>{ props.title.clone() }</h2>
//                 <table class="table table-striped table-hover table-bordered table-dark">
//                     <thead class="table-dark">
//                         <tr>
//                             { for headings.iter().map(|heading| html!{<th>{ heading.clone() }</th>}) }
//                         </tr>
//                     </thead>
//                     <tbody>
//                         { for props.children.iter()}
//                     </tbody>
//                 </table>
//         </div>
//     }
// }

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub title: String,
    pub children: ChildrenWithProps<Row>,
}
