/// The page and theme the app was launched on, the way the React example
/// reads them from its address: `-page button -theme ember-dark` on the
/// command line, or nothing for the defaults.
import Foundation

func readLocation(_ defaults: UserDefaults = .standard) -> (page: String?, theme: ThemeChoice) {
  func argument(_ name: String) -> String? {
    #if os(Windows)
      let args = ProcessInfo.processInfo.arguments
      if let index = args.firstIndex(of: "-" + name), index + 1 < args.count {
        return args[index + 1]
      }
    #endif
    return defaults.string(forKey: name)
  }
  let page = argument("page").flatMap { $0.isEmpty ? nil : $0 }
  let id = argument("theme") ?? ""
  guard let cut = id.lastIndex(of: "-") else { return (page, defaultTheme) }
  let family = String(id[..<cut])
  let mode = String(id[id.index(after: cut)...])
  let known = themeFamilyGroups.contains { $0.families.contains { $0.value == family } }
  guard known, mode == "light" || mode == "dark" else { return (page, defaultTheme) }
  return (page, ThemeChoice(family: family, dark: mode == "dark"))
}
