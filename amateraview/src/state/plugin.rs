use crate::Message;
use amateraview_common::plugin::PluginHandle;
use amateraview_common::ui::button::{Button, ButtonState};
use amateraview_common::ui::{TreeView, WidgetHandle, WidgetState};
use iced::Element;
use iced::widget::{button, column, row, text};
use std::collections::HashMap;
use tracing::info;

pub struct Plugin {
    handle: PluginHandle,
    pub title: String,
    pub tree_view: TreeView,
    pub widgets: HashMap<WidgetHandle, WidgetState>,
    pub actions: HashMap<WidgetHandle, Box<dyn FnMut()>>,
}

impl Plugin {
    pub fn view(&self) -> Element<Message> {
        traverse(&self.tree_view, &self.widgets, self.handle)
    }

    pub fn triggered(&mut self, handle: WidgetHandle) {
        if let Some(a) = self.actions.get_mut(&handle) {
            a();
        }
    }

    pub fn create_demo() -> (PluginHandle, Plugin) {
        let handle = PluginHandle::new();
        let text_handle = WidgetHandle::new();
        let button_handle = WidgetHandle::new();

        let text = WidgetState::Text("World".into());
        let button = WidgetState::Button(Button {
            description: "Some nice button!".into(),
            state: ButtonState::Released,
        });

        let plugin = Plugin {
            handle,
            title: format!("Hello  {handle}"),
            tree_view: TreeView::Column(
                [TreeView::Leaf(text_handle), TreeView::Leaf(button_handle)].into(),
            ),
            widgets: HashMap::from([(text_handle, text), (button_handle, button)]),
            actions: HashMap::from([(
                button_handle,
                Box::new(move || {
                    info!("Hello from {button_handle}.");
                }) as Box<dyn FnMut()>,
            )]),
        };
        (handle, plugin)
    }
}

fn traverse<'a>(
    widget_tree: &'a TreeView,
    widgets: &'a HashMap<WidgetHandle, WidgetState>,
    plugin_handle: PluginHandle,
) -> Element<'a, Message> {
    match widget_tree {
        TreeView::Leaf(widget_handle) => {
            let widget = &widgets[widget_handle];
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
