// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class AutocompleteStories
{
    public static StoryPage Page { get; } = new("Components", "Autocomplete", [
        new("Overview", theme => ComponentExamples.Autocomplete(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Autocomplete(theme, "Bare"), StoryLayout.Centered),
        new("Mono", theme => ComponentExamples.Autocomplete(theme, "Mono"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Autocomplete(theme, "Sizes"), StoryLayout.Centered),
        new("States", theme => ComponentExamples.Autocomplete(theme, "States"), StoryLayout.Centered),
        new("In a form", theme => ComponentExamples.Autocomplete(theme, "In a form"), StoryLayout.Centered),
    ]);
}
