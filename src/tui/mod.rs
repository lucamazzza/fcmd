use core::{fmt, time};
use std::{char, fmt::format};

use crate::util;

type Color = i32;
type Attr = i32;
type FillReturn = isize;
type BorderCharacter = char;

pub enum BorderShape {
    BorderUndefined,
    BorderNone,
    BorderRounded,
    BorderSharp,
    BorderBold,
    BorderBlock,
    BorderThinBlock,
    BorderDouble,
    BorderHorizontal,
    BorderVertical,
    BorderTop,
    BorderBottom,
    BorderLeft,
    BorderRight,
}

pub enum EventType {
    Rune,
    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlH,
    Tab,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,
    Esc,
    CtrlSpace,
    CtrlDel,
    CtrlBackslash,
    CtrlCaret,
    CtrlSlash,
    CtrlRightbracket,
    ShiftTab,
    Backspace,
    Del,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    Ins,
    ShiftUp,
    ShiftDown,
    ShiftLeft,
    ShiftRight,
    ShiftDelete,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    AltBackspace,
    AltUp,
    AltDown,
    AltLeft,
    AltRight,
    AltShiftUp,
    AltShiftDown,
    AltShiftLeft,
    AltShiftRight,
    Alt,
    CtrlAlt,
    Invalid,
    Fatal,
    Mouse,
    DoubleClick,
    LeftClick,
    RightClick,
    ScrollUp,
    ScrollDown,
    ShiftLeftClick,
    ShiftRightClick,
    ShiftScrollUp,
    ShiftScrollDown,
}

pub enum Events {
    Resize,
    Change,
    BackendEOF,
    Start,
    Load,
    Focus,
    One,
    Zero,
    Result,
    Jump,
    JumpCancel,
    ClickHeader,
}

pub struct ColorPair {
    fg:   Color,
    bg:   Color,
    attr: Attr,
} 

pub struct ColorAttr {
    color: Color,
    attr:  Attr,
} 

pub struct ColorTheme {
    colored:        bool,
    input:          ColorAttr,
    disabled:       ColorAttr,
    fg:             ColorAttr,
    bg:             ColorAttr,
    selected_fg:    ColorAttr,
    selected_bg:    ColorAttr,
    selected_match: ColorAttr,
    preview_fg:     ColorAttr,
    preview_bg:     ColorAttr,
    dark_bg:        ColorAttr,
    gutter:         ColorAttr,
    prompt:         ColorAttr,
    matches:        ColorAttr,
    current:        ColorAttr,
    current_match:  ColorAttr,
    spinner:        ColorAttr,
    info:           ColorAttr,
    cursor:         ColorAttr,
    marker:         ColorAttr,
    header:         ColorAttr,
    separator:      ColorAttr,
    scrollbar:      ColorAttr,
    border:         ColorAttr,
    preview_border: ColorAttr,
    preview_scroll: ColorAttr,
    border_label:   ColorAttr,
    preview_label:  ColorAttr,
}

pub struct Event {
    typ:         EventType,
    character:   char,
    mouse_event: Option<MouseEvent>,
}

pub struct MouseEvent {
    y:      isize,
    x:      isize,
    s:      isize,
    left:   bool,
    down:   bool,
    double: bool,
    module: bool,
}

pub struct BorderStyle {
    shape:        BorderShape,
    top:          char,
    bottom:       char,
    left:         char,
    right:        char,
    top_left:     char,
    top_right:    char,
    bottom_left:  char,
    bottom_right: char,
}

pub struct TermSize {
    lines:     isize,
    columns:   isize,
    px_width:  isize,
    pw_height: isize,
}

pub struct FullscreenRenderer {
    theme:       *mut ColorTheme,
    mouse:       bool,
    force_black: bool,
    pv_downtime: time::Duration,
    clicks:      [[isize;2]],
}

fn make_border_style(shape: BorderShape, utf: bool) -> BorderStyle {
    if !utf {
        return BorderStyle {
            shape,
            top:          '-',
            bottom:       '-',
            left:         '|',
            right:        '|',
            top_left:     '+',
            top_right:    '+',
            bottom_left:  '+',
            bottom_right: '+',
        }
    }
    match shape {
        BorderShape::BorderSharp => return BorderStyle {
            shape,
            top:          '─',
            bottom:       '─',
            left:         '│',
            right:        '│',
            top_left:     '┌',
            top_right:    '┐',
            bottom_left:  '└',
            bottom_right: '┘',  
        },
        BorderShape::BorderBold => return BorderStyle {
            shape,
            top:          '━',
            bottom:       '━',
            left:         '┃',
            right:        '┃',
            top_left:     '┏',
            top_right:    '┓',
            bottom_left:  '┗',
            bottom_right: '┛',  
        },
        BorderShape::BorderBlock => return BorderStyle {
            shape,
            top:          '▀',
            bottom:       '▄',
            left:         '▌',
            right:        '▐',
            top_left:     '▛',
            top_right:    '▜',
            bottom_left:  '▙',
            bottom_right: '▟',  
        },
        BorderShape::BorderThinBlock => return BorderStyle {
            shape,
            top:          '▔',
            bottom:       '▁',
            left:         '▏',
            right:        '▕',
            top_left:     '🭽',
            top_right:    '🭾',
            bottom_left:  '🭼',
            bottom_right: '🭿',  
        },
        BorderShape::BorderDouble => return BorderStyle {
            shape,              
            top:          '═',
            bottom:       '═',
            left:         '║',
            right:        '║',
            top_left:     '╔',
            top_right:    '╗',
            bottom_left:  '╚',
            bottom_right: '╝',  
        },
        _ => return BorderStyle {
            shape,
            top:          '─',
            bottom:       '─',
            left:         '│',
            right:        '│',
            top_left:     '╭',
            top_right:    '╮',
            bottom_left:  '╰',
            bottom_right: '╯',  
        },
    }
}

fn as_event(t: EventType) -> Event {
    Event {
        typ: t,
        character: char::from_u32(0).expect("NaN"),
        mouse_event: None,
    }
}

fn as_string(t: EventType) -> String {
    t.into()
}

fn comparable(e: Event) -> Event {
    Event {
        typ: e.typ,
        character: e.character,
        mouse_event: None,
    }
}

fn keyname(e: Event) -> String {
    match e.typ {
        EventType::Rune             => return format!("{}", e.character),
        EventType::Alt              => return format!("alt-{}", e.character),
        EventType::CtrlAlt          => return format!("ctrl-alt-{}", e.character),
        EventType::CtrlBackslash    => return format!("ctrl-\\"),
        EventType::CtrlRightbracket => return format!("ctrl-]"),
        EventType::CtrlCaret        => return format!("ctrl-^"),
        EventType::CtrlSlash        => return format!("ctrl-/"),
        _ => util::to_kebab_case(e.typ.into()),
    }
}
