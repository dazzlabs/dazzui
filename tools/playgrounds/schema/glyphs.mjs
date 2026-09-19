// The Fluent glyphs a scene may name. React, Flutter and SwiftUI draw the
// glyph from their own Fluent package. Mobile platforms draw system symbols,
// so each entry names the nearest SF Symbol and Material icon: the mapping is
// mobile adapter data, never story content.
export const GLYPHS = {
  Add20Regular: { symbol: 'plus', material: 'Add' },
  ArrowDownload20Regular: {
    symbol: 'arrow.down.to.line',
    material: 'Download',
  },
  ArrowSort20Regular: { symbol: 'arrow.up.arrow.down', material: 'SwapVert' },
  Book20Regular: { symbol: 'book', material: 'Book' },
  Bookmark20Regular: { symbol: 'bookmark', material: 'Bookmark' },
  Checkmark12Filled: { symbol: 'checkmark', material: 'Check' },
  Checkmark20Regular: { symbol: 'checkmark', material: 'Check' },
  ChevronDown20Regular: {
    symbol: 'chevron.down',
    material: 'KeyboardArrowDown',
  },
  Circle20Filled: { symbol: 'circle.fill', material: 'Circle' },
  Copy20Regular: { symbol: 'doc.on.doc', material: 'ContentCopy' },
  Delete20Regular: { symbol: 'trash', material: 'Delete' },
  Document16Regular: { symbol: 'doc', material: 'Description' },
  Document20Regular: { symbol: 'doc', material: 'Description' },
  Document24Regular: { symbol: 'doc', material: 'Description' },
  Filter16Regular: {
    symbol: 'line.3.horizontal.decrease',
    material: 'FilterAlt',
  },
  Filter20Regular: {
    symbol: 'line.3.horizontal.decrease',
    material: 'FilterAlt',
  },
  Flash20Regular: { symbol: 'bolt', material: 'Bolt' },
  Folder16Regular: { symbol: 'folder', material: 'Folder' },
  Globe20Regular: { symbol: 'globe', material: 'Public' },
  Info16Regular: { symbol: 'info.circle', material: 'Info' },
  MoreHorizontal20Regular: { symbol: 'ellipsis', material: 'MoreHoriz' },
  Person16Regular: { symbol: 'person', material: 'Person' },
  Pin20Regular: { symbol: 'pin', material: 'PushPin' },
  Settings20Regular: { symbol: 'gearshape', material: 'Settings' },
  Share20Regular: { symbol: 'square.and.arrow.up', material: 'Share' },
  TextBold24Regular: { symbol: 'bold', material: 'FormatBold' },
  TextItalic24Regular: { symbol: 'italic', material: 'FormatItalic' },
  TextUnderline24Regular: { symbol: 'underline', material: 'FormatUnderlined' },
  Wrench16Regular: { symbol: 'wrench', material: 'Build' },
}
