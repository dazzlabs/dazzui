// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ColorStories
{
    public static StoryPage Page { get; } = new("Tokens", "Color", [
        new("Palette", theme => TokenExamples.Color(theme, "Palette"), StoryLayout.Page),
        new("Roles", theme => TokenExamples.Color(theme, "Roles"), StoryLayout.Page),
    ]);
}
