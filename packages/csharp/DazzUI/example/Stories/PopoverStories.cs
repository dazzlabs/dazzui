// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class PopoverStories
{
    public static StoryPage Page { get; } = new("Components", "Popover", [
        new("Overview", theme => ComponentExamples.Popover(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Popover(theme, "Bare"), StoryLayout.Centered),
        new("Sides", theme => ComponentExamples.Popover(theme, "Sides"), StoryLayout.Centered),
        new("With controls", theme => ComponentExamples.Popover(theme, "With controls"), StoryLayout.Centered),
        new("A form", theme => ComponentExamples.Popover(theme, "A form"), StoryLayout.Centered),
        new("In a toolbar", theme => ComponentExamples.Popover(theme, "In a toolbar"), StoryLayout.Centered),
    ]);
}
