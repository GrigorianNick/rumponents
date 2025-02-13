pub struct Element {
    elm_tag: String,
    children: Vec<Element>,
}

impl Element {
    pub fn to_string(&self) -> String {
        if self.children.len() > 0 {
            format!("<{}>{}</{}>", self.elm_tag, self.children[0].to_string(), self.elm_tag)
        } else {
            format!("<{}>{}</{}>", self.elm_tag, "No children!", self.elm_tag)
        }
    }
}

pub fn div(children: Vec<Element>) -> Element {
    Element {
        elm_tag: String::from("div"),
        children: vec![]
    }
}

pub fn p(children: Vec<Element>) -> Element {
    Element {
        elm_tag: String::from("p"),
        children: vec![]
    }
}