// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ThemesStories
{
    public static StoryPage Page { get; } = new("Tokens", "Themes", [
        new("All at once", theme => TokenExamples.Themes(theme, "All at once"), StoryLayout.Page),
        new("Ramps", theme => TokenExamples.Themes(theme, "Ramps"), StoryLayout.Page),
    ]);
}
