// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class BadgeStories
{
    public static StoryPage Page { get; } = new("Components", "Badge", [
        new("Overview", theme => ComponentExamples.Badge(theme, "Overview"), StoryLayout.Page),
        new("Tinted", theme => ComponentExamples.Badge(theme, "Tinted"), StoryLayout.Centered),
        new("Filled", theme => ComponentExamples.Badge(theme, "Filled"), StoryLayout.Centered),
        new("Outlined", theme => ComponentExamples.Badge(theme, "Outlined"), StoryLayout.Centered),
        new("Raised", theme => ComponentExamples.Badge(theme, "Raised"), StoryLayout.Centered),
        new("All variants", theme => ComponentExamples.Badge(theme, "All variants"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Badge(theme, "Tints"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Badge(theme, "Sizes"), StoryLayout.Centered),
        new("States", theme => ComponentExamples.Badge(theme, "States"), StoryLayout.Centered),
        new("Counts", theme => ComponentExamples.Badge(theme, "Counts"), StoryLayout.Centered),
        new("With a glyph", theme => ComponentExamples.Badge(theme, "With a glyph"), StoryLayout.Centered),
        new("In a table", theme => ComponentExamples.Badge(theme, "In a table"), StoryLayout.Centered),
        new("On a card", theme => ComponentExamples.Badge(theme, "On a card"), StoryLayout.Centered),
    ]);
}
