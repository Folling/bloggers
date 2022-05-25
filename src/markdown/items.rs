use bitflags::bitflags;

bitflags! {
    pub struct TextStyleFlags: u8 {
        const ITALIC        = 0b0000_0001;
        const BOLD          = 0b0000_0010;
        const STRIKETHROUGH = 0b0000_0100;
        const UNDERLINE     = 0b0000_1000;
        const INLINE_CODE   = 0b0001_0000;
    }
}

#[derive(Debug)]
pub struct StyledText {
    pub content: String,
    pub flags: TextStyleFlags,
}

impl StyledText {
    pub fn generate(&self) -> String {
        let mut delim = String::with_capacity(16);
        let mut closing_delim = String::with_capacity(16);

        if self.flags.contains(TextStyleFlags::ITALIC) {
            delim += "<i>";
            closing_delim += "</i>";
        }

        if self.flags.contains(TextStyleFlags::BOLD) {
            delim += "<b>";
            closing_delim += "</b>";
        }

        if self.flags.contains(TextStyleFlags::STRIKETHROUGH) {
            delim += "<s>";
            closing_delim += "</s>";
        }

        if self.flags.contains(TextStyleFlags::UNDERLINE) {
            delim += "<u>";
            closing_delim += "</u>";
        }

        if self.flags.contains(TextStyleFlags::INLINE_CODE) {
            delim += "<code class='plaintext'>";
            closing_delim += "</code>";
        }

        format!(
            "{delim}{content}{closing_delim}",
            delim = delim,
            content = self.content,
            closing_delim = closing_delim
        )
    }
}

#[derive(Debug)]
pub enum TextItem {
    Plain { content: StyledText },
    HyperLink { display: String, link: String },
    MediaLink { display: String, link: String },
}

#[derive(Debug)]
pub enum ListType {
    Ordered,
    Unordered,
}

#[derive(Debug)]
pub struct Newline;

impl Newline {
    #[allow(clippy::unused_self)]
    pub fn generate(&self) -> String {
        "<br>".to_owned()
    }
}

#[derive(Debug)]
pub struct HorizontalLine;

impl HorizontalLine {
    #[allow(clippy::unused_self)]
    pub fn generate(&self) -> String {
        "<hr>".to_owned()
    }
}

#[derive(Debug)]
pub struct Header {
    pub level: u8,
    pub text: Text,
}

impl Header {
    pub fn generate(&self) -> String {
        format!("<h{level}>{content}</h{level}>", level = self.level, content = self.text.generate())
    }
}

#[derive(Debug)]
pub struct Text {
    pub items: Vec<TextItem>,
}

impl Text {
    pub fn generate(&self) -> String {
        self.items
            .iter()
            .map(|v| match v {
                TextItem::Plain { content } => content.generate(),
                TextItem::HyperLink { display, link } => format!("<a href='/media/{}>{}</a>'", link, display),
                TextItem::MediaLink { display, link } => format!("<img src='/media/{}>{}</a>'", link, display),
            })
            .collect::<String>()
    }
}

#[derive(Debug)]
pub enum ListItem {
    Singular(Text),
    SubList(List),
}

#[derive(Debug)]
pub struct List {
    pub r#type: ListType,
    pub items: Vec<ListItem>,
}

impl List {
    pub fn generate(&self) -> String {
        let ident = match self.r#type {
            ListType::Ordered => "ol",
            ListType::Unordered => "ul",
        };

        format!(
            "<{ident}>{content}</{ident}>",
            ident = ident,
            content = self
                .items
                .iter()
                .map(|v| match v {
                    ListItem::Singular(v) => format!("<li>{}</li>", v.generate()),
                    ListItem::SubList(v) => v.generate(),
                })
                .collect::<String>()
        )
    }
}

#[derive(Debug)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
}

impl CodeBlock {
    pub fn generate(&self) -> String {
        format!(
            "<pre><code class={}>{}</code></pre>",
            self.language.as_ref().unwrap_or(&"plaintext".to_owned()),
            self.content
        )
    }
}

#[derive(Debug)]
pub enum TopLevelItem {
    Newline { newline: Newline },
    HorizontalLine { line: HorizontalLine },
    Header { header: Header },
    Text { text: Text },
    CodeBlock { code_block: CodeBlock },
    List { list: List },
}

impl TopLevelItem {
    pub fn generate(&self) -> String {
        #[allow(clippy::pattern_type_mismatch)]
        match self {
            TopLevelItem::Newline { newline } => newline.generate(),
            TopLevelItem::HorizontalLine { line: horizontal_line } => horizontal_line.generate(),
            TopLevelItem::Header { header } => header.generate(),
            TopLevelItem::Text { text } => text.generate(),
            TopLevelItem::CodeBlock { code_block } => code_block.generate(),
            TopLevelItem::List { list } => list.generate(),
        }
    }
}
