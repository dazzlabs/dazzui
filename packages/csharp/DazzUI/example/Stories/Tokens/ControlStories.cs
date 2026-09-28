// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ControlStories
{
    public static StoryPage Page { get; } = new("Tokens", "Control", [
        new("Sizing", theme => TokenExamples.Control(theme, "Sizing"), StoryLayout.Page),
        new("Colors", theme => TokenExamples.Control(theme, "Colors"), StoryLayout.Page),
        new("Focus", theme => TokenExamples.Control(theme, "Focus"), StoryLayout.Page),
    ]);
}
