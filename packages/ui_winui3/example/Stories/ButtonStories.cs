// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ButtonStories
{
    public static StoryPage Page { get; } = new("Components", "Button", [
        new("Overview", theme => ComponentExamples.Button(theme, "Overview"), StoryLayout.Page),
        new("Normal", theme => ComponentExamples.Button(theme, "Normal"), StoryLayout.Centered),
        new("Recessed", theme => ComponentExamples.Button(theme, "Recessed"), StoryLayout.Centered),
        new("Filled", theme => ComponentExamples.Button(theme, "Filled"), StoryLayout.Centered),
        new("Tinted", theme => ComponentExamples.Button(theme, "Tinted"), StoryLayout.Centered),
        new("Outlined", theme => ComponentExamples.Button(theme, "Outlined"), StoryLayout.Centered),
        new("Plain", theme => ComponentExamples.Button(theme, "Plain"), StoryLayout.Centered),
        new("All variants", theme => ComponentExamples.Button(theme, "All variants"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Button(theme, "Disabled"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Button(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Button(theme, "Tints"), StoryLayout.Centered),
        new("With an icon", theme => ComponentExamples.Button(theme, "With an icon"), StoryLayout.Centered),
        new("With a shortcut", theme => ComponentExamples.Button(theme, "With a shortcut"), StoryLayout.Centered),
        new("Busy", theme => ComponentExamples.Button(theme, "Busy"), StoryLayout.Centered),
        new("Full width", theme => ComponentExamples.Button(theme, "Full width"), StoryLayout.Centered),
        new("Destructive", theme => ComponentExamples.Button(theme, "Destructive"), StoryLayout.Centered),
        new("In an action bar", theme => ComponentExamples.Button(theme, "In an action bar"), StoryLayout.Centered),
    ]);
}
