// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SearchFieldStories
{
    public static StoryPage Page { get; } = new("Components", "SearchField", [
        new("Overview", theme => ComponentExamples.SearchField(theme, "Overview"), StoryLayout.Page),
        new("Empty", theme => ComponentExamples.SearchField(theme, "Empty"), StoryLayout.Centered),
        new("With a query", theme => ComponentExamples.SearchField(theme, "With a query"), StoryLayout.Centered),
        new("With a hint", theme => ComponentExamples.SearchField(theme, "With a hint"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.SearchField(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.SearchField(theme, "Tints"), StoryLayout.Centered),
        new("Dismissable", theme => ComponentExamples.SearchField(theme, "Dismissable"), StoryLayout.Centered),
        new("Filtering a list", theme => ComponentExamples.SearchField(theme, "Filtering a list"), StoryLayout.Centered),
        new("In a pane header", theme => ComponentExamples.SearchField(theme, "In a pane header"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.SearchField(theme, "Disabled"), StoryLayout.Centered),
    ]);
}
