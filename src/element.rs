pub struct Element {
    elm_tag: String,
    children: Vec<Element>,
    attrs: Vec<(String, String)>,
    inner: Option<String>
}

impl Element {
    pub fn to_string(&self) -> String {
        let open_tag = if self.attrs.len() > 0 {
            let x = self.attrs.iter().map(|(a, v)| format!("{}={}", a, v)).fold(String::from(""), |acc, val| format!("{} {}", acc, val));
            format!("<{} {}>", self.elm_tag, x)
        } else {
            format!("<{}>", self.elm_tag)
        };
        match &self.inner {
            Some(s) => format!("{}{}</{}>", open_tag, s, self.elm_tag),
            None => {
                let c: Vec<String> = self.children.iter().map(|c| c.to_string()).collect();
                format!("{}{}</{}>", open_tag, c.join(""), self.elm_tag)
            }
        }
    }

    fn tag(tag: String) -> Element {
        Element {
            elm_tag: tag,
            children: vec![],
            attrs: vec![],
            inner: None
        }
    }

    fn text(tag: String, text: String) -> Element {
        Element {
            elm_tag: tag,
            children: vec![],
            attrs: vec![],
            inner: Some(text),
        }
    }

    pub fn attr(mut self, attr: String, value: String) -> Element {
        if value.ends_with('\'') && value.starts_with('\'') {
            self.attrs.push((attr, value));
        } else {
            self.attrs.push((attr, ["\"".into(), value, "\"".into()].join("")));
        }
        self
    }

    pub fn inner(mut self, inner: String) -> Element {
        self.inner = Some(inner);
        self
    }

    // Useful attrs
    pub fn class(mut self, class: String) -> Element {
        self.attr("class".into(), class)
    }

    pub fn id(mut self, id: String) -> Element {
        self.attr("id".into(), id)
    }
}

macro_rules! elem  {
    ($($n:ident),*) => {
        $(
        pub fn $n(children: Vec<Element>) -> Element {
            Element {
                elm_tag: String::from(stringify!($n)),
                children: children,
                attrs: vec![],
                inner: None
            }
        }
        )*
    };
}

// a's
elem!(abbr, address, article, aside, audio);

// b's
elem!(b, base, bdi, blockquote, body, button);

// c's
elem!(canvas, caption, cite, code, colgroup);

// d's
elem!(data, datalist, dd, del, details, dfn, dialog, div, dl, dt);

// e's
elem!(em, embed);

// f's
elem!(fieldset, figcaption, figure, footer, form);

// h's
elem!(h1, h2, h3, h4, h5, h6, head, header, hgroup, html);

// i's
elem!(i, ins);

// k's
elem!(kbd);

// l's
elem!(label, legend, li, link);

// m's
elem!(main, mark, menu, meter);

// n's
elem!(nav, noscript);

// o's
elem!(object, ol, optgroup, option, output);

// p's
elem!(p, param, picture, pre, progress);

// q's
elem!(q);

// r's
elem!(rp, rt, ruby);

// s's
elem!(s, samp, script, search, section, select, small, source, span, strong, style, sub, summary, sup, svg);

// t's
elem!(table, tbody, td, template, tfoot, th, thread, time, tr, track);

// u's
elem!(u, ul);

// v's
elem!(var, video);

pub fn a(url: String, children: Vec<Element>) -> Element {
    Element {
        elm_tag: String::from("a"),
        children: children,
        attrs: vec![(String::from("href"), url)],
        inner: None
    }
}

pub fn area(shape: String, coords: String) -> Element {
    Element {
        elm_tag: String::from("area"),
        children: vec![],
        attrs: vec![(String::from("shape"), shape), (String::from("coords"), coords)],
        inner: None
    }
}

pub fn bdo(dir: String, children: Vec<Element>) -> Element {
    Element {
        elm_tag: String::from("bdo"),
        children: children,
        attrs: vec![(String::from("dir"), dir)],
        inner: None
    }
}

pub fn br() -> Element {
    Element::tag(String::from("br"))
}

pub fn col() -> Element {
    Element::tag(String::from("col"))
}

pub fn hr() -> Element {
    Element::tag(String::from("hr"))
}

pub fn iframe(src: String) -> Element {
    Element {
        elm_tag: String::from("iframe"),
        children: vec![],
        attrs: vec![(String::from("src"), src)],
        inner: None
    }
}

pub fn input(input_type: String) -> Element {
    Element {
        elm_tag: String::from("input"),
        children: vec![],
        attrs: vec![(String::from("type"), input_type)],
        inner: None
    }
}

pub fn map(name: String, children: Vec<Element>) -> Element {
    Element {
        elm_tag: String::from("map"),
        children: children,
        attrs: vec![(String::from("name"), name)],
        inner: None
    }
}

pub fn meta(name: String, content: String) -> Element {
    Element {
        elm_tag: String::from("meta"),
        children: vec![],
        attrs: vec![(name, content)],
        inner: None
    }
}

pub fn textarea(text: String) -> Element {
    Element {
        elm_tag: String::from("textarea"),
        children: vec![],
        attrs: vec![],
        inner: Some(text)
    }
}

pub fn title(title: String) -> Element {
    Element::text(String::from("title"), title)
}

pub fn wbr() -> Element {
    Element::tag(String::from("wbr"))
}

// Special cases
pub fn img(src: String) -> Element {
    Element {
        elm_tag: String::from("img"),
        children: vec![],
        attrs: vec![(String::from("src"), src)],
        inner: None,
    }
}