// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SpacingStories
{
    public static StoryPage Page { get; } = new("Tokens", "Spacing", [
        new("Scale", theme => TokenExamples.Spacing(theme, "Scale"), StoryLayout.Page),
    ]);
}
