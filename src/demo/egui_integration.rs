use glfw::{Key, Window, WindowEvent};

/// Translates a [glfw::Modifiers] instance to an [egui::Modifers] struct.
fn to_egui_modifiers(modifiers: &glfw::Modifiers) -> egui::Modifiers {
    egui::Modifiers {
        alt: modifiers.contains(glfw::Modifiers::Alt),
        ctrl: modifiers.contains(glfw::Modifiers::Control),
        shift: modifiers.contains(glfw::Modifiers::Shift),
        mac_cmd: modifiers.contains(glfw::Modifiers::Super),
        command: modifiers.contains(glfw::Modifiers::Super),
    }
}

/// Returns true when the [glfw::Action] indicates a pressed / repeated state.
fn is_pressed(action: &glfw::Action) -> bool {
    match action {
        glfw::Action::Press => true,
        glfw::Action::Repeat => true,
        glfw::Action::Release => false,
    }
}

/// Returns the [egui::Key] that matches the provided [glfw::Key].
fn to_egui_key(key: &Key) -> Option<egui::Key> {
    macro_rules! match_keys {
        ( $( $glfw_key: ident -> $egui_key: ident ),* ) => {
            match key {
                $(
                    Key::$glfw_key => { Some(egui::Key::$egui_key) },
                )*
                // No explicit rule, so try to create the mapping based on the key name.
                _ => key.get_name().and_then(|key| egui::Key::from_name(&key)),
            }
        }
    }
    match_keys!(
        // Commands
        Down -> ArrowDown,
        Left -> ArrowLeft,
        Right -> ArrowRight,
        Up -> ArrowUp,
        Escape -> Escape,
        Tab -> Tab,
        Backspace -> Backspace,
        Enter -> Enter,
        Insert -> Insert,
        Delete -> Delete,
        Home -> Home,
        End -> End,
        PageUp -> PageUp,
        PageDown -> PageDown,

        // Punctuation
        Space -> Space,
        Comma -> Comma,
        Minus -> Minus,
        Period -> Period,
        Semicolon -> Semicolon,
        LeftBracket -> OpenBracket,
        RightBracket -> CloseBracket,
        Backslash -> Backslash,
        Slash -> Slash,

        // Digits
        Num0 -> Num0,
        Num1 -> Num1,
        Num2 -> Num2,
        Num3 -> Num3,
        Num4 -> Num4,
        Num5 -> Num5,
        Num6 -> Num6,
        Num7 -> Num7,
        Num8 -> Num8,
        Num9 -> Num9,

        // Letters
        A -> A,
        B -> B,
        C -> C,
        D -> D,
        E -> E,
        F -> F,
        G -> G,
        H -> H,
        I -> I,
        J -> J,
        K -> K,
        L -> L,
        M -> M,
        N -> N,
        O -> O,
        P -> P,
        Q -> Q,
        R -> R,
        S -> S,
        T -> T,
        U -> U,
        V -> V,
        W -> W,
        X -> X,
        Y -> Y,
        Z -> Z,

        // Function Keys
        F1 -> F1,
        F2 -> F2,
        F3 -> F3,
        F4 -> F4,
        F5 -> F5,
        F6 -> F6,
        F7 -> F7,
        F8 -> F8,
        F9 -> F9,
        F10 -> F10,
        F11 -> F11,
        F12 -> F12,
        F13 -> F13,
        F14 -> F14,
        F15 -> F15,
        F16 -> F16,
        F17 -> F17,
        F18 -> F18,
        F19 -> F19,
        F20 -> F20,
        F21 -> F21,
        F22 -> F22,
        F23 -> F23,
        F24 -> F24,
        F25 -> F25
    )
}

/// Translates a GLFW WindowEvent into an egui Event which can be added to
/// a [egui::RawInput] instance.
///
/// # Returns
///
/// * `Some(event)` - when a compatible [egui::Event] can be constructed.
/// * `None` - if no compatible [egui::Event] can be constructed based on the
///   [glfw::WindowEvent].
pub fn glfw_event_to_egui_event(
    window: &Window,
    event: &WindowEvent,
) -> Option<egui::Event> {
    let egui_event = match &event {
        WindowEvent::CursorPos(x, y) => {
            Some(egui::Event::PointerMoved(egui::pos2(*x as f32, *y as f32)))
        }
        WindowEvent::MouseButton(button, action, modifiers) => {
            let (x, y) = window.get_cursor_pos();
            Some(egui::Event::PointerButton {
                pos: egui::pos2(x as f32, y as f32),
                button: match button {
                    glfw::MouseButtonLeft => egui::PointerButton::Primary,
                    glfw::MouseButtonRight => egui::PointerButton::Secondary,
                    glfw::MouseButtonMiddle => egui::PointerButton::Middle,
                    _ => egui::PointerButton::Primary,
                },
                pressed: is_pressed(action),
                modifiers: to_egui_modifiers(modifiers),
            })
        }
        WindowEvent::Key(key, _, action, modifiers) => {
            let egui_key = to_egui_key(key);
            log::info!("original key {:?} mapped to {:?}", key, egui_key);
            egui_key.map(|key| egui::Event::Key {
                key,
                physical_key: None,
                pressed: is_pressed(action),
                repeat: *action == glfw::Action::Repeat,
                modifiers: to_egui_modifiers(modifiers),
            })
        }
        WindowEvent::Char(char) => Some(egui::Event::Text(char.to_string())),
        WindowEvent::CursorEnter(false) => Some(egui::Event::PointerGone),
        _ => None,
    };
    log::info!("GLFW EVENT {event:#?}\nEGUI EVENT {egui_event:#?}");
    egui_event
}
