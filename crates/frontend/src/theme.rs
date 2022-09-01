use yew::{classes, Classes};

pub static TEXT_DARK: &'static str = "text-slate-200";
pub static TEXT_LIGHT: &'static str = "text-slate-400";
pub static BG_DARK: &'static str = "bg-slate-800";
pub static BG_MD_DARK: &'static str = "bg-slate-700";

pub static BTN: &'static str = "m-2 px-2 py-1 rounded-md transition";
pub static BTN_PRIMARY: &'static str = "bg-blue-500 text-slate-300 hover:bg-blue-700";
pub static BTN_SECONDARY: &'static str = "bg-gray-500 text-slate-300 hover:bg-gray-700";
pub static BTN_WARN: &'static str = "bg-red-500 text-slate-300 hover:bg-red-700";

pub static BTN_PRIMARY_OUTLINE: &'static str =
    "border border-blue-500 hover:bg-blue-700 text-blue-500 hover:text-slate-300";

pub fn hover(class: &str) -> String {
    format!("hover:{}", class)
}
