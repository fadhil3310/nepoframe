use std::fmt::Display;

use slotmap::{DefaultKey, SlotMap};

pub mod event;
use self::event::{SceneGraphMessage, WidgetEvent};

// Scene Graph ===============================================

// NodeTree is the scene graph

// #[derive(Debug)]
pub struct SceneGraph {
    elements: SlotMap<DefaultKey, Node>,
    root: DefaultKey,
    message_queue: Vec<SceneGraphMessage>
}

impl SceneGraph {
    pub fn new(root_node: Node) -> Self {
        let mut map = SlotMap::new();

        let root_node_key = map.insert(root_node);

        let graph = SceneGraph {
            elements: map,
            root: root_node_key,
            message_queue: Vec::new()
        };
        graph
    }

    pub fn post_message(&mut self, msg: SceneGraphMessage) {
        self.message_queue.push(msg);
    }

    fn process_messages(&mut self) {
        
    }
}

// Node/Widget ===============================================

// #[derive(Debug)]
pub struct Node {
    parent: Option<DefaultKey>,
    children: Vec<DefaultKey>,
    widget: Box<dyn Widget>,

    pub visible: Visibility,
    pub position: (i32, i32),
    pub size: (i32, i32),
    pub render_transform: Option<RenderTransform>,
    pub opacity: f32,

    pub focusable: bool
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let widget_type = self.widget.as_ref().get_type();

        format!("Element; Widget type: {}", widget_type);

        std::fmt::Result::Ok(())
    }
}

impl Node {
    pub fn new(widget: Box<dyn Widget>) -> Node {
        let node = Node {
            parent: None,
            children: Vec::new(),
            widget: widget,

            visible: Visibility::Visible,
            position: (0, 0),
            size: (0, 0),
            render_transform: None,
            opacity: 1.0,

            focusable: false
        };

        node
    }
}

#[derive(Debug)]
pub enum Visibility {
    Visible,
    Collapsed,
    Hidden
}

#[derive(Default, Debug)]
pub struct RenderTransform {
    pub enabled: bool,
    pub translate: (i32, i32),
    pub scale: (i32, i32),
    pub rotate: (i32, i32, i32),
}

pub trait Widget {
    fn render(&self, wctx: WidgetContext);
    fn handle_event(&mut self, wctx: WidgetContext, event: WidgetEvent);

    fn on_mount(&mut self, wctx: WidgetContext);
    fn on_unmount(&mut self, wctx: WidgetContext);

    fn get_type(&self) -> &'static str;
}

pub struct WidgetContext {
    pub node_key: DefaultKey,
    pub node: &'static mut Node,
    pub scene_graph: &'static mut SceneGraph
}

// ===============================================