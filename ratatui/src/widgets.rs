#![warn(missing_docs)]
//! `widgets` is a collection of types that implement [`Widget`] or [`StatefulWidget`] or both.
//!
//! Widgets are created for each frame as they are consumed after rendered.
//! They are not meant to be stored but used as *commands* to draw common figures in the UI.
//!
//! The available widgets are:
//! - [`Block`]: a basic widget that draws a block with optional borders, titles and styles.
//! - [`BarChart`]: displays multiple datasets as bars with optional grouping.
//! - [`calendar::Monthly`]: displays a single month.
//! - [`Canvas`]: draws arbitrary shapes using drawing characters.
//! - [`Chart`]: displays multiple datasets as a lines or scatter graph.
//! - [`Clear`]: clears the area it occupies. Useful to render over previously drawn widgets.
//! - [`Gauge`]: displays progress percentage using block characters.
//! - [`LineGauge`]: display progress as a line.
//! - [`List`]: displays a list of items and allows selection.
//! - [`Paragraph`]: displays a paragraph of optionally styled and wrapped text.
//! - [`Scrollbar`]: displays a scrollbar.
//! - [`Sparkline`]: display a single data set as a sparkline.
//! - [`Table`]: displays multiple rows and columns in a grid and allows selection.
//! - [`Tabs`]: displays a tab bar and allows selection.
//!
//! [`Canvas`]: crate::widgets::canvas::Canvas

pub use ratatui_widgets::{
    canvas, Axis, Bar, BarChart, BarGroup, Block, BorderType, Borders, Cell, Chart, Clear, Dataset,
    Gauge, GraphType, HighlightSpacing, LegendPosition, LineGauge, List, ListDirection, ListItem,
    ListState, Padding, Paragraph, RatatuiLogo, RatatuiLogoSize, RenderDirection, Row,
    ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState, Sparkline, SparklineBar,
    Table, TableState, Tabs, Wrap,
};

// TODO remove this module once title etc. are gone
pub use ratatui_widgets::block;

#[cfg(feature = "widget-calendar")]
pub use ratatui_widgets::calendar;

pub use ratatui_core::widgets::{StatefulWidget, Widget};
#[instability::unstable(feature = "widget-ref")]
pub use ratatui_core::widgets::{StatefulWidgetRef, WidgetRef};
