#[derive(Debug)]
pub(crate) struct TagInfo {
    /// The name of the tag. E.g. `a`, `br` or `img`.
    name: &'static str,

    /// The name of the type in `web_sys`. This closely correlates with the
    /// "interface" in the HTML standard (only the casing is different).
    ty: &'static str,

    categories: &'static [ContentModel],

    children: &'static [Child],

    attributes: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ContentModel {
    Metadata,
    Flow,
    Sectioning,
    Heading,
    Phrasing,
    Embedded,
    Interactive,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Child {
    Transparent,
    Text,
    Model(ContentModel),
    Tag(&'static str),
}

macro_rules! def_tags {
    ($const_name:ident; [ $(
        $name:ident: $ty:ident => {
            [ $($category:ident),* ],
            [ $($child:expr),* ],
            [ $($attribute:literal),* ],
        },
    )* ]) => {
        use ContentModel::*;
        use Child::*;

        pub(crate) const $const_name: &[TagInfo] = &[ $(
            TagInfo {
                name: stringify!($name),
                ty: stringify!($ty),
                categories: &[ $( $category ),* ],
                children: &[ $( $child ),* ],
                attributes: &[ $( $attribute ),* ],
            },
        )* ];
    };
}

// This data was generated from the table of HTML elements from the official
// HTML standard (in the section "Index"):
//
//     https://html.spec.whatwg.org/#elements-3
//
// However, HTML is messy and we can't capture all rules within this single
// list. The table from the standard was simplified:
//
// - The `svg` tag, MathML elements and custom elements were ignored.
// - Only the columns "element", "categories", "children", "attributes" and
//   "Interface" were used.
// - All asterisks (denoting special rules) were removed.
// - For some tags, certain parts of the "categories" or "children" column were
//   ignored (some of them are marked with "TODO").
def_tags!(TAG_INFOS; [
    // Syntax:
    // tag: Type => { [categories...], [children...], [attributes...] }
    a: HtmlAnchorElement => {
        [Flow, Phrasing, Interactive],
        [Transparent],
        ["href", "target", "download", "ping", "rel", "hreflang", "type", "referrerpolicy"],
    },
    abbr: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    address: HtmlElement => {
        [Flow],
        [Model(Flow)],
        [],
    },
    area: HtmlAreaElement => {
        [Flow, Phrasing],
        [],
        ["alt", "coords", "shape", "href", "target", "download", "ping", "rel", "referrerpolicy"],
    },
    article: HtmlElement => {
        [Flow, Sectioning],
        [Model(Flow)],
        [],
    },
    aside: HtmlElement => {
        [Flow, Sectioning],
        [Model(Flow)],
        [],
    },
    audio: HtmlAudioElement => {
        [Flow, Phrasing, Embedded, Interactive],
        [Tag("source"), Tag("track"), Transparent],
        ["src", "crossorigin", "preload", "autoplay", "loop", "muted", "controls"],
    },
    b: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    base: HtmlBaseElement => {
        [Metadata],
        [],
        ["href", "target"],
    },
    bdi: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    bdo: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    blockquote: HtmlQuoteElement => {
        [Flow],
        [Model(Flow)],
        ["cite"],
    },
    body: HtmlBodyElement => {
        [],
        [Model(Flow)],
        ["onafterprint", "onbeforeprint", "onbeforeunload", "onhashchange", "onlanguagechange",
            "onmessage", "onmessageerror", "onoffline", "ononline", "onpagehide", "onpageshow",
            "onpopstate", "onrejectionhandled", "onstorage", "onunhandledrejection", "onunload"],
    },
    br: HtmlBRElement => {
        [Flow, Phrasing],
        [],
        [],
    },
    button: HtmlButtonElement => {
        [Flow, Phrasing, Interactive],
        [Model(Phrasing)],
        ["disabled", "form", "formaction", "formenctype", "formmethod", "formnovalidate",
            "formtarget", "name", "type", "value"],
    },
    canvas: HtmlCanvasElement => {
        [Flow, Phrasing, Embedded],
        [Transparent],
        ["width", "height"],
    },
    caption: HtmlTableCaptionElement => {
        [],
        [Model(Flow)],
        [],
    },
    cite: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    code: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    col: HtmlTableColElement => {
        [],
        [],
        ["span"],
    },
    colgroup: HtmlTableColElement => {
        [],
        [Tag("col"), Tag("template")],
        ["span"],
    },
    data: HtmlDataElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        ["value"],
    },
    datalist: HtmlDataListElement => {
        [Flow, Phrasing],
        [Model(Phrasing), Tag("option")],
        [],
    },
    dd: HtmlElement => {
        [],
        [Model(Flow)],
        [],
    },
    del: HtmlModElement => {
        [Flow, Phrasing],
        [Transparent],
        ["cite", "datetime"],
    },
    details: HtmlDetailsElement => {
        [Flow, Interactive],
        [Tag("summary"), Model(Flow)],
        ["open"],
    },
    dfn: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    dialog: HtmlDialogElement => {
        [Flow],
        [Model(Flow)],
        ["open"],
    },
    div: HtmlDivElement => {
        [Flow],
        [Model(Flow)],
        [],
    },
    dl: HtmlDListElement => {
        [Flow],
        [Tag("dt"), Tag("dd"), Tag("div")],
        [],
    },
    dt: HtmlElement => {
        [],
        [Model(Flow)],
        [],
    },
    em: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    embed: HtmlEmbedElement => {
        [Flow, Phrasing, Embedded, Interactive],
        [],
        ["src", "type", "width", "height", "any"],
    },
    fieldset: HtmlFieldSetElement => {
        [Flow],
        [Tag("legend"), Model(Flow)],
        ["disabled", "form", "name"],
    },
    figcaption: HtmlElement => {
        [],
        [Model(Flow)],
        [],
    },
    figure: HtmlElement => {
        [Flow],
        [Tag("figcaption"), Model(Flow)],
        [],
    },
    footer: HtmlElement => {
        [Flow],
        [Model(Flow)],
        [],
    },
    form: HtmlFormElement => {
        [Flow],
        [Model(Flow)],
        ["action", "autocomplete", "enctype", "method", "name", "novalidate", "target"],
    },
    h1: HtmlHeadingElement => {
        [Flow, Heading],
        [Model(Phrasing)],
        [],
    },
    h2: HtmlHeadingElement => {
        [Flow, Heading],
        [Model(Phrasing)],
        [],
    },
    h3: HtmlHeadingElement => {
        [Flow, Heading],
        [Model(Phrasing)],
        [],
    },
    h4: HtmlHeadingElement => {
        [Flow, Heading],
        [Model(Phrasing)],
        [],
    },
    h5: HtmlHeadingElement => {
        [Flow, Heading],
        [Model(Phrasing)],
        [],
    },
    h6: HtmlHeadingElement => {
        [Flow, Heading],
        [Model(Phrasing)],
        [],
    },
    head: HtmlHeadElement => {
        [],
        [Model(Metadata)],
        [],
    },
    header: HtmlElement => {
        [Flow],
        [Model(Flow)],
        [],
    },
    hgroup: HtmlElement => {
        [Flow, Heading],
        [Tag("h1"), Tag("h2"), Tag("h3"), Tag("h4"), Tag("h5"), Tag("h6")],
        [],
    },
    hr: HtmlHRElement => {
        [Flow],
        [],
        [],
    },
    html: HtmlHtmlElement => {
        [],
        [Tag("head"), Tag("body")],
        ["manifest"],
    },
    i: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    iframe: HtmlIFrameElement => {
        [Flow, Phrasing, Embedded, Interactive],
        [],
        ["src", "srcdoc", "name", "sandbox", "allow", "allowfullscreen", "allowpaymentrequest",
            "width", "height", "referrerpolicy"],
    },
    img: HtmlImageElement => {
        [Flow, Phrasing, Embedded, Interactive],
        [],
        ["alt", "src", "srcset", "crossorigin", "usemap", "ismap", "width", "height",
            "decoding", "referrerpolicy"],
    },
    input: HtmlInputElement => {
        [Flow, Phrasing, Interactive],
        [],
        ["accept", "alt", "autocomplete", "checked", "dirname", "disabled", "form", "formaction",
            "formenctype", "formmethod", "formnovalidate", "formtarget", "height", "list", "max",
            "maxlength", "min", "minlength", "multiple", "name", "pattern", "placeholder",
            "readonly", "required", "size", "src", "step", "type", "value", "width"],
    },
    ins: HtmlModElement => {
        [Flow, Phrasing],
        [Transparent],
        ["cite", "datetime"],
    },
    kbd: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    label: HtmlLabelElement => {
        [Flow, Phrasing, Interactive],
        [Model(Phrasing)],
        ["for"],
    },
    legend: HtmlLegendElement => {
        [],
        [Model(Phrasing)],
        [],
    },
    li: HtmlLIElement => {
        [],
        [Model(Flow)],
        ["value"],
    },
    link: HtmlLinkElement => {
        [Metadata, Flow, Phrasing],
        [],
        ["href", "crossorigin", "rel", "as", "media", "hreflang", "type", "sizes", "imagesrcset",
            "imagesizes", "referrerpolicy", "integrity"],
    },
    main: HtmlElement => {
        [Flow],
        [Model(Flow)],
        [],
    },
    map: HtmlMapElement => {
        [Flow, Phrasing],
        [Transparent, Tag("area")],
        ["name"],
    },
    mark: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    menu: HtmlMenuElement => {
        [Flow],
        [Tag("li")],
        [],
    },
    meta: HtmlMetaElement => {
        [Metadata, Flow, Phrasing],
        [],
        ["name", "content", "charset"],
    },
    meter: HtmlMeterElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        ["value", "min", "max", "low", "high", "optimum"],
    },
    nav: HtmlElement => {
        [Flow, Sectioning],
        [Model(Flow)],
        [],
    },
    noscript: HtmlElement => {
        [Metadata, Flow, Phrasing],
        [], // TODO
        [],
    },
    object: HtmlObjectElement => {
        [Flow, Phrasing, Embedded, Interactive],
        [Tag("param"), Transparent],
        ["data", "type", "name", "usemap", "form", "width", "height"],
    },
    ol: HtmlOListElement => {
        [Flow],
        [Tag("li")],
        ["reversed", "start", "type"],
    },
    optgroup: HtmlOptGroupElement => {
        [],
        [Tag("option")],
        ["disabled", "label"],
    },
    option: HtmlOptionElement => {
        [],
        [Text],
        ["disabled", "label", "selected", "value"],
    },
    output: HtmlOutputElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        ["for", "form", "name"],
    },
    p: HtmlParagraphElement => {
        [Flow],
        [Model(Phrasing)],
        [],
    },
    param: HtmlParamElement => {
        [],
        [],
        ["name", "value"],
    },
    picture: HtmlPictureElement => {
        [Flow, Phrasing, Embedded],
        [Tag("source"), Tag("img")],
        [],
    },
    pre: HtmlPreElement => {
        [Flow],
        [Model(Phrasing)],
        [],
    },
    progress: HtmlProgressElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        ["value", "max"],
    },
    q: HtmlQuoteElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        ["cite"],
    },
    rp: HtmlElement => {
        [],
        [Text],
        [],
    },
    rt: HtmlElement => {
        [],
        [Model(Phrasing)],
        [],
    },
    ruby: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing), Tag("rt"), Tag("rp")],
        [],
    },
    s: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    samp: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    script: HtmlScriptElement => {
        [Metadata, Flow, Phrasing],
        [], // TODO
        ["src", "type", "async", "defer", "crossorigin", "integrity", "referrerpolicy"],
    },
    section: HtmlElement => {
        [Flow, Sectioning],
        [Model(Flow)],
        [],
    },
    select: HtmlSelectElement => {
        [Flow, Phrasing, Interactive],
        [Tag("option"), Tag("optgroup")],
        ["autocomplete", "disabled", "form", "multiple", "name", "required", "size"],
    },
    slot: HtmlSlotElement => {
        [Flow, Phrasing],
        [Transparent],
        ["name"],
    },
    small: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    source: HtmlSourceElement => {
        [],
        [],
        ["src", "type", "srcset", "sizes", "media"],
    },
    span: HtmlSpanElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    strong: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    style: HtmlStyleElement => {
        [Metadata],
        [Text],
        ["media"],
    },
    sub: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    summary: HtmlElement => {
        [],
        [Model(Phrasing)],
        [],
    },
    sup: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    table: HtmlTableElement => {
        [Flow],
        [Tag("caption"), Tag("colgroup"), Tag("thead"), Tag("tbody"), Tag("tfoot"), Tag("tr")],
        [],
    },
    tbody: HtmlTableSectionElement => {
        [],
        [Tag("tr")],
        [],
    },
    td: HtmlTableCellElement => {
        [],
        [Model(Flow)],
        ["colspan", "rowspan", "headers"],
    },
    template: HtmlTemplateElement => {
        [Metadata, Flow, Phrasing],
        [],
        [],
    },
    textarea: HtmlTextAreaElement => {
        [Flow, Phrasing, Interactive],
        [Text],
        ["cols", "dirname", "disabled", "form", "maxlength", "minlength", "name", "placeholder",
            "readonly", "required", "rows", "wrap"],
    },
    tfoot: HtmlTableSectionElement => {
        [],
        [Tag("tr")],
        [],
    },
    th: HtmlTableCellElement => {
        [Interactive],
        [Model(Flow)],
        ["colspan", "rowspan", "headers", "scope", "abbr"],
    },
    thead: HtmlTableSectionElement => {
        [],
        [Tag("tr")],
        [],
    },
    time: HtmlTimeElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        ["datetime"],
    },
    title: HtmlTitleElement => {
        [Metadata],
        [Text],
        [],
    },
    tr: HtmlTableRowElement => {
        [],
        [Tag("th"), Tag("td")],
        [],
    },
    track: HtmlTrackElement => {
        [],
        [],
        ["default", "kind", "label", "src", "srclang"],
    },
    u: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    ul: HtmlUListElement => {
        [Flow],
        [Tag("li")],
        [],
    },
    var: HtmlElement => {
        [Flow, Phrasing],
        [Model(Phrasing)],
        [],
    },
    video: HtmlVideoElement => {
        [Flow, Phrasing, Embedded, Interactive],
        [Tag("source"), Tag("track"), Transparent],
        ["src", "crossorigin", "poster", "preload", "autoplay", "playsinline", "loop", "muted",
            "controls", "width", "height"],
    },
    wbr: HtmlElement => {
        [Flow, Phrasing],
        [],
        [],
    },
]);
