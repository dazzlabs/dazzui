// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class PreviewCardStories
{
    public static StoryPage Page { get; } = new("Components", "PreviewCard", [
        new("Overview", theme => ComponentExamples.PreviewCard(theme, "Overview"), StoryLayout.Page),
        new("On a link", theme => ComponentExamples.PreviewCard(theme, "On a link"), StoryLayout.Centered),
        new("A repository", theme => ComponentExamples.PreviewCard(theme, "A repository"), StoryLayout.Centered),
        new("Sides", theme => ComponentExamples.PreviewCard(theme, "Sides"), StoryLayout.Centered),
        new("Instant", theme => ComponentExamples.PreviewCard(theme, "Instant"), StoryLayout.Centered),
    ]);
}
