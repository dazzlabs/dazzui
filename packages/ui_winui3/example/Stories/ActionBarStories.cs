// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ActionBarStories
{
    public static StoryPage Page { get; } = new("Components", "ActionBar", [
        new("Overview", theme => ComponentExamples.ActionBar(theme, "Overview"), StoryLayout.Page),
        new("Start", theme => ComponentExamples.ActionBar(theme, "Start"), StoryLayout.Centered),
        new("End", theme => ComponentExamples.ActionBar(theme, "End"), StoryLayout.Centered),
        new("Between", theme => ComponentExamples.ActionBar(theme, "Between"), StoryLayout.Centered),
        new("As a toolbar", theme => ComponentExamples.ActionBar(theme, "As a toolbar"), StoryLayout.Centered),
        new("An action row", theme => ComponentExamples.ActionBar(theme, "An action row"), StoryLayout.Centered),
        new("With a key hint", theme => ComponentExamples.ActionBar(theme, "With a key hint"), StoryLayout.Centered),
        new("Mixed controls", theme => ComponentExamples.ActionBar(theme, "Mixed controls"), StoryLayout.Centered),
    ]);
}
