pub static TEXT_BLUE: &'static str = "text-blue-600";
pub static TEXT_GRAY: &'static str = "text-gray-400";
pub static TEXT_DARK: &'static str = "text-slate-200";
pub static TEXT_LIGHT: &'static str = "text-slate-400";
pub static BG_PRIME_DARK: &'static str = "bg-slate-800";
pub static BG_PRIME_MD_DARK: &'static str = "bg-slate-700";
pub static BG_SEC_DARK: &'static str = "bg-gray-800";
pub static BG_SEC_MD_DARK: &'static str = "bg-gray-700";

pub static BTN: &'static str =
    "text-white font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 cursor-pointer";
pub static BTN_PRIMARY: &'static str =
    "bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-blue-800";
pub static BTN_SECONDARY: &'static str =
    "bg-gray-500 hover:bg-gray-600 focus:outline-none focus:ring-gray-700";
pub static BTN_DANGER: &'static str =
    "bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-red-800";
pub static BTN_PRIMARY_OUTLINE: &'static str =
    "border border-blue-500 hover:bg-blue-700 text-blue-500 hover:text-slate-300";
pub static BTN_SECONDARY_OUTLINE: &'static str =
    "border border-gray-500 hover:bg-gray-700 text-gray-500 hover:text-slate-300 hover:bg-gray-400";

pub fn hover(class: &str) -> String {
    format!("hover:{}", class)
}

//////////////////
// Details Card
//////////////////
pub static DETAIL_CARD: &'static str = "
    flex
    items-center
    justify-between
    w-full
    p-5
    font-medium
    text-left
    border
    border-b-0
    rounded-t-xl
    focus:ring-4
    focus:ring-gray-800
    border-gray-700
    text-white
    bg-gray-700
    ";

pub static DETAIL_HEADER: &'static str = "
    p-5
    font-light
    border
    border-b-0
    border-gray-700
    bg-gray-800
    ";

pub static DETAIL_HEADER_ITEM: &'static str = "
    mb-2
    text-gray-500
    dark:text-gray-400
    ";

pub static DETAIL_LIST_CONTAINER: &'static str = "
    font-light
    border
    border-gray-700
    ";

pub static DETAIL_LIST: &'static str = "
    flex
    items-center
    justify-between
    w-full
    p-5
    font-medium
    text-left
    border
    border-b-0
    focus:ring-4
    focus:ring-gray-800
    border-gray-700
    text-white
    bg-gray-700
    ";

pub static DETAIL_LIST_ITEM_ROW: &'static str = "
    text-gray-500
    hover:text-white
    ";

pub static DETAIL_LIST_ITEM_LINK: &'static str = "
    p-2
    inline-block
    text-sm
    truncate
    font-light
    flex
    hover:bg-gray-700
    bg-gray-800
    border
    border-x-0
    border-gray-800
    hover:border-gray-800
    ";

pub static DETAIL_FOOTER: &'static str = "
    border
    border-b-0
    rounded-b-xl
    border-gray-700
    text-white
    bg-gray-700
    h-2
    ";
