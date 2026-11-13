/// The page and theme the app was launched on, the way the React example
/// reads them from its address: `-page button -theme ember-dark` on the
/// command line, or nothing for the defaults.
import Foundation
#if os(WASI)
  import JavaScriptKit
#endif

#if os(WASI)
func readLocation() -> (page: String?, theme: ThemeChoice) {
  let search = JSObject.global.location.search.string ?? ""
  let query = JSObject.global.URLSearchParams.object!.new(search)
  return resolveLocation { query.get!($0).string }
}
#else
func readLocation(_ defaults: UserDefaults = .standard) -> (page: String?, theme: ThemeChoice) {
  func argument(_ name: String) -> String? {
    #if os(Windows) || os(Linux)
      let args = ProcessInfo.processInfo.arguments
      if let index = args.firstIndex(of: "-" + name), index + 1 < args.count {
        return args[index + 1]
      }
    #endif
    return defaults.string(forKey: name)
  }
  return resolveLocation(argument)
}
#endif

private func resolveLocation(_ argument: (String) -> String?) -> (page: String?, theme: ThemeChoice) {
  let page = argument("page").flatMap { $0.isEmpty ? nil : $0 }
  let id = argument("theme") ?? ""
  guard let cut = id.lastIndex(of: "-") else { return (page, defaultTheme) }
  let family = String(id[..<cut])
  let mode = String(id[id.index(after: cut)...])
  let known = themeFamilyGroups.contains { $0.families.contains { $0.value == family } }
  guard known, mode == "light" || mode == "dark" else { return (page, defaultTheme) }
  return (page, ThemeChoice(family: family, dark: mode == "dark"))
}
