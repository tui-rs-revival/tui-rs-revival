#![warn(missing_docs)]
use strum::{Display, EnumString};

use super::StatefulWidget;
use crate::{
    prelude::*,
    symbols::scrollbar::{Set, DOUBLE_HORIZONTAL, DOUBLE_VERTICAL},
};

/// An enum representing a scrolling direction.
///
/// This is used with [`ScrollbarState::scroll`].
///
/// It is useful for example when you want to store in which direction to scroll.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ScrollDirection {
    /// Forward scroll direction, usually corresponds to scrolling downwards or rightwards.
    #[default]
    Forward,
    /// Backward scroll direction, usually corresponds to scrolling upwards or leftwards.
    Backward,
}

/// A struct representing the state of a Scrollbar widget.
///
/// # Important
///
/// It's essential to set the `content_length` field when using this struct. This field
/// represents the total length of the scrollable content. The default value is zero
/// which will result in the Scrollbar not rendering.
///
/// For example, in the following list, assume there are 4 bullet points:
///
/// - the `content_length` is 4
/// - the `position` is 0
/// - the `viewport_content_length` is 2
///
/// ```text
/// ┌───────────────┐
/// │1. this is a   █
/// │   single item █
/// │2. this is a   ║
/// │   second item ║
/// └───────────────┘
/// ```
///
/// If you don't have multi-line content, you can leave the `viewport_content_length` set to the
/// default of 0 and it'll use the track size as a `viewport_content_length`.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScrollbarState {
    /// The total length of the scrollable content.
    content_length: usize,
    /// The current position within the scrollable content.
    position: usize,
    /// The length of content in current viewport.
    viewport_content_length: usize,
}

impl ScrollbarState {
    /// Constructs a new ScrollbarState with the specified content length.
    ///
    /// `content_length` is the total number of element, that can be scrolled. See
    /// [`ScrollbarState`] for more details.
    pub fn new(content_length: usize) -> Self {
        Self {
            content_length,
            ..Default::default()
        }
    }

    /// Sets the scroll position of the scrollbar.
    ///
    /// This represents the number of scrolled items.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Sets the length of the scrollable content.
    ///
    /// This is the number of scrollable items. If items have a length of one, then this is the
    /// same as the number of scrollable cells.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn content_length(mut self, content_length: usize) -> Self {
        self.content_length = content_length;
        self
    }

    /// Sets the items' size.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn viewport_content_length(mut self, viewport_content_length: usize) -> Self {
        self.viewport_content_length = viewport_content_length;
        self
    }

    /// Decrements the scroll position by one, ensuring it doesn't go below zero.
    pub fn prev(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    /// Increments the scroll position by one, ensuring it doesn't exceed the length of the content.
    pub fn next(&mut self) {
        self.position = self
            .position
            .saturating_add(1)
            .min(self.content_length.saturating_sub(1))
    }

    /// Sets the scroll position to the start of the scrollable content.
    pub fn first(&mut self) {
        self.position = 0;
    }

    /// Sets the scroll position to the end of the scrollable content.
    pub fn last(&mut self) {
        self.position = self.content_length.saturating_sub(1)
    }

    /// Changes the scroll position based on the provided [`ScrollDirection`].
    pub fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Forward => {
                self.next();
            }
            ScrollDirection::Backward => {
                self.prev();
            }
        }
    }
}

/// This is the position of the scrollbar around a given area.
///
/// ```plain
///           HorizontalTop
///             ┌───────┐
/// VerticalLeft│       │VerticalRight
///             └───────┘
///          HorizontalBottom
/// ```
#[derive(Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash)]
pub enum ScrollbarOrientation {
    /// Positions the scrollbar on the right, scrolling vertically
    #[default]
    VerticalRight,
    /// Positions the scrollbar on the left, scrolling vertically
    VerticalLeft,
    /// Positions the scrollbar on the bottom, scrolling horizontally
    HorizontalBottom,
    /// Positions the scrollbar on the top, scrolling horizontally
    HorizontalTop,
}

/// A widget to display a scrollbar
///
/// The following components of the scrollbar are customizable in symbol and style. Note the
/// scrollbar is represented horizontally but it can also be set vertically (which is actually the
/// default).
///
/// ```text
/// <--▮------->
/// ^  ^   ^   ^
/// │  │   │   └ end
/// │  │   └──── track
/// │  └──────── thumb
/// └─────────── begin
/// ```
///
/// # Examples
///
/// ```rust
/// use ratatui::{prelude::*, widgets::*};
///
/// # fn render_paragraph_with_scrollbar(frame: &mut Frame, area: Rect) {
/// let vertical_scroll = 0; // from app state
///
/// let items = vec![
///     Line::from("Item 1"),
///     Line::from("Item 2"),
///     Line::from("Item 3"),
/// ];
/// let paragraph = Paragraph::new(items.clone())
///     .scroll((vertical_scroll as u16, 0))
///     .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar
///
/// let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
///     .begin_symbol(Some("↑"))
///     .end_symbol(Some("↓"));
///
/// let mut scrollbar_state = ScrollbarState::new(items.len()).position(vertical_scroll);
///
/// let area = frame.size();
/// // Note we render the paragraph
/// frame.render_widget(paragraph, area);
/// // and the scrollbar, those are separate widgets
/// frame.render_stateful_widget(
///     scrollbar,
///     area.inner(&Margin {
///         // using an inner vertical margin of 1 unit makes the scrollbar inside the block
///         vertical: 1,
///         horizontal: 0,
///     }),
///     &mut scrollbar_state,
/// );
/// # }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Scrollbar<'a> {
    orientation: ScrollbarOrientation,
    thumb_style: Style,
    thumb_symbol: &'a str,
    track_style: Style,
    track_symbol: Option<&'a str>,
    begin_symbol: Option<&'a str>,
    begin_style: Style,
    end_symbol: Option<&'a str>,
    end_style: Style,
}

impl<'a> Default for Scrollbar<'a> {
    fn default() -> Self {
        Self {
            orientation: ScrollbarOrientation::default(),
            thumb_symbol: DOUBLE_VERTICAL.thumb,
            thumb_style: Style::default(),
            track_symbol: Some(DOUBLE_VERTICAL.track),
            track_style: Style::default(),
            begin_symbol: Some(DOUBLE_VERTICAL.begin),
            begin_style: Style::default(),
            end_symbol: Some(DOUBLE_VERTICAL.end),
            end_style: Style::default(),
        }
    }
}

impl<'a> Scrollbar<'a> {
    /// Creates a new scrollbar with the given position.
    ///
    /// Most of the time you'll want [`ScrollbarOrientation::VerticalLeft`] or
    /// [`ScrollbarOrientation::HorizontalBottom`]. See [`ScrollbarOrientation`] for more options.
    pub fn new(orientation: ScrollbarOrientation) -> Self {
        Self::default().orientation(orientation)
    }

    /// Sets the position of the scrollbar.
    ///
    /// The orientation of the scrollbar is the position it will take around a [`Rect`]. See
    /// [`ScrollbarOrientation`] for more details.
    ///
    /// Resets the symbols to [`DOUBLE_VERTICAL`] or [`DOUBLE_HORIZONTAL`] based on orientation.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn orientation(mut self, orientation: ScrollbarOrientation) -> Self {
        self.orientation = orientation;
        let set = if self.is_vertical() {
            DOUBLE_VERTICAL
        } else {
            DOUBLE_HORIZONTAL
        };
        self.symbols(set)
    }

    /// Sets the orientation and symbols for the scrollbar from a [`Set`].
    ///
    /// This has the same effect as calling [`Scrollbar::orientation`] and then
    /// [`Scrollbar::symbols`]. See those for more details.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn orientation_and_symbol(mut self, orientation: ScrollbarOrientation, set: Set) -> Self {
        self.orientation = orientation;
        self.symbols(set)
    }

    /// Sets the symbol that represents the thumb of the scrollbar.
    ///
    /// The thumb is the handle representing the progression on the scrollbar. See [`Scrollbar`]
    /// for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn thumb_symbol(mut self, thumb_symbol: &'a str) -> Self {
        self.thumb_symbol = thumb_symbol;
        self
    }

    /// Sets the style on the scrollbar thumb.
    ///
    /// The thumb is the handle representing the progression on the scrollbar. See [`Scrollbar`]
    /// for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn thumb_style<S: Into<Style>>(mut self, thumb_style: S) -> Self {
        self.thumb_style = thumb_style.into();
        self
    }

    /// Sets the symbol that represents the track of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn track_symbol(mut self, track_symbol: Option<&'a str>) -> Self {
        self.track_symbol = track_symbol;
        self
    }

    /// Sets the style that is used for the track of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn track_style<S: Into<Style>>(mut self, track_style: S) -> Self {
        self.track_style = track_style.into();
        self
    }

    /// Sets the symbol that represents the beginning of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn begin_symbol(mut self, begin_symbol: Option<&'a str>) -> Self {
        self.begin_symbol = begin_symbol;
        self
    }

    /// Sets the style that is used for the beginning of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn begin_style<S: Into<Style>>(mut self, begin_style: S) -> Self {
        self.begin_style = begin_style.into();
        self
    }

    /// Sets the symbol that represents the end of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn end_symbol(mut self, end_symbol: Option<&'a str>) -> Self {
        self.end_symbol = end_symbol;
        self
    }

    /// Sets the style that is used for the end of the scrollbar.
    ///
    /// See [`Scrollbar`] for a visual example of what this represents.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn end_style<S: Into<Style>>(mut self, end_style: S) -> Self {
        self.end_style = end_style.into();
        self
    }

    /// Sets the symbols used for the various parts of the scrollbar from a [`Set`].
    ///
    /// ```text
    /// <--▮------->
    /// ^  ^   ^   ^
    /// │  │   │   └ end
    /// │  │   └──── track
    /// │  └──────── thumb
    /// └─────────── begin
    /// ```
    ///
    /// Only sets begin_symbol, end_symbol and track_symbol if they already contain a value.
    /// If they were set to `None` explicitly, this function will respect that choice. Use their
    /// respective setters to change their value.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn symbols(mut self, symbol: Set) -> Self {
        self.thumb_symbol = symbol.thumb;
        if self.track_symbol.is_some() {
            self.track_symbol = Some(symbol.track);
        }
        if self.begin_symbol.is_some() {
            self.begin_symbol = Some(symbol.begin);
        }
        if self.end_symbol.is_some() {
            self.end_symbol = Some(symbol.end);
        }
        self
    }

    /// Sets the style used for the various parts of the scrollbar from a [`Style`].
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// ```text
    /// <--▮------->
    /// ^  ^   ^   ^
    /// │  │   │   └ end
    /// │  │   └──── track
    /// │  └──────── thumb
    /// └─────────── begin
    /// ```
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        let style = style.into();
        self.track_style = style;
        self.thumb_style = style;
        self.begin_style = style;
        self.end_style = style;
        self
    }

    fn is_vertical(&self) -> bool {
        match self.orientation {
            ScrollbarOrientation::VerticalRight | ScrollbarOrientation::VerticalLeft => true,
            ScrollbarOrientation::HorizontalBottom | ScrollbarOrientation::HorizontalTop => false,
        }
    }

    fn get_track_info(&self, area: Rect) -> (u16, u16, u16, u16) {
        let (mut track_start, mut track_end, mut track_size, viewport_size) = if self.is_vertical()
        {
            (
                area.y,
                (area.y + area.height.saturating_sub(1)),
                area.height,
                area.height,
            )
        } else {
            (
                area.x,
                (area.x + area.width.saturating_sub(1)),
                area.width,
                area.width,
            )
        };
        if let Some(s) = self.begin_symbol {
            track_size = track_size.saturating_sub(s.len() as u16);
            track_start = track_start.saturating_add(1);
        };
        if let Some(s) = self.end_symbol {
            track_size = track_size.saturating_sub(s.len() as u16);
            track_end = track_end.saturating_sub(1);
        };
        (track_start, track_end, track_size, viewport_size)
    }

    fn get_track_axis(&self, area: Rect) -> u16 {
        if self.is_vertical() {
            area.x
        } else {
            area.y
        }
    }

    fn get_thumb_start_end(&self, area: Rect, state: &mut ScrollbarState) -> (u16, u16) {
        let (track_start, track_end, track_size, viewport_size) = self.get_track_info(area);
        let viewport_size = viewport_size as f64;

        let track_size = track_size as f64;
        let content_size = state.content_length as f64;
        let position = state.position as f64;

        let (thumb_position, thumb_size) = if content_size == 0.0 {
            let thumb_position = 0;
            let thumb_size = track_size as u16;
            (thumb_position, thumb_size)
        } else {
            let scroll_ratio = position / content_size;
            let thumb_position = (scroll_ratio * track_size).round() as u16;

            let thumb_ratio = viewport_size / (content_size + viewport_size);
            let thumb_size = (thumb_ratio * track_size).round() as u16;
            (thumb_position, thumb_size)
        };

        let thumb_start = (track_start + thumb_position).min(track_end.saturating_sub(thumb_size));
        let thumb_end = (thumb_start + thumb_size).min(track_end);

        (thumb_start, thumb_end)
    }

    //          1234567890
    // Renders: ·════════·
    fn render_track(&self, area: Rect, buf: &mut Buffer) {
        let (track_start, track_end, _, _) = self.get_track_info(area);
        let track_axis = self.get_track_axis(area);

        for i in track_start..=track_end {
            let (symbol, style) = if let Some(track_symbol) = self.track_symbol {
                (track_symbol, self.track_style)
            } else {
                continue;
            };
            if self.is_vertical() {
                buf.set_string(track_axis, i, symbol, style);
            } else {
                buf.set_string(i, track_axis, symbol, style);
            }
        }
    }

    //          1234567890
    // Renders: ·██══════·
    fn render_thumb(&self, area: Rect, buf: &mut Buffer, state: &mut ScrollbarState) {
        let track_axis = self.get_track_axis(area);
        let (thumb_start, thumb_end) = self.get_thumb_start_end(area, state);
        for i in thumb_start..=thumb_end {
            let (style, symbol) = (self.thumb_style, self.thumb_symbol);
            if self.is_vertical() {
                buf.set_string(track_axis, i, symbol, style);
            } else {
                buf.set_string(i, track_axis, symbol, style);
            }
        }
    }

    //          1234567890
    // Renders: ◄██══════►
    fn render_arrowheads(&self, area: Rect, buf: &mut Buffer) {
        let track_axis = self.get_track_axis(area);
        let (track_start, track_end, _, _) = self.get_track_info(area);
        if let Some(s) = self.begin_symbol {
            if self.is_vertical() {
                buf.set_string(
                    track_axis,
                    track_start.saturating_sub(1),
                    s,
                    self.begin_style,
                );
            } else {
                buf.set_string(
                    track_start.saturating_sub(1),
                    track_axis,
                    s,
                    self.begin_style,
                );
            }
        };
        if let Some(s) = self.end_symbol {
            if self.is_vertical() {
                buf.set_string(track_axis, track_end.saturating_add(1), s, self.end_style);
            } else {
                buf.set_string(track_end.saturating_add(1), track_axis, s, self.end_style);
            }
        }
    }
}

impl<'a> StatefulWidget for Scrollbar<'a> {
    type State = ScrollbarState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_track(area, buf);
        self.render_thumb(area, buf, state);
        self.render_arrowheads(area, buf);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_debug() {
        let content_length = 100;
        for p in 0..content_length {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 50, 1));
            let mut state = ScrollbarState::default()
                .position(p)
                .content_length(content_length);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            println!("{:?}", buffer);
        }
    }

    #[test]
    fn test_rendering_empty() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(0).content_length(0);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .render(buffer.area, &mut buffer, &mut state);
        let expected = "◄████████►";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
    }

    #[test]
    fn test_rendering_thumb_half_with_symbols() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(0).content_length(10);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(Some(" "))
            .end_symbol(Some(" "))
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = " ████════ ";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(5).content_length(10);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(Some(" "))
            .end_symbol(Some(" "))
            .render(buffer.area, &mut buffer, &mut state);
        let expected = " ══████══ ";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(10).content_length(10);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(Some(" "))
            .end_symbol(Some(" "))
            .render(buffer.area, &mut buffer, &mut state);
        let expected = " ════████ ";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
    }

    #[test]
    fn test_rendering_thumb_half() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(0).content_length(10);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "█████═════";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(5).content_length(10);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "═══█████══";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(10).content_length(10);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "═════█████";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
    }

    #[test]
    fn test_rendering_thumb_third() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 50, 1));
        let mut state = ScrollbarState::default().position(50).content_length(100);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        let expected = "════════════════████████████████══════════════════";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
    }

    #[test]
    fn test_rendering_thumb_quarter() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 1));
        let mut state = ScrollbarState::default().position(0).content_length(50);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "██════════";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
        let mut state = ScrollbarState::default().position(10).content_length(50);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "══██══════";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
        let mut state = ScrollbarState::default().position(20).content_length(50);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "═══██═════";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
        let mut state = ScrollbarState::default().position(30).content_length(50);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "═════██═══";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
        let mut state = ScrollbarState::default().position(40).content_length(50);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "═══════██═";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
        let mut state = ScrollbarState::default().position(50).content_length(50);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        //             "1234567890"
        let expected = "════════██";
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![expected]));
    }

    use strum::ParseError;

    use super::*;
    use crate::{
        assert_buffer_eq,
        symbols::scrollbar::{HORIZONTAL, VERTICAL},
    };

    #[test]
    fn scroll_direction_to_string() {
        assert_eq!(ScrollDirection::Forward.to_string(), "Forward");
        assert_eq!(ScrollDirection::Backward.to_string(), "Backward");
    }

    #[test]
    fn scroll_direction_from_str() {
        assert_eq!(
            "Forward".parse::<ScrollDirection>(),
            Ok(ScrollDirection::Forward)
        );
        assert_eq!(
            "Backward".parse::<ScrollDirection>(),
            Ok(ScrollDirection::Backward)
        );
        assert_eq!(
            "".parse::<ScrollDirection>(),
            Err(ParseError::VariantNotFound)
        );
    }

    #[test]
    fn scrollbar_orientation_to_string() {
        assert_eq!(
            ScrollbarOrientation::VerticalRight.to_string(),
            "VerticalRight"
        );
        assert_eq!(
            ScrollbarOrientation::VerticalLeft.to_string(),
            "VerticalLeft"
        );
        assert_eq!(
            ScrollbarOrientation::HorizontalBottom.to_string(),
            "HorizontalBottom"
        );
        assert_eq!(
            ScrollbarOrientation::HorizontalTop.to_string(),
            "HorizontalTop"
        );
    }

    #[test]
    fn scrollbar_orientation_from_str() {
        assert_eq!(
            "VerticalRight".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::VerticalRight)
        );
        assert_eq!(
            "VerticalLeft".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::VerticalLeft)
        );
        assert_eq!(
            "HorizontalBottom".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::HorizontalBottom)
        );
        assert_eq!(
            "HorizontalTop".parse::<ScrollbarOrientation>(),
            Ok(ScrollbarOrientation::HorizontalTop)
        );
        assert_eq!(
            "".parse::<ScrollbarOrientation>(),
            Err(ParseError::VariantNotFound)
        );
    }

    #[test]
    fn test_renders_empty_with_content_length_is_zero() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 8));
        let mut state = ScrollbarState::default().position(0);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec!["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "])
        );

        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 8));
        let mut state = ScrollbarState::new(8).position(0);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![" █", " █", " █", " █", " █", " █", " █", " █"])
        );
    }

    #[test]
    fn test_no_render_when_area_zero() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 0));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));
    }

    #[test]
    fn test_no_render_when_height_zero_with_without_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 0));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 3, 0));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::empty(buffer.area));
    }

    #[test]
    fn test_no_render_when_height_too_small_for_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["    ", "    "]));
    }

    #[test]
    fn test_renders_all_thumbs_at_minimum_height_without_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["   █", "   █"]));
    }

    #[test]
    fn test_renders_all_thumbs_at_minimum_height_and_minimum_width_without_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["█", "█"]));
    }

    #[test]
    fn test_renders_two_arrows_one_thumb_at_minimum_height_with_arrows() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 3));
        let mut state = ScrollbarState::default().position(0).content_length(1);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["   ▲", "   █", "   ▼"]));
    }

    #[test]
    fn test_no_render_when_content_length_zero() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 2));
        let mut state = ScrollbarState::default().position(0).content_length(0);
        Scrollbar::default().render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec!["  ", "  "]));
    }

    #[test]
    fn test_renders_all_thumbs_when_height_equals_content_length() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 2));
        let mut state = ScrollbarState::default().position(0).content_length(2);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(buffer, Buffer::with_lines(vec![" █", " █"]));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 8));
        let mut state = ScrollbarState::default().position(0).content_length(8);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .render(buffer.area, &mut buffer, &mut state);
        assert_buffer_eq!(
            buffer,
            Buffer::with_lines(vec![" █", " █", " █", " █", " █", " █", " █", " █"])
        );
    }

    #[test]
    fn test_renders_single_vertical_thumb_when_content_length_square_of_height() {
        for i in 0..=17 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 4));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 2 {
                vec![" █", " ║", " ║", " ║"]
            } else if i <= 7 {
                vec![" ║", " █", " ║", " ║"]
            } else if i <= 13 {
                vec![" ║", " ║", " █", " ║"]
            } else {
                vec![" ║", " ║", " ║", " █"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_single_horizontal_thumb_when_content_length_square_of_width() {
        for i in 0..=17 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 2 {
                vec!["    ", "█═══"]
            } else if i <= 7 {
                vec!["    ", "═█══"]
            } else if i <= 13 {
                vec!["    ", "══█═"]
            } else {
                vec!["    ", "═══█"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_one_thumb_for_large_content_relative_to_height() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(0).content_length(1600);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .render(buffer.area, &mut buffer, &mut state);
        let expected = vec!["    ", "█═══"];
        assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));

        let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
        let mut state = ScrollbarState::default().position(800).content_length(1600);
        Scrollbar::default()
            .begin_symbol(None)
            .end_symbol(None)
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .render(buffer.area, &mut buffer, &mut state);
        let expected = vec!["    ", "══█═"];
        assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
    }

    #[test]
    fn test_renders_two_thumb_default_symbols_for_content_double_height() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 4));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec![" █", " █", " ║", " ║"]
            } else if i <= 5 {
                vec![" ║", " █", " █", " ║"]
            } else {
                vec![" ║", " ║", " █", " █"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_two_thumb_custom_symbols_for_content_double_height() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 2, 4));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .symbols(VERTICAL)
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec![" █", " █", " │", " │"]
            } else if i <= 5 {
                vec![" │", " █", " █", " │"]
            } else {
                vec![" │", " │", " █", " █"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_two_thumb_default_symbols_for_content_double_width() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["    ", "██══"]
            } else if i <= 5 {
                vec!["    ", "═██═"]
            } else {
                vec!["    ", "══██"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_renders_two_thumb_custom_symbols_for_content_double_width() {
        for i in 0..=7 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 4, 2));
            let mut state = ScrollbarState::default().position(i).content_length(8);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .symbols(HORIZONTAL)
                .begin_symbol(None)
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["    ", "██──"]
            } else if i <= 5 {
                vec!["    ", "─██─"]
            } else {
                vec!["    ", "──██"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_viewport_content_length() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default()
                .position(i)
                .content_length(16)
                .viewport_content_length(4);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄██════►"]
            } else if i <= 5 {
                vec!["        ", "◄═██═══►"]
            } else if i <= 9 {
                vec!["        ", "◄══██══►"]
            } else if i <= 13 {
                vec!["        ", "◄═══██═►"]
            } else {
                vec!["        ", "◄════██►"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }

        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default()
                .position(i)
                .content_length(16)
                .viewport_content_length(1);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄█═════►"]
            } else if i <= 4 {
                vec!["        ", "◄═█════►"]
            } else if i <= 7 {
                vec!["        ", "◄══█═══►"]
            } else if i <= 11 {
                vec!["        ", "◄═══█══►"]
            } else if i <= 14 {
                vec!["        ", "◄════█═►"]
            } else {
                vec!["        ", "◄═════█►"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_begin_end_arrows_horizontal_bottom() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄██════►"]
            } else if i <= 5 {
                vec!["        ", "◄═██═══►"]
            } else if i <= 9 {
                vec!["        ", "◄══██══►"]
            } else if i <= 13 {
                vec!["        ", "◄═══██═►"]
            } else {
                vec!["        ", "◄════██►"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_begin_end_arrows_horizontal_top() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalTop)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(Some(DOUBLE_HORIZONTAL.end))
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["◄██════►", "        "]
            } else if i <= 5 {
                vec!["◄═██═══►", "        "]
            } else if i <= 9 {
                vec!["◄══██══►", "        "]
            } else if i <= 13 {
                vec!["◄═══██═►", "        "]
            } else {
                vec!["◄════██►", "        "]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_only_begin_arrow_horizontal_bottom() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(Some(DOUBLE_HORIZONTAL.begin))
                .end_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄███════"]
            } else if i <= 5 {
                vec!["        ", "◄═███═══"]
            } else if i <= 9 {
                vec!["        ", "◄══███══"]
            } else if i <= 13 {
                vec!["        ", "◄═══███═"]
            } else {
                vec!["        ", "◄════███"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }

    #[test]
    fn test_rendering_without_track_horizontal_bottom() {
        for i in 0..=16 {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 2));
            let mut state = ScrollbarState::default().position(i).content_length(16);
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .track_symbol(None)
                .render(buffer.area, &mut buffer, &mut state);
            let expected = if i <= 1 {
                vec!["        ", "◄██    ►"]
            } else if i <= 5 {
                vec!["        ", "◄ ██   ►"]
            } else if i <= 9 {
                vec!["        ", "◄  ██  ►"]
            } else if i <= 13 {
                vec!["        ", "◄   ██ ►"]
            } else {
                vec!["        ", "◄    ██►"]
            };
            assert_buffer_eq!(buffer, Buffer::with_lines(expected.clone()));
        }
    }
}
