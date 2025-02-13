pub struct Element {
    elm_tag: String,
    children: Vec<Element>,
    attrs: Vec<(String, String)>,
    inner: Option<String>
}

impl Element {
    pub fn to_string(&self) -> String {
        let open_tag = if self.attrs.len() > 0 {
            let x = self.attrs.iter().map(|(a, v)| format!("{}=\"{}\"", a, v)).fold(String::from(""), |acc, val| format!("{} {}", acc, val));
            format!("<{} {}>", self.elm_tag, x)
        } else {
            format!("<{}>", self.elm_tag)
        };
        match &self.inner {
            Some(s) => format!("{}{}</{}>", open_tag, "No children!", self.elm_tag),
            None => {
                let c: Vec<String> = self.children.iter().map(|c| c.to_string()).collect();
                format!("{}{}</{}>", open_tag, c.join(""), self.elm_tag)
            }
        }
    }

    pub fn attr(mut self, attr: String, value: String) -> Element {
        self.attrs.push((attr, value));
        self
    }

    pub fn inner(mut self, inner: String) -> Element {
        self.inner = Some(inner);
        self
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
elem!(a, abbr, address, area, article, aside, audio);

// b's
elem!(b, base, bdi, bdo, blockquote, body, br, button);

// c's
elem!(canvas, caption, cite, code, col, colgroup);

// d's
elem!(data, datalist, dd, del, details, dfn, dialog, div, dl, dt);

// e's
elem!(em, embed);

// f's
elem!(fieldset, figcaption, figure, footer, form);

// h's
elem!(h1, h2, h3, h4, h5, h6, head, header, hgroup, hr, html);

// i's
elem!(i, iframe, img, input, ins);

// k's
elem!(kbd);

// l's
elem!(label, legend, li, link);

// m's
elem!(main, map, mark, menu, meta, meter);

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
elem!(table, tbody, td, template, textarea, tfoot, th, thread, time, title, tr, track);

// u's
elem!(u, ul);

// v's
elem!(var, video);

// w's
elem!(wbr);