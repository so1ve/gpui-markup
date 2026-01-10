# gpui-markup

A declarative markup DSL for building [GPUI](https://gpui.rs) applications.

## Installation

```toml
[dependencies]
gpui-markup = "0.1"
```

## Usage

```rust
use gpui::prelude::*;
use gpui_markup::ui;

fn my_view(cx: &mut ViewContext<Self>) -> impl IntoElement {
    ui! {
        <div flex flex_col gap_2 p_4 bg={cx.theme().colors().background}>
            <div text_size={px(24.0)} font_weight={FontWeight::BOLD}>
                {"Hello, GPUI!"}
            </div>
            <div text_color={cx.theme().colors().text_muted}>
                {"A declarative way to build UIs"}
            </div>
        </div>
    }
}
```

## Syntax

### Elements

```rust
// Self-closing div
ui! { <div/> }
// -> div()

// Div with children
ui! { <div>{"content"}</div> }
// -> div().child("content")
```

### Deferred

The `<deferred>` element wraps content for deferred rendering. The child must implement `IntoElement`:

```rust
ui! {
    <deferred>
        <div>{"Deferred content"}</div>
    </deferred>
}
// -> deferred(div().child("Deferred content").into_any_element())

// Can also use expressions
let element = div().child("Content");
ui! {
    <deferred>
        {element}
    </deferred>
}
// -> deferred(element.into_any_element())
```

### Attributes

```rust
// Flag attributes (no value)
ui! { <div flex flex_col/> }
// -> div().flex().flex_col()

// Key-value attributes
ui! { <div w={px(200.0)} h={px(100.0)}/> }
// -> div().w(px(200.0)).h(px(100.0))

// Multi-value attributes
ui! { <div when={condition, |d| d.bg(red())}/> }
// -> div().when(condition, |d| d.bg(red()))
```

### Children

```rust
// Children use chained .child() calls
ui! {
    <div>
        {"First"}
        {"Second"}
    </div>
}
// -> div().child("First").child("Second")
```

### Spread Children

Use `{..expr}` to spread an iterable as children:

```rust
let items: Vec<Div> = vec![div().child("A"), div().child("B")];

// Spread an iterable
ui! {
    <div>
        {..items}
    </div>
}
// -> div().children(items)

// Can be mixed with regular children
ui! {
    <div>
        {"Header"}
        {..items}
        {"Footer"}
    </div>
}
// -> div().child("Header").children(items).child("Footer")
```

### Method Chains

Use `{.method(args)}` to insert method calls at any position. Supports method chains and generics:

```rust
ui! {
    <div>
        {"static child"}
        {.when(condition, |d| d.child("dynamic"))}
        {.flex().gap_2()}
        {.map::<Div, _>(|d| d)}
    </div>
}
```

This gives you full control over method call order.

### Comments

Use standard Rust comments inside `ui!`:

```rust
ui! {
    <div>
        // This is a comment
        {"Visible content"}
        /* Multi-line
           comment */
    </div>
}
// -> div().child("Visible content")
```

### Components

Components used with `<Component/>` syntax must:
1. Have a `new()` constructor
2. Implement `IntoElement` (use `#[derive(IntoElement)]`)

```rust
#[derive(IntoElement)]
struct Header {
    title: SharedString,
}

impl Header {
    fn new() -> Self {
        Self { title: "Header".into() }
    }
}

impl RenderOnce for Header {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div().child(self.title)
    }
}
```

Usage:

```rust
// Simple component (must be self-closing)
ui! { <Header/> }
// -> Header::new()

// Component with configuration - use expression syntax
ui! { <{Button::new("click me").style(ButtonStyle::Primary)}/> }
// -> Button::new("click me").style(ButtonStyle::Primary)
```

### Expression Tags

For elements that need method chains or complex initialization:

```rust
// Self-closing expression
ui! { <{icon.clone()}/> }
// -> icon.clone()

// Expression with attributes and children
ui! {
    <{Container::new()} flex p_4>
        {"Content"}
    </{}>
}
// -> Container::new().flex().p_4().child("Content")
```

### Nested Structures

```rust
ui! {
    <div flex flex_col gap_4>
        <div flex justify_between>
            <{Label::new("Title")}/>
            <{Button::new("Action")}/>
        </div>
        <div flex_1 overflow_hidden>
            <{ScrollView::new(content)}/>
        </div>
    </div>
}
```

### Nested Macros

When an attribute accepts a closure that returns an element (e.g. for a list delegate), you can use the `ui!` macro inside that closure:

```rust
ui! {
    <List
        delegate={move |ix| ui! {
            <ListItem>
                {"Item "}{ix}
            </ListItem>
        }}
    />
}
```

## How It Works

The `ui!` macro transforms the JSX-like syntax into GPUI's builder pattern at compile time:

| Markup | Generated Code |
|--------|----------------|
| `<div/>` | `div()` |
| `<div flex/>` | `div().flex()` |
| `<div w={x}/>` | `div().w(x)` |
| `<div when={a, b}/>` | `div().when(a, b)` |
| `<div>{a}{b}</div>` | `div().child(a).child(b)` |
| `<div>{..items}</div>` | `div().children(items)` |
| `<div>{.a().b()}</div>` | `div().a().b()` |
| `<deferred>{e}</deferred>` | `deferred(e.into_any_element())` |
| `<Foo/>` | `Foo::new()` |
| `<{expr}/>` | `expr` |

## License

[MIT](./LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
