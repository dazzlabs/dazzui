// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class OptionCardStories
{
    public static StoryPage Page { get; } = new("Components", "OptionCard", [
        new("Overview", theme => ComponentExamples.OptionCard(theme, "Overview"), StoryLayout.Page),
        new("Selected", theme => ComponentExamples.OptionCard(theme, "Selected"), StoryLayout.Centered),
        new("Unselected", theme => ComponentExamples.OptionCard(theme, "Unselected"), StoryLayout.Centered),
        new("A group", theme => ComponentExamples.OptionCard(theme, "A group"), StoryLayout.Centered),
        new("Without a description", theme => ComponentExamples.OptionCard(theme, "Without a description"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.OptionCard(theme, "Disabled"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.OptionCard(theme, "Tints"), StoryLayout.Centered),
        new("In a grid", theme => ComponentExamples.OptionCard(theme, "In a grid"), StoryLayout.Centered),
    ]);
}
