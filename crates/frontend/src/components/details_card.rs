use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DetailsCardProps {
    pub title: String,
    pub children: Children,
}

#[function_component(DetailsCard)]
pub fn details_card(props: &DetailsCardProps) -> Html {
    html! {
    <div class="container mx-auto max-w-lg my-5">
      <div>
          <span class="flex items-center justify-between w-full p-5 font-medium text-left text-gray-900 bg-gray-100 border border-b-0 border-gray-200 rounded-t-xl focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-white dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-800">{props.title.clone()}</span>
      </div>
          {for props.children.iter()}
    </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DetailsHeaderItemProps {
    pub content: String,
}

#[function_component(DetailsHeaderItem)]
pub fn details_header_item(props: &DetailsHeaderItemProps) -> Html {
    html! {
        <p class="mb-2 text-gray-500 dark:text-gray-400">{props.content.clone()}</p>
    }
}

#[derive(Properties, PartialEq)]
pub struct DetailsHeaderProps {
    pub children: Children,
}

#[function_component(DetailsHeader)]
pub fn details_header(props: &DetailsHeaderProps) -> Html {
    html! {
      <div>
        <div class="p-5 font-light border border-b-0 border-gray-200 dark:border-gray-700 dark:bg-gray-900">
            {for props.children.iter()}
        </div>
      </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DetailsListProps {
    pub label: String,
    pub children: ChildrenWithProps<DetailsListItem>,
}

#[function_component(DetailsList)]
pub fn details_list(props: &DetailsListProps) -> Html {
    html! {
        <>
            <div>
                <span class="flex items-center justify-between w-full p-5 font-medium text-left text-gray-500 border border-b-0 border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800">{props.label.clone()}</span>
            </div>
            <div class="">
                <div class="p-5 font-light border  border-gray-200 dark:border-gray-700">
                    <ul role="list" class="divide-y divide-gray-200 dark:divide-gray-700">
                        {for props.children.iter()}
                    </ul>
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct DetailsListItemProps {
    pub label: String,
}

#[function_component(DetailsListItem)]
pub fn details_list_item(props: &DetailsListItemProps) -> Html {
    html! {
        <li class="py-3 sm:py-4">
            <div class="flex items-center space-x-4">
                <p class="text-sm font-medium text-gray-900 truncate dark:text-white">
                    {props.label.clone()}
                </p>
            </div>
        </li>
    }
}
