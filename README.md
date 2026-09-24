# gpui-markup

A declarative markup DSL for building [GPUI](https://gpui.rs) applications.

## Installation

```bash
cargo add gpui-markup
```

### GPUI versions

`gpui-markup` generates builder calls and does not depend on a specific GPUI runtime.

For the current `gpui-unofficial` prerelease, rename the dependency to `gpui`:

```toml
[dependencies]
gpui = { package = "gpui-unofficial", version = "1.21.0-pre" }
gpui-markup = "0.5.2"
```

For upstream GPUI from Zed's Git `main`:

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed", branch = "main" }
gpui-markup = "0.5.2"
```

For GPUI Kit, use its matching GPUI APIs:

```toml
[dependencies]
gpui-kit = "0.6.6"
gpui-markup = "0.5.2"
```

```rust
use gpui_kit as gpui;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::{IntoElement, Styled, div};
use gpui_markup::ui;

fn toolbar() -> impl IntoElement {
    ui! {
        div @[flex, gap_2] {
            Button::new("save") @[primary, label: "Save"] {},
        }
    }
}
```

## Usage

```rust
use gpui::prelude::*;
use gpui::{FontWeight, div, px};
use gpui_markup::ui;

fn my_view() -> impl IntoElement {
    ui! {
        div @[flex, flex_col, gap_2, p_4] {
            div @[text_size: px(24.0), font_weight: FontWeight::BOLD] {
                "Hello, GPUI!",
            },
            div {
                "A declarative way to build UIs",
            },
        }
    }
}
```

## Syntax

### Elements

All elements require braces `{}`. Attributes go before braces with `@[...]`:

```rust
// Empty div
ui! { div {} }
// -> div()

// Div with attributes
ui! { div @[flex, flex_col] {} }
// -> div().flex().flex_col()

// Div with children
ui! { div { "content" } }
// -> gpui::ParentElement::child(div(), "content")

// Full form: attributes before braces, children inside
ui! { div @[flex] { "content" } }
// -> gpui::ParentElement::child(div().flex(), "content")
```

### Attributes

Attributes use `@[...]` before braces, comma-separated:

```rust
// Flag attributes (no value)
ui! { div @[flex, flex_col] {} }
// -> div().flex().flex_col()

// Key-value attributes
ui! { div @[w: px(200.0), h: px(100.0)] {} }
// -> div().w(px(200.0)).h(px(100.0))

// Multi-value attributes (use tuples)
ui! { div @[when: (condition, |d| d.bg(red()))] {} }
// -> div().when(condition, |d| d.bg(red()))
```

### Children

Children go inside `{...}`, comma-separated:

```rust
ui! {
    div {
        "First",
        "Second",
        div @[bold] { "Nested" },
    }
}
// -> gpui::ParentElement::child(
//      gpui::ParentElement::child(
//        gpui::ParentElement::child(div(), "First"),
//        "Second"
//      ),
//      gpui::ParentElement::child(div().bold(), "Nested")
//    )
```

### Deferred

The `deferred` element wraps content for deferred rendering:

```rust
ui! {
    deferred {
        div { "Deferred content" },
    }
}
// -> deferred(gpui::IntoElement::into_any_element(gpui::ParentElement::child(div(), "Deferred content")))
```

### Spread Children

Use `..expr` to spread an iterable as children:

```rust
let items: Vec<Div> = vec![div().child("A"), div().child("B")];

ui! {
    div {
        ..items,
    }
}
// -> gpui::ParentElement::children(div(), items)

// Can be mixed with regular children
ui! {
    div {
        "Header",
        ..items,
        "Footer",
    }
}
// -> gpui::ParentElement::child(
//      gpui::ParentElement::children(
//        gpui::ParentElement::child(div(), "Header"),
//        items
//      ),
//      "Footer"
//    )
```

### Method Chains

Use `.method(args)` to insert method calls at any position:

```rust
ui! {
    div {
        "static child",
        .when(condition, |d| d.child("dynamic")),
        .flex().gap_2(),
        .map::<Div, _>(|d| d),
    }
}
```

### Comments

Use standard Rust comments inside `ui!`:

```rust
ui! {
    div {
        // This is a comment
        "Visible content",
        /* Multi-line
           comment */
    }
}
// -> gpui::ParentElement::child(div(), "Visible content")
```

### Components

Components are any uppercase non-native element names. They automatically call `::new()`:

```rust
// Simple component
ui! { Header {} }
// -> Header::new()

// Component with attributes
ui! { Button @[style: Primary] {} }
// -> Button::new().style(Primary)

// Component with children
ui! {
    Container {
        "Content",
        Footer {},
    }
}
// -> gpui::ParentElement::child(gpui::ParentElement::child(Container::new(), "Content"), Footer::new())
```

### Expression Elements

Any expression can be used as an element at the top level (braces required):

```rust
// Custom constructor
ui! { Button::with_label("Click") {} }
// -> Button::with_label("Click")

// Expression with attributes
ui! { Button::with_label("Click") @[style: Primary] {} }
// -> Button::with_label("Click").style(Primary)

// Builder pattern expression
ui! {
    div().flex() @[flex_col] {
        "Content",
    }
}
// -> gpui::ParentElement::child(div().flex().flex_col(), "Content")

// Parentheses for complex expressions (braces optional)
ui! { (a + b) }
// -> a + b
```

**Why braces are required at top level?**

The `ui!` macro builds a GPUI component tree. At the top level, `{}` declares "this is a UI element":

- Marks this as a node in the component tree, not just an expression
- For components, `{}` triggers the implicit `::new()` call
- Provides a place for attributes `@[...]` and children

```rust
// Clear: defining a UI element, Header::new() is called
ui! { Header {} }

// As a child, context already indicates it's part of the tree
div {
    Header::new(),  // braces optional here, but no implicit ::new()
}
```

### Closure Parameters as Elements

When using closure parameters (e.g., from `.when()`, `.map()`) in nested `ui!` macros, lowercase identifiers are treated as expression elements, not components:

```rust
// Closure parameter as element
ui! {
    div {
        .when(selected, |s| {
            ui! {
                s {}  // `s` is treated as an expression element, not a component
            }
        })
    }
}
// -> div().when(selected, |s| { s })

// With attributes and children
ui! {
    div {
        .when(condition, |styled| {
            ui! {
                styled @[flex, gap_2] {
                    "Content",
                }
            }
        })
    }
}
// -> div().when(condition, |styled| {
//      gpui::ParentElement::child(styled.flex().gap_2(), "Content")
//    })
```

**How it works:**

- **Uppercase identifiers** (e.g., `Header`, `Button`) → Components, call `::new()` implicitly
- **Lowercase native elements** (`div`, `svg`, `anchored`) → Native GPUI elements
- **Other lowercase identifiers** (e.g., `s`, `element`, `styled`) → Expression elements (variables, parameters)

This allows seamless use of closure parameters from GPUI's builder methods like `.when()`, `.map()`, `.hover()`, etc.

### Nested Structures

```rust
ui! {
    div @[flex, flex_col, gap_4] {
        div @[flex, justify_between] {
            Label {},
            Button @[on_click: handle_click] {},
        },
        div @[flex: 1, overflow: hidden] {
            ScrollView { content },
        },
    }
}
```

## How It Works

The `ui!` macro transforms the markup syntax into GPUI's builder pattern at compile time:

| Markup | Equivalent Builder Code |
|--------|----------------|
| `div {}` | `div()` |
| `div @[flex] {}` | `div().flex()` |
| `div @[w: x] {}` | `div().w(x)` |
| `div @[when: (a, b)] {}` | `div().when(a, b)` |
| `div { a, b }` | `gpui::ParentElement::child(gpui::ParentElement::child(div(), a), b)` |
| `div { ..items }` | `gpui::ParentElement::children(div(), items)` |
| `div { .a().b() }` | `div().a().b()` |
| `deferred { e }` | `deferred(gpui::IntoElement::into_any_element(e))` |
| `Header {}` | `Header::new()` |
| `Header @[a] {}` | `Header::new().a()` |
| `expr {}` | `expr` |
| `expr @[a] {}` | `expr.a()` |
| `(expr)` | `expr` |

## License

[MIT](./LICENSE). Made with ❤️ by [Ray](https://github.com/so1ve)
