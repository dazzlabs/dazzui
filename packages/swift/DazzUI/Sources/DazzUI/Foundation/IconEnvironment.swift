import SwiftUI

private struct DazzIconSizeKey: EnvironmentKey {
  static let defaultValue: CGFloat? = nil
}
extension EnvironmentValues {
  var dazzIconSize: CGFloat? {
    get { self[DazzIconSizeKey.self] }
    set { self[DazzIconSizeKey.self] = newValue }
  }
}
