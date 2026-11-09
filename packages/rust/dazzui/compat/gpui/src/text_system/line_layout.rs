use crate::{FontId, GlyphId, Pixels, PlatformTextSystem, Point, SharedString, Size, point, px};
use collections::FxHashMap;
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ops::Range,
    sync::Arc,
};

use super::LineWrapper;
use unicode_segmentation::UnicodeSegmentation;

/// A laid out and styled line of text
#[derive(Default, Debug)]
pub struct LineLayout {
    /// The font size for this line
    pub font_size: Pixels,
    /// The width of the line
    pub width: Pixels,
    /// The ascent of the line
    pub ascent: Pixels,
    /// The descent of the line
    pub descent: Pixels,
    /// The shaped runs that make up this line
    pub runs: Vec<ShapedRun>,
    /// Native insertion points, including ligatures and bidi boundaries, in logical byte order.
    pub carets: Vec<CaretStop>,
    /// The length of the line in utf-8 bytes
    pub len: usize,
}

/// Selection edges and native insertion sides at a logical source boundary.
#[derive(Clone, Debug)]
pub struct CaretStop {
    /// Byte index in the original string.
    pub index: usize,
    /// Leading selection edge of the following character.
    pub leading: Pixels,
    /// Trailing selection edge of the preceding character.
    pub trailing: Pixels,
    /// The paragraph's weaker embedding level chooses the ordinary caret.
    pub prefer_leading: bool,
    /// Resolved embedding level of the following character.
    pub embedding_level: u8,
    /// Native insertion edges can differ from a character's painted selection
    /// edges under kerning. Preserve them independently when normalizing ink.
    pub insertion_edges: Option<(Pixels, Pixels)>,
}

impl CaretStop {
    /// Record native edges and their adjacent bidi embedding levels.
    pub fn new(index: usize, leading: Pixels, trailing: Pixels, before: u8, after: u8) -> Self {
        Self {
            index,
            leading,
            trailing,
            prefer_leading: after < before,
            embedding_level: after,
            insertion_edges: None,
        }
    }

    /// The ordinary caret on the weaker adjacent embedding level.
    pub fn primary(&self) -> Pixels {
        let (leading, trailing) = self
            .insertion_edges
            .unwrap_or((self.leading, self.trailing));
        if self.prefer_leading {
            leading
        } else {
            trailing
        }
    }

    /// The alternate visual caret at an embedding boundary.
    pub fn secondary(&self) -> Pixels {
        let (leading, trailing) = self
            .insertion_edges
            .unwrap_or((self.leading, self.trailing));
        if self.prefer_leading {
            trailing
        } else {
            leading
        }
    }
}

/// A run of text that has been shaped .
#[derive(Debug, Clone)]
pub struct ShapedRun {
    /// The font id for this run
    pub font_id: FontId,
    /// The glyphs that make up this run
    pub glyphs: Vec<ShapedGlyph>,
}

/// A single glyph, ready to paint.
#[derive(Clone, Debug)]
pub struct ShapedGlyph {
    /// The ID for this glyph, as determined by the text system.
    pub id: GlyphId,

    /// The position of this glyph in its containing line.
    pub position: Point<Pixels>,

    /// The index of this glyph in the original text.
    pub index: usize,

    /// Whether this glyph is an emoji
    pub is_emoji: bool,
}

// Unicode's Bidi_Control property (UCD PropList.txt). These affect ordering
// but contribute no visual advance or selection ink of their own.
fn is_bidi_control(ch: char) -> bool {
    matches!(ch, '\u{061c}' | '\u{200e}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

struct InlineFragment {
    range: Range<usize>,
    left: Pixels,
    right: Pixels,
    origin: Pixels,
}

impl InlineFragment {
    fn position(&self, edge: Pixels) -> Pixels {
        self.origin + ((edge - self.left) * 64.).round() / 64.
    }
}

impl LineLayout {
    fn inline_fragments(&self) -> Vec<InlineFragment> {
        let carets = &self.carets;
        if carets.is_empty() || carets.iter().all(|stop| stop.embedding_level == 0) {
            return Vec::new();
        }
        let mut fragments = Vec::new();
        let mut start = 0;
        for end in 1..carets.len() {
            if end == carets.len() - 1
                || carets[end].embedding_level != carets[start].embedding_level
            {
                let edges = carets[start..=end]
                    .windows(2)
                    .filter_map(|pair| {
                        let (a, b) = (pair[0].leading, pair[1].trailing);
                        (a != b).then_some((a.min(b), a.max(b)))
                    })
                    .reduce(|(left, right), (a, b)| (left.min(a), right.max(b)));
                // Zero-ink control boxes can be attached to a remote edge;
                // they do not allocate any part of the inline fragment.
                let (left, right) =
                    edges.unwrap_or((carets[start].primary(), carets[start].primary()));
                fragments.push(InlineFragment {
                    range: carets[start].index..carets[end].index,
                    left,
                    right,
                    origin: px(0.),
                });
                start = end;
            }
        }
        fragments.sort_by(|a, b| a.left.partial_cmp(&b.left).unwrap());
        let mut origin = px(0.);
        for fragment in &mut fragments {
            fragment.origin = origin;
            origin += ((fragment.right - fragment.left) * 64.).ceil() / 64.;
        }
        fragments
    }

    /// Ordinary bidi insertion on the same CSS inline grid as selection.
    /// Raw native insertion coordinates remain available through x_for_index.
    pub fn inline_x_for_index(&self, index: usize) -> Option<Pixels> {
        let stop = self.carets.iter().find(|stop| stop.index == index)?;
        let fragments = self.inline_fragments();
        let fragment = fragments
            .iter()
            .find(|fragment| {
                if stop.prefer_leading {
                    fragment.range.start <= index && index < fragment.range.end
                } else {
                    fragment.range.start < index && index <= fragment.range.end
                }
            })
            .or_else(|| {
                fragments.iter().find(|fragment| {
                    fragment.left <= stop.primary() && stop.primary() <= fragment.right
                })
            })?;
        Some(fragment.position(stop.primary()))
    }

    /// The index for the character at the given x coordinate
    pub fn index_for_x(&self, x: Pixels) -> Option<usize> {
        if !self.carets.is_empty() && x < self.width {
            return self
                .carets
                .windows(2)
                .find(|pair| {
                    let (a, b) = (pair[0].leading, pair[1].trailing);
                    a.min(b) <= x && x < a.max(b)
                })
                .map(|pair| pair[0].index)
                .or(Some(0));
        }
        if x >= self.width {
            None
        } else {
            for run in self.runs.iter().rev() {
                for glyph in run.glyphs.iter().rev() {
                    if glyph.position.x <= x {
                        return Some(glyph.index);
                    }
                }
            }
            Some(0)
        }
    }

    /// closest_index_for_x returns the character boundary closest to the given x coordinate
    /// (e.g. to handle aligning up/down arrow keys)
    pub fn closest_index_for_x(&self, x: Pixels) -> usize {
        if !self.carets.is_empty() {
            return self
                .carets
                .iter()
                .min_by(|a, b| {
                    let distance = |stop: &CaretStop| (stop.primary() - x).abs();
                    distance(a).partial_cmp(&distance(b)).unwrap()
                })
                .unwrap()
                .index;
        }
        let mut prev_index = 0;
        let mut prev_x = px(0.);

        for run in self.runs.iter() {
            for glyph in run.glyphs.iter() {
                if glyph.position.x >= x {
                    if glyph.position.x - x < x - prev_x {
                        return glyph.index;
                    } else {
                        return prev_index;
                    }
                }
                prev_index = glyph.index;
                prev_x = glyph.position.x;
            }
        }

        if self.len == 1 {
            if x > self.width / 2. {
                return 1;
            } else {
                return 0;
            }
        }

        self.len
    }

    /// The x position of the character at the given index
    pub fn x_for_index(&self, index: usize) -> Pixels {
        match self.carets.binary_search_by_key(&index, |stop| stop.index) {
            Ok(at) => return self.carets[at].primary(),
            Err(after)
                if after < self.carets.len()
                    && self.carets.iter().all(|stop| stop.embedding_level == 0) =>
            {
                // A logical index inside a grapheme shares its downstream
                // insertion edge; its pre-base glyph may paint elsewhere.
                return self.carets[after].primary();
            }
            _ => {}
        }
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return glyph.position.x;
                }
            }
        }
        self.width
    }

    /// Alternate native insertion position at a bidi embedding boundary.
    pub fn secondary_x_for_index(&self, index: usize) -> Pixels {
        self.carets
            .binary_search_by_key(&index, |stop| stop.index)
            .map(|at| self.carets[at].secondary())
            .unwrap_or_else(|_| self.x_for_index(index))
    }

    /// The corresponding Font at the given index
    pub fn font_id_for_index(&self, index: usize) -> Option<FontId> {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return Some(run.font_id);
                }
            }
        }
        None
    }

    fn logical_rows(
        &self,
        text: &str,
        width: Pixels,
        max_lines: Option<usize>,
    ) -> Vec<Range<usize>> {
        let opportunities: std::collections::HashSet<_> = unicode_linebreak::linebreaks(text)
            .map(|(at, _)| at)
            .collect();
        let mut rows = Vec::new();
        let mut start = 0;
        let mut prefix = px(0.);
        let mut row_prefix = px(0.);
        let mut candidate = None;
        for (at, cluster) in text.grapheme_indices(true) {
            if opportunities.contains(&at) {
                candidate = Some((at, prefix));
            }
            let end = at + cluster.len();
            let first = self.carets.partition_point(|stop| stop.index < at);
            let last = self.carets.partition_point(|stop| stop.index < end);
            let advance = if cluster.chars().all(is_bidi_control) {
                px(0.)
            } else {
                (self.carets[last].trailing - self.carets[first].leading).abs()
            };
            if prefix + advance - row_prefix > width
                && at > start
                && !cluster.chars().all(char::is_whitespace)
                && max_lines.is_none_or(|max| rows.len() + 1 < max)
            {
                let (next, next_prefix) = candidate
                    .filter(|(at, _)| *at > start)
                    .unwrap_or((at, prefix));
                rows.push(start..next);
                start = next;
                row_prefix = next_prefix;
                candidate = None;
            }
            prefix += advance;
        }
        rows.push(start..text.len());
        rows
    }

    pub(crate) fn reordered_row(
        &self,
        bidi: &unicode_bidi::ParagraphBidiInfo<'_>,
        range: Range<usize>,
        source_glyphs: &[(FontId, &ShapedGlyph)],
    ) -> LineLayout {
        // Preserve paragraph resolution while applying L1/L2 to this row.
        // Slice the resolved data so each row does not copy the full paragraph.
        let row_bidi = unicode_bidi::ParagraphBidiInfo {
            text: &bidi.text[range.clone()],
            original_classes: bidi.original_classes[range.clone()].to_vec(),
            levels: bidi.levels[range.clone()].to_vec(),
            paragraph_level: bidi.paragraph_level,
            is_pure_ltr: bidi.is_pure_ltr,
        };
        let (levels, fragments) = row_bidi.visual_runs(0..range.len());
        let first = self.carets.partition_point(|stop| stop.index < range.start);
        let last = self.carets.partition_point(|stop| stop.index <= range.end);
        let mut carets = self.carets[first..last].to_vec();
        let mut runs: Vec<ShapedRun> = Vec::new();
        let mut x = px(0.);
        // A soft row's hanging spaces form a separate inline item. Allocate
        // the preceding text first; rounding their combined advance can move
        // the space and its caret by two layout-grid units at fractional sizes.
        let hanging = row_bidi.text.trim_end_matches([' ', '\t']).len();
        let fragments = fragments.into_iter().flat_map(|fragment| {
            if fragment.start < hanging && hanging < fragment.end {
                vec![fragment.start..hanging, hanging..fragment.end]
            } else {
                vec![fragment]
            }
        });
        for local in fragments {
            let fragment = local.start + range.start..local.end + range.start;
            let first_char = self
                .carets
                .partition_point(|stop| stop.index < fragment.start);
            let last_char = self
                .carets
                .partition_point(|stop| stop.index < fragment.end);
            let pairs = &self.carets[first_char..=last_char];
            let left = pairs
                .windows(2)
                .filter(|p| !is_bidi_control(bidi.text[p[0].index..].chars().next().unwrap()))
                .map(|p| p[0].leading.min(p[1].trailing))
                .min()
                .unwrap_or(px(0.));
            let right = pairs
                .windows(2)
                .filter(|p| !is_bidi_control(bidi.text[p[0].index..].chars().next().unwrap()))
                .map(|p| p[0].leading.max(p[1].trailing))
                .max()
                .unwrap_or(left);
            let changed_direction = levels[fragment.start - range.start].is_rtl()
                != (self.carets[first_char].embedding_level % 2 == 1);
            let transform = |edge: Pixels| x + ((edge - left) * 64.).round() / 64.;
            for index in first_char..last_char {
                if is_bidi_control(
                    bidi.text[self.carets[index].index..]
                        .chars()
                        .next()
                        .unwrap(),
                ) {
                    continue;
                }
                let a = self.carets[index].leading;
                let b = self.carets[index + 1].trailing;
                let edge = |value| {
                    if changed_direction {
                        x + ((right - value) * 64.).round() / 64.
                    } else {
                        transform(value)
                    }
                };
                carets[index - first].leading = edge(a);
                carets[index + 1 - first].trailing = edge(b);
                let native_a = self.carets[index]
                    .insertion_edges
                    .map_or(a, |edges| edges.0);
                let native_b = self.carets[index + 1]
                    .insertion_edges
                    .map_or(b, |edges| edges.1);
                carets[index - first]
                    .insertion_edges
                    .get_or_insert((px(0.), px(0.)))
                    .0 = edge(native_a);
                carets[index + 1 - first]
                    .insertion_edges
                    .get_or_insert((px(0.), px(0.)))
                    .1 = edge(native_b);
            }
            let first_glyph =
                source_glyphs.partition_point(|(_, glyph)| glyph.index < fragment.start);
            let last_glyph = source_glyphs.partition_point(|(_, glyph)| glyph.index < fragment.end);
            let mut glyphs: Vec<_> = source_glyphs[first_glyph..last_glyph]
                .iter()
                .filter(|(_, glyph)| {
                    !is_bidi_control(bidi.text[glyph.index..].chars().next().unwrap())
                })
                .map(|(font, glyph)| (*font, (*glyph).clone()))
                .collect();
            for (_, glyph) in &mut glyphs {
                if changed_direction {
                    let at = self.carets.partition_point(|stop| stop.index < glyph.index);
                    let advance = (self.carets[at].leading - self.carets[at + 1].trailing).abs();
                    glyph.position.x = x + right - glyph.position.x - advance;
                } else {
                    glyph.position.x += x - left;
                }
                glyph.index -= range.start;
            }
            glyphs.sort_by(|a, b| a.1.position.x.partial_cmp(&b.1.position.x).unwrap());
            for (font_id, glyph) in glyphs {
                if let Some(run) = runs.last_mut().filter(|run| run.font_id == font_id) {
                    run.glyphs.push(glyph);
                } else {
                    runs.push(ShapedRun {
                        font_id,
                        glyphs: vec![glyph],
                    });
                }
            }
            x += ((right - left) * 64.).ceil() / 64.;
        }
        carets.last_mut().unwrap().leading = x;
        for index in (0..carets.len() - 1).rev() {
            if is_bidi_control(bidi.text[carets[index].index..].chars().next().unwrap()) {
                let edge = carets[index + 1].leading;
                carets[index].leading = edge;
                carets[index + 1].trailing = edge;
                carets[index].insertion_edges = None;
                carets[index + 1].insertion_edges = None;
            }
        }
        let mut before = 0;
        for stop in &mut carets {
            let after = if stop.index < range.end {
                levels[stop.index - range.start].number()
            } else {
                0
            };
            stop.prefer_leading = after < before;
            stop.embedding_level = after;
            stop.index -= range.start;
            before = after;
        }
        carets.first_mut().unwrap().trailing = px(0.);
        carets.last_mut().unwrap().leading = x;
        if let Some(edges) = &mut carets.first_mut().unwrap().insertion_edges {
            edges.1 = px(0.);
        }
        let end_trailing = carets.last().unwrap().trailing;
        if let Some(edges) = &mut carets.last_mut().unwrap().insertion_edges {
            edges.0 = x;
            edges.1 = end_trailing;
        }
        LineLayout {
            font_size: self.font_size,
            width: x,
            ascent: self.ascent,
            descent: self.descent,
            runs,
            carets,
            len: range.len(),
        }
    }

    fn compute_wrap_boundaries(
        &self,
        text: &str,
        wrap_width: Pixels,
        max_lines: Option<usize>,
    ) -> SmallVec<[WrapBoundary; 1]> {
        let mut boundaries = SmallVec::new();
        let mut first_non_whitespace_ix = None;
        let mut last_candidate_ix = None;
        let mut last_candidate_x = px(0.);
        let mut last_boundary = WrapBoundary {
            run_ix: 0,
            glyph_ix: 0,
        };
        let mut last_boundary_x = px(0.);
        let mut prev_ch = '\0';
        let mut glyphs = self
            .runs
            .iter()
            .enumerate()
            .flat_map(move |(run_ix, run)| {
                run.glyphs.iter().enumerate().map(move |(glyph_ix, glyph)| {
                    let character = text[glyph.index..].chars().next().unwrap();
                    (
                        WrapBoundary { run_ix, glyph_ix },
                        character,
                        glyph.position.x,
                    )
                })
            })
            .peekable();

        while let Some((boundary, ch, x)) = glyphs.next() {
            if ch == '\n' {
                continue;
            }

            // Here is very similar to `LineWrapper::wrap_line` to determine text wrapping,
            // but there are some differences, so we have to duplicate the code here.
            if LineWrapper::is_word_char(ch) {
                if prev_ch == ' ' && ch != ' ' && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = Some(boundary);
                    last_candidate_x = x;
                }
            } else {
                if ch != ' ' && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = Some(boundary);
                    last_candidate_x = x;
                }
            }

            if ch != ' ' && first_non_whitespace_ix.is_none() {
                first_non_whitespace_ix = Some(boundary);
            }

            let next_x = glyphs.peek().map_or(self.width, |(_, _, x)| *x);
            let width = next_x - last_boundary_x;

            if width > wrap_width && boundary > last_boundary {
                // When used line_clamp, we should limit the number of lines.
                if let Some(max_lines) = max_lines
                    && boundaries.len() >= max_lines - 1
                {
                    break;
                }

                if let Some(last_candidate_ix) = last_candidate_ix.take() {
                    last_boundary = last_candidate_ix;
                    last_boundary_x = last_candidate_x;
                } else {
                    last_boundary = boundary;
                    last_boundary_x = x;
                }
                boundaries.push(last_boundary);
            }
            prev_ch = ch;
        }

        boundaries
    }
}

/// A line of text that has been wrapped to fit a given width
#[derive(Default, Debug)]
pub struct WrappedLineLayout {
    /// The line layout, pre-wrapping.
    pub unwrapped_layout: Arc<LineLayout>,

    /// The boundaries at which the line was wrapped
    pub wrap_boundaries: SmallVec<[WrapBoundary; 1]>,

    /// Independently reordered visual rows for a bidirectional paragraph.
    pub row_layouts: Vec<WrappedRowLayout>,

    /// The width of the line, if it was wrapped
    pub wrap_width: Option<Pixels>,
}

/// A visual row retaining its range in the original logical paragraph.
#[derive(Debug)]
pub struct WrappedRowLayout {
    /// UTF-8 range within the original paragraph.
    pub range: Range<usize>,
    /// Native glyphs reordered within this row, with local source indices.
    pub layout: Arc<LineLayout>,
}

/// A boundary at which a line was wrapped
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrapBoundary {
    /// The index in the run just before the line was wrapped
    pub run_ix: usize,
    /// The index of the glyph just before the line was wrapped
    pub glyph_ix: usize,
}

impl WrappedLineLayout {
    /// The length of the underlying text, in utf8 bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.unwrapped_layout.len
    }

    /// The width of this line, in pixels, whether or not it was wrapped.
    pub fn width(&self) -> Pixels {
        self.wrap_width
            .unwrap_or(Pixels::MAX)
            .min(self.unwrapped_layout.width)
    }

    /// The size of the whole wrapped text, for the given line_height.
    /// can span multiple lines if there are multiple wrap boundaries.
    pub fn size(&self, line_height: Pixels) -> Size<Pixels> {
        Size {
            width: self.width(),
            height: line_height * (self.wrap_boundaries.len() + 1),
        }
    }

    /// The ascent of a line in this layout
    pub fn ascent(&self) -> Pixels {
        self.unwrapped_layout.ascent
    }

    /// The descent of a line in this layout
    pub fn descent(&self) -> Pixels {
        self.unwrapped_layout.descent
    }

    /// The wrap boundaries in this layout
    pub fn wrap_boundaries(&self) -> &[WrapBoundary] {
        &self.wrap_boundaries
    }

    /// The logical source start of the row following a visual wrap.
    pub fn wrap_source_index(&self, row: usize) -> usize {
        if !self.row_layouts.is_empty() {
            self.row_layouts[row + 1].range.start
        } else {
            let boundary = self.wrap_boundaries[row];
            self.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
        }
    }

    fn row_origin_x(&self, row: usize) -> Pixels {
        if row == 0 {
            px(0.)
        } else {
            let boundary = self.wrap_boundaries[row - 1];
            self.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix]
                .position
                .x
        }
    }

    /// The font size of this layout
    pub fn font_size(&self) -> Pixels {
        self.unwrapped_layout.font_size
    }

    /// The runs in this layout, sans wrapping
    pub fn runs(&self) -> &[ShapedRun] {
        &self.unwrapped_layout.runs
    }

    /// The index corresponding to a given position in this layout for the given line height.
    ///
    /// See also [`Self::closest_index_for_position`].
    pub fn index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
    ) -> Result<usize, usize> {
        self._index_for_position(position, line_height, false)
    }

    /// The closest index to a given position in this layout for the given line height.
    ///
    /// Closest means the character boundary closest to the given position.
    ///
    /// See also [`LineLayout::closest_index_for_x`].
    pub fn closest_index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
    ) -> Result<usize, usize> {
        self._index_for_position(position, line_height, true)
    }

    fn _index_for_position(
        &self,
        mut position: Point<Pixels>,
        line_height: Pixels,
        closest: bool,
    ) -> Result<usize, usize> {
        let wrapped_line_ix = (position.y / line_height) as usize;
        if !self.row_layouts.is_empty() {
            let Some(row) = self.row_layouts.get(wrapped_line_ix) else {
                return Err(self.len());
            };
            if position.x < px(0.) {
                return Err(row.range.start);
            }
            if position.x >= row.layout.width {
                return Err(row.range.end);
            }
            return Ok(row.range.start
                + if closest {
                    row.layout.closest_index_for_x(position.x)
                } else {
                    row.layout.index_for_x(position.x).unwrap_or(row.layout.len)
                });
        }

        let wrapped_line_start_index;
        let wrapped_line_start_x;
        if wrapped_line_ix > 0 {
            let Some(line_start_boundary) = self.wrap_boundaries.get(wrapped_line_ix - 1) else {
                return Err(0);
            };
            let run = &self.unwrapped_layout.runs[line_start_boundary.run_ix];
            let glyph = &run.glyphs[line_start_boundary.glyph_ix];
            wrapped_line_start_index = self.wrap_source_index(wrapped_line_ix - 1);
            wrapped_line_start_x = glyph.position.x;
        } else {
            wrapped_line_start_index = 0;
            wrapped_line_start_x = Pixels::ZERO;
        };

        let wrapped_line_end_index;
        let wrapped_line_end_x;
        if wrapped_line_ix < self.wrap_boundaries.len() {
            let next_wrap_boundary_ix = wrapped_line_ix;
            let next_wrap_boundary = self.wrap_boundaries[next_wrap_boundary_ix];
            let run = &self.unwrapped_layout.runs[next_wrap_boundary.run_ix];
            let glyph = &run.glyphs[next_wrap_boundary.glyph_ix];
            wrapped_line_end_index = self.wrap_source_index(wrapped_line_ix);
            wrapped_line_end_x = glyph.position.x;
        } else {
            wrapped_line_end_index = self.unwrapped_layout.len;
            wrapped_line_end_x = self.unwrapped_layout.width;
        };

        let mut position_in_unwrapped_line = position;
        position_in_unwrapped_line.x += wrapped_line_start_x;
        if position_in_unwrapped_line.x < wrapped_line_start_x {
            Err(wrapped_line_start_index)
        } else if position_in_unwrapped_line.x >= wrapped_line_end_x {
            Err(wrapped_line_end_index)
        } else {
            if closest {
                Ok(self
                    .unwrapped_layout
                    .closest_index_for_x(position_in_unwrapped_line.x))
            } else {
                Ok(self
                    .unwrapped_layout
                    .index_for_x(position_in_unwrapped_line.x)
                    .unwrap())
            }
        }
    }

    /// Returns the pixel position for the given byte index.
    pub fn position_for_index(&self, index: usize, line_height: Pixels) -> Option<Point<Pixels>> {
        self.caret_position(index, line_height)
    }

    /// Selection fragments from the native leading/trailing character edges.
    /// Each soft-wrapped row has its own visual origin and logical source
    /// range. Paragraphs whose visual wrapping crosses source rows still need
    /// row-level reordering shared with glyph painting.
    pub fn selection_rects(
        &self,
        range: Range<usize>,
        line_height: Pixels,
    ) -> Option<Vec<crate::Bounds<Pixels>>> {
        if !self.row_layouts.is_empty() {
            let mut rects = Vec::new();
            for (index, row) in self.row_layouts.iter().enumerate() {
                let start = range.start.max(row.range.start);
                let end = range.end.min(row.range.end);
                if start >= end {
                    continue;
                }
                let line = WrappedLineLayout {
                    unwrapped_layout: row.layout.clone(),
                    ..Default::default()
                };
                rects.extend(
                    // Row layout already allocated its inline fragments.
                    line.selection_rects_with_fragments(
                        start - row.range.start..end - row.range.start,
                        line_height,
                        &[],
                    )?
                    .into_iter()
                    .map(|mut rect| {
                        rect.origin.y += line_height * index;
                        rect
                    }),
                );
            }
            return Some(rects);
        }
        let mut inline_fragments = self.unwrapped_layout.inline_fragments();
        inline_fragments.sort_unstable_by_key(|fragment| fragment.range.start);
        self.selection_rects_with_fragments(range, line_height, &inline_fragments)
    }

    fn selection_rects_with_fragments(
        &self,
        range: Range<usize>,
        line_height: Pixels,
        inline_fragments: &[InlineFragment],
    ) -> Option<Vec<crate::Bounds<Pixels>>> {
        let carets = &self.unwrapped_layout.carets;
        if carets.is_empty() {
            return None;
        }
        let mut rects = Vec::new();
        let mut start = 0;
        let mut origin_x = px(0.);
        for (row, end) in (0..self.wrap_boundaries.len())
            .map(|row| self.wrap_source_index(row))
            .chain([self.len()])
            .enumerate()
        {
            if end < start {
                return None;
            }
            let first = carets.partition_point(|stop| stop.index < start);
            let last = carets.partition_point(|stop| stop.index <= end);
            let mut spans: Vec<_> = carets[first..last]
                .windows(2)
                .filter(|pair| pair[0].index < range.end && pair[1].index > range.start)
                .map(|pair| {
                    let after = inline_fragments
                        .partition_point(|fragment| fragment.range.start <= pair[0].index);
                    let fragment = after
                        .checked_sub(1)
                        .and_then(|at| inline_fragments.get(at))
                        .filter(|fragment| fragment.range.contains(&pair[0].index));
                    let edge = |x| fragment.map_or(x, |fragment| fragment.position(x));
                    (
                        edge(pair[0].leading.min(pair[1].trailing)) - origin_x,
                        edge(pair[0].leading.max(pair[1].trailing)) - origin_x,
                    )
                })
                .filter(|(left, right)| left < right)
                .collect();
            spans.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let mut merged: Vec<(Pixels, Pixels)> = Vec::new();
            for (left, right) in spans {
                if let Some(last) = merged.last_mut().filter(|last| left <= last.1 + px(0.001)) {
                    last.1 = last.1.max(right);
                } else {
                    merged.push((left, right));
                }
            }
            let top = line_height * row;
            rects.extend(merged.into_iter().map(|(left, right)| {
                crate::Bounds::from_corners(point(left, top), point(right, top + line_height))
            }));
            start = end;
            if row < self.wrap_boundaries.len() {
                origin_x = self.row_origin_x(row + 1);
            }
        }
        Some(rects)
    }

    fn caret_position(&self, index: usize, line_height: Pixels) -> Option<Point<Pixels>> {
        if !self.row_layouts.is_empty() {
            return self
                .row_layouts
                .iter()
                .enumerate()
                .find(|(_, row)| row.range.contains(&index) || row.range.end == index)
                .map(|(row_index, row)| {
                    point(
                        row.layout.x_for_index(index - row.range.start),
                        line_height * row_index,
                    )
                });
        }
        let mut line_start_ix = 0;
        let mut line_end_indices = (0..self.wrap_boundaries.len())
            .map(|row| self.wrap_source_index(row))
            .chain([self.len()])
            .enumerate();
        for (ix, line_end_ix) in line_end_indices {
            let line_y = ix as f32 * line_height;
            if index < line_start_ix {
                break;
            } else if index > line_end_ix {
                line_start_ix = line_end_ix;
                continue;
            } else {
                let line_start_x = self.row_origin_x(ix);
                let x = self.unwrapped_layout.x_for_index(index) - line_start_x;
                return Some(point(x, line_y));
            }
        }

        None
    }
}

pub(crate) struct LineLayoutCache {
    previous_frame: Mutex<FrameCache>,
    current_frame: RwLock<FrameCache>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
}

struct RowAtom {
    range: Range<usize>,
    width: Pixels,
    whitespace: bool,
    graphemes: usize,
}

fn row_fit_width(text: &str, layout: &LineLayout) -> Pixels {
    let end = text.trim_end_matches([' ', '\t']).len();
    layout
        .carets
        .windows(2)
        .filter(|pair| {
            pair[0].index < end && !is_bidi_control(text[pair[0].index..].chars().next().unwrap())
        })
        .map(|pair| pair[0].leading.max(pair[1].trailing))
        .max()
        .unwrap_or(px(0.))
}

fn row_atoms(
    text: &str,
    offset: usize,
    layout: &LineLayout,
) -> std::collections::VecDeque<RowAtom> {
    let graphemes = text
        .grapheme_indices(true)
        .map(|(at, _)| at)
        .chain([text.len()])
        .collect::<Vec<_>>();
    let mut boundaries = std::collections::BTreeSet::from([0, text.len()]);
    for glyph in layout.runs.iter().flat_map(|run| &run.glyphs) {
        let at = graphemes
            .partition_point(|at| *at <= glyph.index)
            .saturating_sub(1);
        boundaries.insert(graphemes[at]);
    }
    for (at, ch) in text.char_indices().filter(|(_, ch)| is_bidi_control(*ch)) {
        boundaries.extend([at, at + ch.len_utf8()]);
    }
    boundaries
        .into_iter()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|pair| {
            let range = pair[0]..pair[1];
            let first = layout
                .carets
                .partition_point(|stop| stop.index < range.start);
            let last = layout.carets.partition_point(|stop| stop.index < range.end);
            let whitespace = text[range.clone()]
                .chars()
                .all(|ch| ch == ' ' || ch == '\t');
            let width = if text[range.clone()].chars().all(is_bidi_control) {
                px(0.)
            } else {
                (layout.carets[last].trailing - layout.carets[first].leading).abs()
            };
            RowAtom {
                range: range.start + offset..range.end + offset,
                width,
                whitespace,
                graphemes: text[range].graphemes(true).count(),
            }
        })
        .collect()
}

#[derive(Default)]
struct FrameCache {
    lines: FxHashMap<Arc<CacheKey>, Arc<LineLayout>>,
    wrapped_lines: FxHashMap<Arc<CacheKey>, Arc<WrappedLineLayout>>,
    used_lines: Vec<Arc<CacheKey>>,
    used_wrapped_lines: Vec<Arc<CacheKey>>,
}

#[derive(Clone, Default)]
pub(crate) struct LineLayoutIndex {
    lines_index: usize,
    wrapped_lines_index: usize,
}

impl LineLayoutCache {
    fn contextual_rows(
        &self,
        bidi: &unicode_bidi::ParagraphBidiInfo<'_>,
        font_size: Pixels,
        runs: &[FontRun],
        width: Pixels,
        max_lines: Option<usize>,
    ) -> Option<Vec<WrappedRowLayout>> {
        let shape = |range| {
            self.platform_text_system
                .layout_fragment(bidi, range, font_size, runs)
        };
        let full = shape(0..bidi.text.len())?;
        let mut atoms = row_atoms(bidi.text, 0, &full);
        let opportunities = unicode_linebreak::linebreaks(bidi.text)
            .map(|(at, _)| at)
            .collect::<std::collections::HashSet<_>>();
        let mut rows = Vec::new();
        while let Some(first) = atoms.front() {
            let start = first.range.start;
            let last_row = max_lines.is_some_and(|max| rows.len() + 1 >= max);
            let mut count = 0;
            let mut advance = px(0.);
            let mut legal = None;
            for atom in &atoms {
                if !last_row && advance + atom.width > width && !atom.whitespace {
                    break;
                }
                count += 1;
                advance += atom.width;
                if opportunities.contains(&atom.range.end) {
                    legal = Some(count);
                }
            }
            if count < atoms.len()
                && let Some(legal) = legal
            {
                count = legal;
            }
            // An unsafe glyph cluster cannot be cut using interpolated caret
            // advances. Break its first grapheme, then reshape the remainder
            // of that cluster with context before considering another break.
            let mut force_grapheme = count == 0 && first.graphemes > 1;
            count = count.max(1);
            if !force_grapheme {
                while count < atoms.len() && atoms[count].whitespace {
                    count += 1;
                }
            }
            let layout = loop {
                let end = if force_grapheme {
                    let (at, grapheme) = bidi.text[first.range.clone()]
                        .grapheme_indices(true)
                        .find(|(_, cluster)| !cluster.chars().all(is_bidi_control))?;
                    start + at + grapheme.len()
                } else {
                    atoms[count - 1].range.end
                };
                let layout = shape(start..end)?;
                if last_row
                    || row_fit_width(&bidi.text[start..end], &layout) <= width
                    || force_grapheme
                    || first.graphemes == 1
                        && atoms.iter().take(count).skip(1).all(|atom| atom.whitespace)
                {
                    break (end, layout);
                }
                if count == 1 {
                    force_grapheme = true;
                } else {
                    count = (1..count)
                        .rev()
                        .find(|count| opportunities.contains(&atoms[count - 1].range.end))
                        .unwrap_or(count - 1);
                }
            };
            let (end, layout) = layout;
            if force_grapheme {
                let cluster = atoms.pop_front()?;
                if end < cluster.range.end {
                    let remainder = shape(end..cluster.range.end)?;
                    for atom in row_atoms(&bidi.text[end..cluster.range.end], end, &remainder)
                        .into_iter()
                        .rev()
                    {
                        atoms.push_front(atom);
                    }
                }
            } else {
                for _ in 0..count {
                    atoms.pop_front();
                }
            }
            rows.push(WrappedRowLayout {
                range: start..end,
                layout: Arc::new(layout),
            });
        }
        Some(rows)
    }

    pub fn new(platform_text_system: Arc<dyn PlatformTextSystem>) -> Self {
        Self {
            previous_frame: Mutex::default(),
            current_frame: RwLock::default(),
            platform_text_system,
        }
    }

    pub fn layout_index(&self) -> LineLayoutIndex {
        let frame = self.current_frame.read();
        LineLayoutIndex {
            lines_index: frame.used_lines.len(),
            wrapped_lines_index: frame.used_wrapped_lines.len(),
        }
    }

    pub fn reuse_layouts(&self, range: Range<LineLayoutIndex>) {
        let mut previous_frame = &mut *self.previous_frame.lock();
        let mut current_frame = &mut *self.current_frame.write();

        for key in &previous_frame.used_lines[range.start.lines_index..range.end.lines_index] {
            if let Some((key, line)) = previous_frame.lines.remove_entry(key) {
                current_frame.lines.insert(key, line);
            }
            current_frame.used_lines.push(key.clone());
        }

        for key in &previous_frame.used_wrapped_lines
            [range.start.wrapped_lines_index..range.end.wrapped_lines_index]
        {
            if let Some((key, line)) = previous_frame.wrapped_lines.remove_entry(key) {
                current_frame.wrapped_lines.insert(key, line);
            }
            current_frame.used_wrapped_lines.push(key.clone());
        }
    }

    pub fn truncate_layouts(&self, index: LineLayoutIndex) {
        let mut current_frame = &mut *self.current_frame.write();
        current_frame.used_lines.truncate(index.lines_index);
        current_frame
            .used_wrapped_lines
            .truncate(index.wrapped_lines_index);
    }

    pub fn finish_frame(&self) {
        let mut prev_frame = self.previous_frame.lock();
        let mut curr_frame = self.current_frame.write();
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);
        curr_frame.lines.clear();
        curr_frame.wrapped_lines.clear();
        curr_frame.used_lines.clear();
        curr_frame.used_wrapped_lines.clear();
    }

    pub fn layout_wrapped_line<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
        max_lines: Option<usize>,
    ) -> Arc<WrappedLineLayout>
    where
        Text: AsRef<str>,
        SharedString: From<Text>,
    {
        let key = &CacheKeyRef {
            text: text.as_ref(),
            font_size,
            runs,
            wrap_width,
            force_width: None,
        } as &dyn AsCacheKeyRef;

        let current_frame = self.current_frame.upgradable_read();
        if let Some(layout) = current_frame.wrapped_lines.get(key) {
            return layout.clone();
        }

        let previous_frame_entry = self.previous_frame.lock().wrapped_lines.remove_entry(key);
        if let Some((key, layout)) = previous_frame_entry {
            let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
            current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            current_frame.used_wrapped_lines.push(key);
            layout
        } else {
            drop(current_frame);
            let text = SharedString::from(text);
            let unwrapped_layout = self.layout_line::<&SharedString>(&text, font_size, runs, None);
            let wrap_boundaries = if let Some(wrap_width) = wrap_width {
                unwrapped_layout.compute_wrap_boundaries(text.as_ref(), wrap_width, max_lines)
            } else {
                SmallVec::new()
            };
            let mut row_layouts = Vec::new();
            let mut wrap_boundaries = wrap_boundaries;
            let bidirectional = unwrapped_layout
                .carets
                .iter()
                .any(|stop| stop.embedding_level != 0);
            if let Some(width) = wrap_width.filter(|_| !unwrapped_layout.carets.is_empty()) {
                let ranges = unwrapped_layout.logical_rows(&text, width, max_lines);
                let multiple_rows = ranges.len() > 1;
                {
                    let bidi = unicode_bidi::ParagraphBidiInfo::new(
                        &text,
                        Some(unicode_bidi::Level::ltr()),
                    );
                    let contextual = self.contextual_rows(&bidi, font_size, runs, width, max_lines);
                    if let Some(contextual) = contextual {
                        row_layouts = contextual;
                    } else if bidirectional && multiple_rows {
                        let mut source_glyphs: Vec<_> = unwrapped_layout
                            .runs
                            .iter()
                            .flat_map(|run| run.glyphs.iter().map(|glyph| (run.font_id, glyph)))
                            .collect();
                        source_glyphs.sort_by_key(|(_, glyph)| glyph.index);
                        row_layouts = ranges
                            .into_iter()
                            .map(|range| WrappedRowLayout {
                                layout: Arc::new(unwrapped_layout.reordered_row(
                                    &bidi,
                                    range.clone(),
                                    &source_glyphs,
                                )),
                                range,
                            })
                            .collect();
                    }
                    if !row_layouts.is_empty() {
                        // The source boundary API is authoritative for these rows;
                        // retain glyph markers for the legacy boundary count API.
                        wrap_boundaries = row_layouts
                            .iter()
                            .skip(1)
                            .map(|row| {
                                unwrapped_layout
                                    .runs
                                    .iter()
                                    .enumerate()
                                    .flat_map(|(run_ix, run)| {
                                        run.glyphs.iter().enumerate().map(
                                            move |(glyph_ix, glyph)| {
                                                (glyph.index, WrapBoundary { run_ix, glyph_ix })
                                            },
                                        )
                                    })
                                    .filter(|(index, _)| *index >= row.range.start)
                                    .min_by_key(|(index, _)| *index)
                                    .map_or(
                                        WrapBoundary {
                                            run_ix: 0,
                                            glyph_ix: 0,
                                        },
                                        |(_, marker)| marker,
                                    )
                            })
                            .collect();
                    }
                }
                if row_layouts.is_empty() && bidirectional && !multiple_rows {
                    wrap_boundaries.clear();
                }
            }
            let layout = Arc::new(WrappedLineLayout {
                unwrapped_layout,
                wrap_boundaries,
                row_layouts,
                wrap_width,
            });
            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width,
                force_width: None,
            });

            let mut current_frame = self.current_frame.write();
            current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            current_frame.used_wrapped_lines.push(key);

            layout
        }
    }

    pub fn layout_line<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
    ) -> Arc<LineLayout>
    where
        Text: AsRef<str>,
        SharedString: From<Text>,
    {
        let key = &CacheKeyRef {
            text: text.as_ref(),
            font_size,
            runs,
            wrap_width: None,
            force_width,
        } as &dyn AsCacheKeyRef;

        let current_frame = self.current_frame.upgradable_read();
        if let Some(layout) = current_frame.lines.get(key) {
            return layout.clone();
        }

        let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
        if let Some((key, layout)) = self.previous_frame.lock().lines.remove_entry(key) {
            current_frame.lines.insert(key.clone(), layout.clone());
            current_frame.used_lines.push(key);
            layout
        } else {
            let text = SharedString::from(text);
            let mut layout = self
                .platform_text_system
                .layout_line(&text, font_size, runs);

            if let Some(force_width) = force_width {
                // Forced cell widths replace the native glyph geometry.
                layout.carets.clear();
                let mut glyph_pos = 0;
                for run in layout.runs.iter_mut() {
                    for glyph in run.glyphs.iter_mut() {
                        if (glyph.position.x - glyph_pos * force_width).abs() > px(1.) {
                            glyph.position.x = glyph_pos * force_width;
                        }
                        glyph_pos += 1;
                    }
                }
            }

            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width: None,
                force_width,
            });
            let layout = Arc::new(layout);
            current_frame.lines.insert(key.clone(), layout.clone());
            current_frame.used_lines.push(key);
            layout
        }
    }
}

/// A run of text with a single font.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontRun {
    pub(crate) len: usize,
    pub(crate) font_id: FontId,
}

trait AsCacheKeyRef {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_>;
}

#[derive(Clone, Debug, Eq)]
struct CacheKey {
    text: SharedString,
    font_size: Pixels,
    runs: SmallVec<[FontRun; 1]>,
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [FontRun],
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

impl PartialEq for dyn AsCacheKeyRef + '_ {
    fn eq(&self, other: &dyn AsCacheKeyRef) -> bool {
        self.as_cache_key_ref() == other.as_cache_key_ref()
    }
}

impl Eq for dyn AsCacheKeyRef + '_ {}

impl Hash for dyn AsCacheKeyRef + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state)
    }
}

impl AsCacheKeyRef for CacheKey {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_> {
        CacheKeyRef {
            text: &self.text,
            font_size: self.font_size,
            runs: self.runs.as_slice(),
            wrap_width: self.wrap_width,
            force_width: self.force_width,
        }
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_cache_key_ref().eq(&other.as_cache_key_ref())
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state);
    }
}

impl<'a> Borrow<dyn AsCacheKeyRef + 'a> for Arc<CacheKey> {
    fn borrow(&self) -> &(dyn AsCacheKeyRef + 'a) {
        self.as_ref() as &dyn AsCacheKeyRef
    }
}

impl AsCacheKeyRef for CacheKeyRef<'_> {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_> {
        *self
    }
}

#[cfg(test)]
mod caret_tests {
    use super::*;

    fn mixed() -> LineLayout {
        // Native edges for LTR "ab ", RTL "אב", then LTR " cd".
        let edges = [
            (0, 0., 0., 0, 0),
            (1, 10., 10., 0, 0),
            (2, 20., 20., 0, 0),
            (3, 50., 30., 0, 1),
            (5, 40., 40., 1, 1),
            (7, 50., 30., 1, 0),
            (8, 60., 60., 0, 0),
            (9, 70., 70., 0, 0),
            (10, 80., 80., 0, 0),
        ];
        LineLayout {
            width: px(80.),
            len: 10,
            carets: edges
                .into_iter()
                .map(|(at, a, b, before, after)| CaretStop::new(at, px(a), px(b), before, after))
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn clicks_choose_the_paragraph_caret_at_bidi_boundaries() {
        let line = mixed();
        assert_eq!(line.x_for_index(3), px(30.));
        assert_eq!(line.secondary_x_for_index(3), px(50.));
        assert_eq!(line.x_for_index(7), px(50.));
        assert_eq!(line.secondary_x_for_index(7), px(30.));
        for (x, index) in [(29., 3), (31., 3), (39., 5), (41., 5), (49., 7), (51., 7)] {
            assert_eq!(line.closest_index_for_x(px(x)), index);
        }
    }

    #[test]
    fn logical_selections_keep_disjoint_visual_fragments() {
        let line = WrappedLineLayout {
            unwrapped_layout: Arc::new(mixed()),
            ..Default::default()
        };
        let rects = line.selection_rects(0..5, px(15.)).unwrap();
        assert_eq!(rects.len(), 2);
        assert_eq!((rects[0].left(), rects[0].right()), (px(0.), px(30.)));
        assert_eq!((rects[1].left(), rects[1].right()), (px(40.), px(50.)));
        let rtl = line.selection_rects(3..7, px(15.)).unwrap();
        assert_eq!(rtl.len(), 1);
        assert_eq!((rtl[0].left(), rtl[0].right()), (px(30.), px(50.)));
        assert_eq!(line.position_for_index(5, px(15.)).unwrap().x, px(40.));
    }
}
