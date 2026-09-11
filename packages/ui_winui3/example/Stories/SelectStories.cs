// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SelectStories
{
    public static StoryPage Page { get; } = new("Components", "Select", [
        new("Overview", theme => ComponentExamples.Select(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.Select(theme, "Default"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Select(theme, "Disabled"), StoryLayout.Centered),
        new("Error state", theme => ComponentExamples.Select(theme, "Error state"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Select(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Select(theme, "Tints"), StoryLayout.Centered),
        new("Beside a button", theme => ComponentExamples.Select(theme, "Beside a button"), StoryLayout.Centered),
        new("In a form field", theme => ComponentExamples.Select(theme, "In a form field"), StoryLayout.Centered),
        new("In a settings row", theme => ComponentExamples.Select(theme, "In a settings row"), StoryLayout.Centered),
    ]);
}
