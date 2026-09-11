// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class TypographyStories
{
    public static StoryPage Page { get; } = new("Tokens", "Typography", [
        new("Scale", theme => TokenExamples.Typography(theme, "Scale"), StoryLayout.Page),
    ]);
}
