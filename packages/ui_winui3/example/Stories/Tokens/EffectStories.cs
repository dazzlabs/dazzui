// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class EffectStories
{
    public static StoryPage Page { get; } = new("Tokens", "Effect", [
        new("Shadow", theme => TokenExamples.Effect(theme, "Shadow"), StoryLayout.Page),
    ]);
}
