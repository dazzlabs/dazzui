// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class CalloutStories
{
    public static StoryPage Page { get; } = new("Components", "Callout", [
        new("Overview", theme => ComponentExamples.Callout(theme, "Overview"), StoryLayout.Page),
        new("Info", theme => ComponentExamples.Callout(theme, "Info"), StoryLayout.Centered),
        new("Success", theme => ComponentExamples.Callout(theme, "Success"), StoryLayout.Centered),
        new("Warning", theme => ComponentExamples.Callout(theme, "Warning"), StoryLayout.Centered),
        new("Danger", theme => ComponentExamples.Callout(theme, "Danger"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Callout(theme, "Tints"), StoryLayout.Centered),
        new("Without a title", theme => ComponentExamples.Callout(theme, "Without a title"), StoryLayout.Centered),
        new("With an icon", theme => ComponentExamples.Callout(theme, "With an icon"), StoryLayout.Centered),
        new("With an action", theme => ComponentExamples.Callout(theme, "With an action"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Callout(theme, "Sizes"), StoryLayout.Centered),
        new("In a pane", theme => ComponentExamples.Callout(theme, "In a pane"), StoryLayout.Centered),
    ]);
}
