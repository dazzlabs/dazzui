// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ProgressStories
{
    public static StoryPage Page { get; } = new("Components", "Progress", [
        new("Overview", theme => ComponentExamples.Progress(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.Progress(theme, "Default"), StoryLayout.Centered),
        new("Values", theme => ComponentExamples.Progress(theme, "Values"), StoryLayout.Centered),
        new("Indeterminate", theme => ComponentExamples.Progress(theme, "Indeterminate"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Progress(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Progress(theme, "Tints"), StoryLayout.Centered),
        new("Gradient", theme => ComponentExamples.Progress(theme, "Gradient"), StoryLayout.Centered),
        new("Live", theme => ComponentExamples.Progress(theme, "Live"), StoryLayout.Centered),
        new("With a count", theme => ComponentExamples.Progress(theme, "With a count"), StoryLayout.Centered),
        new("Beside stages", theme => ComponentExamples.Progress(theme, "Beside stages"), StoryLayout.Centered),
    ]);
}
