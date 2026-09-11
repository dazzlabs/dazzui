// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class AccordionStories
{
    public static StoryPage Page { get; } = new("Components", "Accordion", [
        new("Overview", theme => ComponentExamples.Accordion(theme, "Overview"), StoryLayout.Page),
        new("One at a time", theme => ComponentExamples.Accordion(theme, "One at a time"), StoryLayout.Centered),
        new("Multiple", theme => ComponentExamples.Accordion(theme, "Multiple"), StoryLayout.Centered),
        new("Divided", theme => ComponentExamples.Accordion(theme, "Divided"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Accordion(theme, "Disabled"), StoryLayout.Centered),
    ]);
}
