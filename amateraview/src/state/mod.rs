use crate::Message;
use amateraview_common::plugin::PluginHandle;
use amateraview_common::ui::button::{Button, ButtonState};
use amateraview_common::ui::{TreeView, WidgetHandle, WidgetState};
use iced::Element;
use iced::widget::{button, column, pane_grid, row, text};
use std::collections::HashMap;
use tracing::info;

pub struct State {
    pub panes: pane_grid::State<PluginHandle>,
    pub plugins: HashMap<PluginHandle, Plugin>,
}

impl Default for State {
    fn default() -> Self {
        let handle = PluginHandle::new();
        let text = WidgetState::Text("World".into());
        let button = WidgetState::Button(Button {
            description: "Some nice button!".into(),
            state: ButtonState::Released,
        });
        let button_function: Option<Box<dyn FnMut()>> = Some(Box::new(move || {
            info!("Hello");
        }));
        let text_handle = WidgetHandle::new();
        let button_handle = WidgetHandle::new();

        let plugin = Plugin {
            handle,
            title: "Hello".into(),
            tree_view: TreeView::Column(
                [TreeView::Leaf(text_handle), TreeView::Leaf(button_handle)].into(),
            ),
            widgets: HashMap::from([
                (text_handle, (text, None)),
                (button_handle, (button, button_function)),
            ]),
        };
        let (panes, _) = pane_grid::State::new(handle);
        let mut plugins = HashMap::new();
        plugins.insert(handle, plugin);
        Self { panes, plugins }
    }
}

pub struct Plugin {
    handle: PluginHandle,
    pub title: String,
    pub tree_view: TreeView,
    pub widgets: HashMap<WidgetHandle, (WidgetState, Option<Box<dyn FnMut()>>)>,
}

impl Plugin {
    pub fn view(&self) -> Element<Message> {
        traverse(&self.tree_view, &self.widgets, self.handle)
    }
    pub fn triggered(&mut self, handle: WidgetHandle) {
        if let Some((w, Some(f))) = self.widgets.get_mut(&handle) {
            f();
        }
    }
}

fn traverse<'a>(
    widget_tree: &'a TreeView,
    widgets: &'a HashMap<WidgetHandle, (WidgetState, Option<Box<dyn FnMut()>>)>,
    plugin_handle: PluginHandle,
) -> Element<'a, Message> {
    match widget_tree {
        TreeView::Leaf(widget_handle) => {
            let (widget, action) = &widgets[widget_handle];
            match widget {
                WidgetState::Button(b) => button(b.description.as_str())
                    .on_press(Message::ButtonPressed(plugin_handle, *widget_handle))
                    .style(button::primary)
                    .into(),
                WidgetState::Text(t) => text(t).into(),
            }
        }
        TreeView::Row(content) => {
            row(content.iter().map(|x| traverse(x, widgets, plugin_handle))).into()
        }
        TreeView::Column(content) => {
            column(content.iter().map(|x| traverse(x, widgets, plugin_handle))).into()
        }
    }
}
