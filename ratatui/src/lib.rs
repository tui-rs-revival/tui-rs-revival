// show the feature flags in the generated documentation
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/ratatui/ratatui/main/assets/favicon.ico"
)]
#![warn(missing_docs)]
//! ![Demo](https://github.com/ratatui/ratatui/blob/87ae72dbc756067c97f6400d3e2a58eeb383776e/examples/demo2-destroy.gif?raw=true)
//!
//! <div align="center">
//!
//! [![Crate Badge]][Crate] [![Docs Badge]][API Docs] [![CI Badge]][CI Workflow] [![Deps.rs
//! Badge]][Deps.rs]<br> [![Codecov Badge]][Codecov] [![License Badge]](./LICENSE) [![Sponsors
//! Badge]][GitHub Sponsors]<br> [![Discord Badge]][Discord Server] [![Matrix Badge]][Matrix]
//! [![Forum Badge]][Forum]<br>
//!
//! [Ratatui Website] · [API Docs] · [Examples] · [Changelog] · [Breaking Changes]<br>
//! [Contributing] · [Report a bug] · [Request a Feature] · [Create a Pull Request]
//!
//! </div>
//!
//! # Ratatui
//!
//! [Ratatui][Ratatui Website] is a crate for cooking up terminal user interfaces in Rust. It is a
//! lightweight library that provides a set of widgets and utilities to build complex Rust TUIs.
//! Ratatui was forked from the [tui-rs] crate in 2023 in order to continue its development.
//!
//! ## Quickstart
//!
//! Add `ratatui` and `crossterm` as dependencies to your cargo.toml:
//!
//! ```shell
//! cargo add ratatui crossterm
//! ```
//!
//! Then you can create a simple "Hello World" application:
//!
//! ```rust,no_run
//! use crossterm::event::{self, Event};
//! use ratatui::{text::Text, Frame};
//!
//! fn main() {
//!     let mut terminal = ratatui::init();
//!     loop {
//!         terminal.draw(draw).expect("failed to draw frame");
//!         if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
//!             break;
//!         }
//!     }
//!     ratatui::restore();
//! }
//!
//! fn draw(frame: &mut Frame) {
//!     let text = Text::raw("Hello World!");
//!     frame.render_widget(text, frame.area());
//! }
//! ```
//!
//! The full code for this example which contains a little more detail is in the [Examples]
//! directory. For more guidance on different ways to structure your application see the
//! [Application Patterns] and [Hello World tutorial] sections in the [Ratatui Website] and the
//! various [Examples]. There are also several starter templates available in the [templates]
//! repository.
//!
//! ## Other documentation
//!
//! - [Ratatui Website] - explains the library's concepts and provides step-by-step tutorials
//! - [Ratatui Forum][Forum] - a place to ask questions and discuss the library
//! - [API Docs] - the full API documentation for the library on docs.rs.
//! - [Examples] - a collection of examples that demonstrate how to use the library.
//! - [Contributing] - Please read this if you are interested in contributing to the project.
//! - [Changelog] - generated by [git-cliff] utilizing [Conventional Commits].
//! - [Breaking Changes] - a list of breaking changes in the library.
//!
//! You can also watch the [FOSDEM 2024 talk] about Ratatui which gives a brief introduction to
//! terminal user interfaces and showcases the features of Ratatui, along with a hello world demo.
//!
//! ## Introduction
//!
//! Ratatui is based on the principle of immediate rendering with intermediate buffers. This means
//! that for each frame, your app must render all widgets that are supposed to be part of the UI.
//! This is in contrast to the retained mode style of rendering where widgets are updated and then
//! automatically redrawn on the next frame. See the [Rendering] section of the [Ratatui Website]
//! for more info.
//!
//! Ratatui uses [Crossterm] by default as it works on most platforms. See the [Installation]
//! section of the [Ratatui Website] for more details on how to use other backends ([Termion] /
//! [Termwiz]).
//!
//! Every application built with `ratatui` needs to implement the following steps:
//!
//! - Initialize the terminal
//! - A main loop that:
//!   - Draws the UI
//!   - Handles input events
//! - Restore the terminal state
//!
//! ### Initialize and restore the terminal
//!
//! The [`Terminal`] type is the main entry point for any Ratatui application. It is generic over a
//! a choice of [`Backend`] implementations that each provide functionality to draw frames, clear
//! the screen, hide the cursor, etc. There are backend implementations for [Crossterm], [Termion]
//! and [Termwiz].
//!
//! The simplest way to initialize the terminal is to use the [`init`] function which returns a
//! [`DefaultTerminal`] instance with the default options, enters the Alternate Screen and Raw mode
//! and sets up a panic hook that restores the terminal in case of panic. This instance can then be
//! used to draw frames and interact with the terminal state. (The [`DefaultTerminal`] instance is a
//! type alias for a terminal with the [`crossterm`] backend.) The [`restore`] function restores the
//! terminal to its original state.
//!
//! ```rust,no_run
//! fn main() -> std::io::Result<()> {
//!     let mut terminal = ratatui::init();
//!     let result = run(&mut terminal);
//!     ratatui::restore();
//!     result
//! }
//! # fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> { Ok(()) }
//! ```
//!
//! See the [`backend` module] and the [Backends] section of the [Ratatui Website] for more info on
//! the alternate screen and raw mode.
//!
//! ### Drawing the UI
//!
//! Drawing the UI is done by calling the [`Terminal::draw`] method on the terminal instance. This
//! method takes a closure that is called with a [`Frame`] instance. The [`Frame`] provides the size
//! of the area to draw to and allows the app to render any [`Widget`] using the provided
//! [`render_widget`] method. After this closure returns, a diff is performed and only the changes
//! are drawn to the terminal. See the [Widgets] section of the [Ratatui Website] for more info.
//!
//! The closure passed to the [`Terminal::draw`] method should handle the rendering of a full frame.
//!
//! ```rust,no_run
//! use ratatui::{widgets::Paragraph, Frame};
//!
//! fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
//!     loop {
//!         terminal.draw(|frame| draw(frame))?;
//!         if handle_events()? {
//!             break Ok(());
//!         }
//!     }
//! }
//!
//! fn draw(frame: &mut Frame) {
//!     let text = Paragraph::new("Hello World!");
//!     frame.render_widget(text, frame.area());
//! }
//! # fn handle_events() -> std::io::Result<bool> { Ok(false) }
//! ```
//!
//! ### Handling events
//!
//! Ratatui does not include any input handling. Instead event handling can be implemented by
//! calling backend library methods directly. See the [Handling Events] section of the [Ratatui
//! Website] for more info. For example, if you are using [Crossterm], you can use the
//! [`crossterm::event`] module to handle events.
//!
//! ```rust,no_run
//! use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
//!
//! fn handle_events() -> std::io::Result<bool> {
//!     match event::read()? {
//!         Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
//!             KeyCode::Char('q') => return Ok(true),
//!             // handle other key events
//!             _ => {}
//!         },
//!         // handle other events
//!         _ => {}
//!     }
//!     Ok(false)
//! }
//! ```
//!
//! ## Layout
//!
//! The library comes with a basic yet useful layout management object called [`Layout`] which
//! allows you to split the available space into multiple areas and then render widgets in each
//! area. This lets you describe a responsive terminal UI by nesting layouts. See the [Layout]
//! section of the [Ratatui Website] for more info.
//!
//! ```rust,no_run
//! use ratatui::{
//!     layout::{Constraint, Layout},
//!     widgets::Block,
//!     Frame,
//! };
//!
//! fn draw(frame: &mut Frame) {
//!     use Constraint::{Fill, Length, Min};
//!
//!     let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
//!     let [title_area, main_area, status_area] = vertical.areas(frame.area());
//!     let horizontal = Layout::horizontal([Fill(1); 2]);
//!     let [left_area, right_area] = horizontal.areas(main_area);
//!
//!     frame.render_widget(Block::bordered().title("Title Bar"), title_area);
//!     frame.render_widget(Block::bordered().title("Status Bar"), status_area);
//!     frame.render_widget(Block::bordered().title("Left"), left_area);
//!     frame.render_widget(Block::bordered().title("Right"), right_area);
//! }
//! ```
//!
//! Running this example produces the following output:
//!
//! ```text
//! Title Bar───────────────────────────────────
//! ┌Left────────────────┐┌Right───────────────┐
//! │                    ││                    │
//! └────────────────────┘└────────────────────┘
//! Status Bar──────────────────────────────────
//! ```
//!
//! ## Text and styling
//!
//! The [`Text`], [`Line`] and [`Span`] types are the building blocks of the library and are used in
//! many places. [`Text`] is a list of [`Line`]s and a [`Line`] is a list of [`Span`]s. A [`Span`]
//! is a string with a specific style.
//!
//! The [`style` module] provides types that represent the various styling options. The most
//! important one is [`Style`] which represents the foreground and background colors and the text
//! attributes of a [`Span`]. The [`style` module] also provides a [`Stylize`] trait that allows
//! short-hand syntax to apply a style to widgets and text. See the [Styling Text] section of the
//! [Ratatui Website] for more info.
//!
//! ```rust,no_run
//! use ratatui::{
//!     layout::{Constraint, Layout},
//!     style::{Color, Modifier, Style, Stylize},
//!     text::{Line, Span},
//!     widgets::{Block, Paragraph},
//!     Frame,
//! };
//!
//! fn draw(frame: &mut Frame) {
//!     let areas = Layout::vertical([Constraint::Length(1); 4]).split(frame.area());
//!
//!     let line = Line::from(vec![
//!         Span::raw("Hello "),
//!         Span::styled(
//!             "World",
//!             Style::new()
//!                 .fg(Color::Green)
//!                 .bg(Color::White)
//!                 .add_modifier(Modifier::BOLD),
//!         ),
//!         "!".red().on_light_yellow().italic(),
//!     ]);
//!     frame.render_widget(line, areas[0]);
//!
//!     // using the short-hand syntax and implicit conversions
//!     let paragraph = Paragraph::new("Hello World!".red().on_white().bold());
//!     frame.render_widget(paragraph, areas[1]);
//!
//!     // style the whole widget instead of just the text
//!     let paragraph = Paragraph::new("Hello World!").style(Style::new().red().on_white());
//!     frame.render_widget(paragraph, areas[2]);
//!
//!     // use the simpler short-hand syntax
//!     let paragraph = Paragraph::new("Hello World!").blue().on_yellow();
//!     frame.render_widget(paragraph, areas[3]);
//! }
//! ```
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! [Ratatui Website]: https://ratatui.rs/
//! [Installation]: https://ratatui.rs/installation/
//! [Rendering]: https://ratatui.rs/concepts/rendering/
//! [Application Patterns]: https://ratatui.rs/concepts/application-patterns/
//! [Hello World tutorial]: https://ratatui.rs/tutorials/hello-world/
//! [Backends]: https://ratatui.rs/concepts/backends/
//! [Widgets]: https://ratatui.rs/how-to/widgets/
//! [Handling Events]: https://ratatui.rs/concepts/event-handling/
//! [Layout]: https://ratatui.rs/how-to/layout/
//! [Styling Text]: https://ratatui.rs/how-to/render/style-text/
//! [templates]: https://github.com/ratatui/templates/
//! [Examples]: https://github.com/ratatui/ratatui/tree/main/ratatui/examples/README.md
//! [Report a bug]: https://github.com/ratatui/ratatui/issues/new?labels=bug&projects=&template=bug_report.md
//! [Request a Feature]: https://github.com/ratatui/ratatui/issues/new?labels=enhancement&projects=&template=feature_request.md
//! [Create a Pull Request]: https://github.com/ratatui/ratatui/compare
//! [git-cliff]: https://git-cliff.org
//! [Conventional Commits]: https://www.conventionalcommits.org
//! [API Docs]: https://docs.rs/ratatui
//! [Changelog]: https://github.com/ratatui/ratatui/blob/main/CHANGELOG.md
//! [Contributing]: https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md
//! [Breaking Changes]: https://github.com/ratatui/ratatui/blob/main/BREAKING-CHANGES.md
//! [FOSDEM 2024 talk]: https://www.youtube.com/watch?v=NU0q6NOLJ20
//! [`Frame`]: terminal::Frame
//! [`render_widget`]: terminal::Frame::render_widget
//! [`Widget`]: widgets::Widget
//! [`Layout`]: layout::Layout
//! [`Text`]: text::Text
//! [`Line`]: text::Line
//! [`Span`]: text::Span
//! [`Style`]: style::Style
//! [`style` module]: style
//! [`Stylize`]: style::Stylize
//! [`Backend`]: backend::Backend
//! [`backend` module]: backend
//! [`crossterm::event`]: https://docs.rs/crossterm/latest/crossterm/event/index.html
//! [Crate]: https://crates.io/crates/ratatui
//! [Crossterm]: https://crates.io/crates/crossterm
//! [Termion]: https://crates.io/crates/termion
//! [Termwiz]: https://crates.io/crates/termwiz
//! [tui-rs]: https://crates.io/crates/tui
//! [GitHub Sponsors]: https://github.com/sponsors/ratatui
//! [Crate Badge]: https://img.shields.io/crates/v/ratatui?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44
//! [License Badge]: https://img.shields.io/crates/l/ratatui?style=flat-square&color=1370D3
//! [CI Badge]: https://img.shields.io/github/actions/workflow/status/ratatui/ratatui/ci.yml?style=flat-square&logo=github
//! [CI Workflow]: https://github.com/ratatui/ratatui/actions/workflows/ci.yml
//! [Codecov Badge]: https://img.shields.io/codecov/c/github/ratatui/ratatui?logo=codecov&style=flat-square&token=BAQ8SOKEST&color=C43AC3&logoColor=C43AC3
//! [Codecov]: https://app.codecov.io/gh/ratatui/ratatui
//! [Deps.rs Badge]: https://deps.rs/repo/github/ratatui/ratatui/status.svg?style=flat-square
//! [Deps.rs]: https://deps.rs/repo/github/ratatui/ratatui
//! [Discord Badge]: https://img.shields.io/discord/1070692720437383208?label=discord&logo=discord&style=flat-square&color=1370D3&logoColor=1370D3
//! [Discord Server]: https://discord.gg/pMCEU9hNEj
//! [Docs Badge]: https://img.shields.io/docsrs/ratatui?logo=rust&style=flat-square&logoColor=E05D44
//! [Matrix Badge]: https://img.shields.io/matrix/ratatui-general%3Amatrix.org?style=flat-square&logo=matrix&label=Matrix&color=C43AC3
//! [Matrix]: https://matrix.to/#/#ratatui:matrix.org
//! [Forum Badge]: https://img.shields.io/discourse/likes?server=https%3A%2F%2Fforum.ratatui.rs&style=flat-square&logo=discourse&label=forum&color=C43AC3
//! [Forum]: https://forum.ratatui.rs
//! [Sponsors Badge]: https://img.shields.io/github/sponsors/ratatui?logo=github&style=flat-square&color=1370D3

/// re-export the `palette` crate so that users don't have to add it as a dependency
#[cfg(feature = "palette")]
pub use palette;
/// re-export the `crossterm` crate so that users don't have to add it as a dependency
#[cfg(feature = "crossterm")]
pub use ratatui_crossterm::crossterm;
/// re-export the `termion` crate so that users don't have to add it as a dependency
#[cfg(all(not(windows), feature = "termion"))]
pub use ratatui_termion::termion;
/// re-export the `termwiz` crate so that users don't have to add it as a dependency
#[cfg(feature = "termwiz")]
pub use ratatui_termwiz::termwiz;
#[cfg(feature = "crossterm")]
pub use terminal::{
    init, init_with_options, restore, try_init, try_init_with_options, try_restore, DefaultTerminal,
};
pub use terminal::{CompletedFrame, Frame, Terminal, TerminalOptions, Viewport};

/// Re-exports for the backend implementations.
pub mod backend {
    pub use ratatui_core::backend::{Backend, TestBackend};
    #[cfg(feature = "crossterm")]
    pub use ratatui_crossterm::{CrosstermBackend, FromCrossterm, IntoCrossterm};
    #[cfg(all(not(windows), feature = "termion"))]
    pub use ratatui_termion::{FromTermion, IntoTermion, TermionBackend};
    #[cfg(feature = "termwiz")]
    pub use ratatui_termwiz::{FromTermwiz, IntoTermwiz, TermwizBackend};
}

pub use ratatui_core::{buffer, layout};
pub mod prelude;
pub use ratatui_core::{style, symbols};
mod terminal;
pub use ratatui_core::text;
pub mod widgets;
pub use ratatui_widgets::border;
