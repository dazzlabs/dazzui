// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class CardStories
{
    public static StoryPage Page { get; } = new("Components", "Card", [
        new("Overview", theme => ComponentExamples.Card(theme, "Overview"), StoryLayout.Page),
        new("Raised", theme => ComponentExamples.Card(theme, "Raised"), StoryLayout.Centered),
        new("Sunken", theme => ComponentExamples.Card(theme, "Sunken"), StoryLayout.Centered),
        new("Outlined", theme => ComponentExamples.Card(theme, "Outlined"), StoryLayout.Centered),
        new("Tinted", theme => ComponentExamples.Card(theme, "Tinted"), StoryLayout.Centered),
        new("Variants", theme => ComponentExamples.Card(theme, "Variants"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Card(theme, "Tints"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Card(theme, "Sizes"), StoryLayout.Centered),
        new("With an action", theme => ComponentExamples.Card(theme, "With an action"), StoryLayout.Centered),
        new("As a grid", theme => ComponentExamples.Card(theme, "As a grid"), StoryLayout.Centered),
        new("As a well", theme => ComponentExamples.Card(theme, "As a well"), StoryLayout.Centered),
        new("Nested", theme => ComponentExamples.Card(theme, "Nested"), StoryLayout.Centered),
    ]);
}
