use crate::{
    edit_mode::{keybindings::edit_bind, Keybindings},
    ReedlineEvent,
};

use {
    crate::EditCommand as EC,
    crossterm::event::{KeyCode as KC, KeyModifiers as KM},
};

pub fn default_vi_normal_keybindings() -> Keybindings {
    let mut kb = Keybindings::new();

    kb.add_binding(KM::CONTROL, KC::Char('c'), ReedlineEvent::CtrlC);

    kb
}

pub fn default_vi_insert_keybindings() -> Keybindings {
    let mut kb = Keybindings::new();

    kb.add_binding(KM::NONE, KC::Up, ReedlineEvent::Up);
    kb.add_binding(KM::NONE, KC::Down, ReedlineEvent::Down);
    kb.add_binding(KM::NONE, KC::Left, edit_bind(EC::MoveLeft));
    kb.add_binding(KM::NONE, KC::Right, ReedlineEvent::Right);
    kb.add_binding(KM::NONE, KC::Backspace, edit_bind(EC::Backspace));
    kb.add_binding(KM::NONE, KC::Delete, edit_bind(EC::Delete));
    kb.add_binding(KM::NONE, KC::End, edit_bind(EC::MoveToLineEnd));
    kb.add_binding(KM::NONE, KC::Home, edit_bind(EC::MoveToLineStart));
    kb.add_binding(KM::CONTROL, KC::Char('c'), ReedlineEvent::CtrlC);
    kb.add_binding(KM::NONE, KC::Tab, ReedlineEvent::ContextMenu);

    kb
}
