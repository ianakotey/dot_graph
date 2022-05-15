/// each node is an index in a vector in the graph.
// pub type Node = usize;

use crate::{
    style::Style,
    utils::quote_string,
    render::{RenderOption}
};

pub struct Node {
    pub name: String,
    pub label: String,
    pub style: Style,
    pub color: Option<&'static str>,
    pub index: usize,
    pub shape: Option<String>
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node { name: self.name.clone(), label: self.label.clone(), style: self.style.clone(), color: self.color, index: self.index , shape: self.shape.clone()}
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node { name: String::from(name), label: String::new(), style: Style::None, color: None, index: 0, shape: None }
    }

    pub fn new_with_label(name_label: &str) -> Self {
        Node { name: String::from(name_label), label: String::from(name_label), style: Style::None, color: None, index: 0, shape: None }
    }

    pub fn set_label(&mut self, label: &str) -> () {
        self.label = String::from(label);
    }

    pub fn set_style(&mut self, style: Style) -> () {
        self.style = style;
    }

    pub fn set_shape(&mut self, shape: Option<&str>) -> () {
        match shape {
            Some(s) => self.shape = Some(String::from(s)),
            None => self.shape = None
        }
    }

    pub fn set_color(&mut self, color: Option<&'static str>) -> () {
        self.color = color;
    }

    pub fn node_id(&self) -> &str {
        self.name.as_str()
    }

    pub fn to_dot_string(&self, options: &[RenderOption]) -> String {
        let colorstring: String;

        let escaped: String = quote_string(self.label.clone());
        let shape: String;

        let mut text = vec![self.node_id()];

        if !options.contains(&RenderOption::NoNodeLabels) {
            text.push("[label=");
            text.push(escaped.as_str());
            text.push("]");
        }

        let style = self.style;
        if !options.contains(&RenderOption::NoNodeStyles) && style != Style::None {
            text.push("[style=\"");
            text.push(style.as_slice());
            text.push("\"]");
        }

        let color = self.color;
        if !options.contains(&RenderOption::NoNodeColors) {
            if let Some(c) = color {
                colorstring = quote_string(c.to_string());
                text.push("[color=");
                text.push(&colorstring);
                text.push("]");
            }
        }

        if let Some(s) = self.shape.clone() {
            shape = s;
            text.push("[shape=\"");
            text.push(&shape);
            text.push("\"]");
        }

        text.push(";");
        return text.into_iter().collect();
    }

}