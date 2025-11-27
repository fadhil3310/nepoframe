use slotmap::DefaultKey;


// Scene Graph Message =================

pub enum SceneGraphMessage {
    WidgetRequestRedraw(DefaultKey),
    WidgetRequestAnimation(RequestAnimationMessage)
}

pub struct RequestAnimationMessage {
    node_key: DefaultKey,
    duration: i32,
    frame_rate: i32
}


// Widget Event ========================

pub enum WidgetEvent {
    // ----- General -----
    Click(MouseEvent),

    // ----- Mouse -----
    MouseEnter(MouseEvent),
    MouseMove(MouseEvent),
    MouseLeave(MouseEvent),
    MouseDown(MouseButtonEvent),
    MouseUp(MouseButtonEvent),
    MouseScroll(MouseEvent),

    // ----- Animation -----
    AnimationTick(AnimationTickEvent)
}


// Mouse event ========

pub struct MouseEvent {
    x: i32,
    y: i32,
    scroll_delta: i32
}

pub struct MouseButtonEvent {
    mouse: MouseEvent,
    button: MouseButton
}

pub enum MouseButton {
    Left,
    Right,
    Middle
}

pub struct  AnimationTickEvent {
    duration: i32,
    frame_rate: i32,
    progress: f32
}