// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SkeletonStories
{
    public static StoryPage Page { get; } = new("Components", "Skeleton", [
        new("Overview", theme => ComponentExamples.Skeleton(theme, "Overview"), StoryLayout.Page),
        new("Shapes", theme => ComponentExamples.Skeleton(theme, "Shapes"), StoryLayout.Centered),
        new("Text", theme => ComponentExamples.Skeleton(theme, "Text"), StoryLayout.Centered),
        new("Lines", theme => ComponentExamples.Skeleton(theme, "Lines"), StoryLayout.Centered),
        new("A list row", theme => ComponentExamples.Skeleton(theme, "A list row"), StoryLayout.Centered),
        new("In a card", theme => ComponentExamples.Skeleton(theme, "In a card"), StoryLayout.Centered),
    ]);
}
