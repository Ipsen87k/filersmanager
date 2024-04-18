use iced::{widget::text, Element, Font};


fn icon<'a,T>(codepoint:char)->Element<'a,T>{
    const ICON_FONT:Font= Font::with_name("iced-image");
    text(codepoint).font(ICON_FONT).into()
}

pub fn output_icon<'a,T>()->Element<'a,T>{
    icon('\u{0e801}')
}

pub fn open_folder_icon<'a,T>()->Element<'a,T>{
    icon('\u{0e802}')
}

pub fn search_icon<'a,T>()->Element<'a,T>{
    icon('\u{0e803}')
}
