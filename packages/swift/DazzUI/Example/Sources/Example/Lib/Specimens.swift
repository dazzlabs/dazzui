import SwiftUI

/*
 * Where each specimen of a scene was laid out, for the conformance
 * collector (Tests/ExampleTests/ConformanceCollector.swift).
 *
 * A SwiftUI view is a value, not a tree the collector can walk, and a
 * story's JSON keeps no order among an element's props, so the walk codegen
 * addresses specimens by cannot be repeated here. Codegen writes each
 * specimen's index into its node as `$at` instead; a component drawn from
 * such a node reports its frame under that index, in the scene's own
 * coordinates. Only while collecting: the example draws nothing extra.
 */

/// The coordinate space a collected scene's frames are measured in.
let specimenSpace = "conformance.scene"

/// Every reported specimen's frame, by its index in the scene.
struct SpecimenFrames: PreferenceKey {
  static let defaultValue: [Int: CGRect] = [:]

  static func reduce(value: inout [Int: CGRect], nextValue: () -> [Int: CGRect]) {
    value.merge(nextValue()) { first, _ in first }
  }
}

private struct CollectsSpecimensKey: EnvironmentKey {
  static let defaultValue = false
}

extension EnvironmentValues {
  /// Whether components report their frames, as the collector asks.
  var collectsSpecimens: Bool {
    get { self[CollectsSpecimensKey.self] }
    set { self[CollectsSpecimensKey.self] = newValue }
  }
}

private struct SpecimenFrame: ViewModifier {
  let at: Int?
  @Environment(\.collectsSpecimens) private var collecting

  func body(content: Content) -> some View {
    if collecting, let at {
      content.background(
        GeometryReader { proxy in
          Color.clear.preference(
            key: SpecimenFrames.self,
            value: [at: proxy.frame(in: .named(specimenSpace))]
          )
        }
      )
    } else {
      content
    }
  }
}

extension View {
  /// Reports this view's frame as the specimen at `at`, while collecting.
  func specimen(_ at: Int?) -> some View {
    modifier(SpecimenFrame(at: at))
  }
}
