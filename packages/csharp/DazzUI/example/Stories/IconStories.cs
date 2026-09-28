// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class IconStories
{
    public static StoryPage Page { get; } = new("Components", "Icon", [
        new("Overview", theme => ComponentExamples.Icon(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.Icon(theme, "Default"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Icon(theme, "Sizes"), StoryLayout.Centered),
        new("Sized by a token", theme => ComponentExamples.Icon(theme, "Sized by a token"), StoryLayout.Centered),
        new("Inherited colour", theme => ComponentExamples.Icon(theme, "Inherited colour"), StoryLayout.Centered),
        new("A set", theme => ComponentExamples.Icon(theme, "A set"), StoryLayout.Centered),
        new("In a button", theme => ComponentExamples.Icon(theme, "In a button"), StoryLayout.Centered),
        new("In a toolbar", theme => ComponentExamples.Icon(theme, "In a toolbar"), StoryLayout.Centered),
        new("In a nav row", theme => ComponentExamples.Icon(theme, "In a nav row"), StoryLayout.Centered),
    ]);
}
