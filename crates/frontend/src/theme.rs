use yew::{classes, Classes};

pub static TEXT_BLUE: &'static str = "text-blue-600";
pub static TEXT_GRAY: &'static str = "text-gray-400";
pub static TEXT_DARK: &'static str = "text-slate-200";
pub static TEXT_LIGHT: &'static str = "text-slate-400";
pub static BG_PRIME_DARK: &'static str = "bg-slate-800";
pub static BG_PRIME_MD_DARK: &'static str = "bg-slate-700";
pub static BG_SEC_DARK: &'static str = "bg-gray-800";
pub static BG_SEC_MD_DARK: &'static str = "bg-gray-700";

pub static BTN: &'static str = "font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2";
pub static BTN_PRIMARY: &'static str = "bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-blue-800";
pub static BTN_SECONDARY: &'static str = "bg-gray-500 text-slate-300 hover:bg-gray-700";
pub static BTN_WARN: &'static str = "bg-red-500 text-slate-300 hover:bg-red-700";
pub static BTN_PRIMARY_OUTLINE: &'static str =
    "border border-blue-500 hover:bg-blue-700 text-blue-500 hover:text-slate-300";

pub fn hover(class: &str) -> String {
    format!("hover:{}", class)
}

// btn = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300  dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
