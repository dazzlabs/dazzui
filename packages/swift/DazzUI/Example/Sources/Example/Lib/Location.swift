import Foundation

/// The page and theme the app was launched on, the way the React example
/// reads them from its address: `-page button -theme ember-dark` on the
/// command line, or nothing for the defaults.
func readLocation(_ defaults: UserDefaults = .standard) -> (page: String?, theme: ThemeChoice) {
  let page = defaults.string(forKey: "page").flatMap { $0.isEmpty ? nil : $0 }
  let id = defaults.string(forKey: "theme") ?? ""
  guard let cut = id.lastIndex(of: "-") else { return (page, defaultTheme) }
  let family = String(id[..<cut])
  let mode = String(id[id.index(after: cut)...])
  let known = themeFamilyGroups.contains { $0.families.contains { $0.value == family } }
  guard known, mode == "light" || mode == "dark" else { return (page, defaultTheme) }
  return (page, ThemeChoice(family: family, dark: mode == "dark"))
}
