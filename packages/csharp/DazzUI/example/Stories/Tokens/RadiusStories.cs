// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class RadiusStories
{
    public static StoryPage Page { get; } = new("Tokens", "Radius", [
        new("Scale", theme => TokenExamples.Radius(theme, "Scale"), StoryLayout.Page),
    ]);
}
