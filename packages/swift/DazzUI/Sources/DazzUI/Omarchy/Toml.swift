import Foundation

/// The flat subset of TOML Omarchy's files are written in, as Omarchy's shell
/// reads them (`shell/Commons/Color.qml`): `key = value` lines, a key under a
/// `[section]` named `section.key`, a value's quotes and a trailing `#`
/// comment dropped. Arrays, inline tables and multi-line strings are not read;
/// neither `colors.toml` nor `shell.toml` writes one.
internal func readOmarchyToml(_ source: String) -> [String: String] {
  var values: [String: String] = [:]
  var section = ""

  for raw in source.split(separator: "\n", omittingEmptySubsequences: false) {
    let line = raw.trimmingCharacters(in: .whitespacesAndNewlines)
    if line.isEmpty || line.hasPrefix("#") { continue }

    if line.hasPrefix("["), let close = line.dropFirst().firstIndex(of: "]") {
      let header = line[line.index(after: line.startIndex)..<close]
        .trimmingCharacters(in: .whitespaces)
      if !header.isEmpty {
        section = header
        continue
      }
    }

    guard let entry = parseTomlEntry(line) else { continue }
    let key = section.isEmpty ? entry.key : "\(section).\(entry.key)"
    values[key] = entry.value
  }

  return values
}

private func parseTomlEntry(_ line: String) -> (key: String, value: String)? {
  var index = line.startIndex
  while index < line.endIndex, isKeyScalar(line[index]) {
    index = line.index(after: index)
  }
  guard index > line.startIndex else { return nil }
  let key = String(line[..<index])

  while index < line.endIndex, line[index].isWhitespace {
    index = line.index(after: index)
  }
  guard index < line.endIndex, line[index] == "=" else { return nil }
  index = line.index(after: index)

  while index < line.endIndex, line[index].isWhitespace {
    index = line.index(after: index)
  }

  let rest = line[index...]
  if let quote = rest.first, quote == "\"" || quote == "'" {
    let afterQuote = rest.dropFirst()
    if let closing = afterQuote.firstIndex(of: quote) {
      return (key, String(afterQuote[..<closing]))
    }
  }

  let unquoted = rest.prefix { $0 != "#" }
  return (key, unquoted.trimmingCharacters(in: .whitespaces))
}

private func isKeyScalar(_ ch: Character) -> Bool {
  ch.isASCII && (ch.isLetter || ch.isNumber || ch == "_" || ch == "-")
}
