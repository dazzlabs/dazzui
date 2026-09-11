// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SpinnerStories
{
    public static StoryPage Page { get; } = new("Components", "Spinner", [
        new("Overview", theme => ComponentExamples.Spinner(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.Spinner(theme, "Default"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Spinner(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Spinner(theme, "Tints"), StoryLayout.Centered),
        new("On an accent", theme => ComponentExamples.Spinner(theme, "On an accent"), StoryLayout.Centered),
        new("In a button", theme => ComponentExamples.Spinner(theme, "In a button"), StoryLayout.Centered),
        new("In a callout", theme => ComponentExamples.Spinner(theme, "In a callout"), StoryLayout.Centered),
        new("In a toast", theme => ComponentExamples.Spinner(theme, "In a toast"), StoryLayout.Centered),
        new("In a panel", theme => ComponentExamples.Spinner(theme, "In a panel"), StoryLayout.Centered),
    ]);
}
