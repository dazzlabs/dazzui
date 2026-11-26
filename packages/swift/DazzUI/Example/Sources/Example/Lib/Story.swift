import Foundation
#if canImport(CoreFoundation)
  import CoreFoundation
#endif

/// A story, as codegen compiled it out of the React example: what the list
/// shows, the one scene that stands for the component on the overview, and
/// the examples its page shows.
struct Story: Identifiable {
  /// The kit's file name for the component, and its place in the address.
  let id: String
  let name: String
  let summary: String
  let preview: Scene
  let examples: [Example]

  /// Decodes the JSON codegen wrote for the story.
  init(_ json: String) {
    guard case .object(let story) = Value.parse(json) else {
      fatalError("A generated story is not an object")
    }
    id = story["id"]?.string ?? ""
    name = story["name"]?.string ?? ""
    summary = story["summary"]?.string ?? ""
    preview = Scene(story["preview"] ?? .null)
    examples = (story["examples"]?.array ?? []).map { example in
      Example(
        title: example["title"]?.string ?? "",
        note: example["note"]?.string,
        demo: Scene(example["demo"] ?? .null)
      )
    }
  }
}

struct Example: Identifiable {
  let title: String
  let note: String?
  let demo: Scene

  var id: String { title }
}

/// A finished tree of components, and the slots it keeps live.
///
/// `content` is a node: a string is text; an object with `$el` is a
/// component with its `props` and `children`. A prop may hold a node, a
/// glyph (`$icon`), a slot's value (`$state`), a test on one (`$eq`, `$not`,
/// `$if`), or — for an event — the writes it makes (`$do`).
struct Scene {
  let state: [String: Value]
  let content: Value

  init(_ value: Value) {
    state = value["state"]?.object ?? [:]
    content = value["content"] ?? .null
  }
}

/// A JSON value, kept typed.
enum Value: Equatable, Decodable {
  case null
  case bool(Bool)
  case number(Double)
  case string(String)
  case array([Value])
  case object([String: Value])

  init(from decoder: Decoder) throws {
    let container = try decoder.singleValueContainer()
    if container.decodeNil() {
      self = .null
    } else if let bool = try? container.decode(Bool.self) {
      self = .bool(bool)
    } else if let number = try? container.decode(Double.self) {
      self = .number(number)
    } else if let string = try? container.decode(String.self) {
      self = .string(string)
    } else if let array = try? container.decode([Value].self) {
      self = .array(array)
    } else {
      self = .object(try container.decode([String: Value].self))
    }
  }

  static func parse(_ json: String) -> Value {
    do {
      let raw = try JSONSerialization.jsonObject(
        with: Data(json.utf8),
        options: .fragmentsAllowed
      )
      return fromJSON(raw)
    } catch {
      fatalError("A generated story is not JSON: \(error)")
    }
  }

  private static func fromJSON(_ raw: Any) -> Value {
    switch raw {
    case is NSNull:
      return .null
    case let string as String:
      return .string(string)
    case let array as [Any]:
      return .array(array.map(fromJSON))
    case let dict as [String: Any]:
      var result: [String: Value] = [:]
      result.reserveCapacity(dict.count)
      for (key, value) in dict {
        result[key] = fromJSON(value)
      }
      return .object(result)
    #if canImport(CoreFoundation)
      case let number as NSNumber:
        if CFGetTypeID(number) == CFBooleanGetTypeID() {
          return .bool(number.boolValue)
        }
        return .number(number.doubleValue)
    #else
      case let bool as Bool:
        return .bool(bool)
      case let number as NSNumber:
        return .number(number.doubleValue)
      case let number as Double:
        return .number(number)
      case let number as Int:
        return .number(Double(number))
    #endif
    default:
      return .null
    }
  }

  subscript(key: String) -> Value? {
    if case .object(let object) = self { return object[key] }
    return nil
  }

  var string: String? {
    if case .string(let string) = self { return string }
    return nil
  }

  var number: Double? {
    if case .number(let number) = self { return number }
    return nil
  }

  var array: [Value]? {
    if case .array(let array) = self { return array }
    return nil
  }

  var object: [String: Value]? {
    if case .object(let object) = self { return object }
    return nil
  }

  /// JavaScript's truthiness, which is what a story's tests were written in.
  var truthy: Bool {
    switch self {
    case .null: return false
    case .bool(let bool): return bool
    case .number(let number): return number != 0
    case .string(let string): return !string.isEmpty
    case .array, .object: return true
    }
  }

  /// The value as text, the way JavaScript would print it.
  var text: String {
    switch self {
    case .null: return ""
    case .bool(let bool): return String(bool)
    case .number(let number):
      return number == number.rounded() ? String(Int(number)) : String(number)
    case .string(let string): return string
    case .array(let array): return array.map(\.text).joined(separator: ",")
    case .object: return "[object Object]"
    }
  }
}
