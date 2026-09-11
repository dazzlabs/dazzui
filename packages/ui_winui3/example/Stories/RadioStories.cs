// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class RadioStories
{
    public static StoryPage Page { get; } = new("Components", "Radio", [
        new("Overview", theme => ComponentExamples.Radio(theme, "Overview"), StoryLayout.Page),
        new("Checked", theme => ComponentExamples.Radio(theme, "Checked"), StoryLayout.Centered),
        new("Unchecked", theme => ComponentExamples.Radio(theme, "Unchecked"), StoryLayout.Centered),
        new("A group", theme => ComponentExamples.Radio(theme, "A group"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Radio(theme, "Disabled"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Radio(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Radio(theme, "Tints"), StoryLayout.Centered),
        new("Against a checkbox", theme => ComponentExamples.Radio(theme, "Against a checkbox"), StoryLayout.Centered),
        new("In a form", theme => ComponentExamples.Radio(theme, "In a form"), StoryLayout.Centered),
    ]);
}
