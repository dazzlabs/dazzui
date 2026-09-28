// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class CheckboxStories
{
    public static StoryPage Page { get; } = new("Components", "Checkbox", [
        new("Overview", theme => ComponentExamples.Checkbox(theme, "Overview"), StoryLayout.Page),
        new("Checked", theme => ComponentExamples.Checkbox(theme, "Checked"), StoryLayout.Centered),
        new("Unchecked", theme => ComponentExamples.Checkbox(theme, "Unchecked"), StoryLayout.Centered),
        new("Interactive", theme => ComponentExamples.Checkbox(theme, "Interactive"), StoryLayout.Centered),
        new("A group", theme => ComponentExamples.Checkbox(theme, "A group"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Checkbox(theme, "Disabled"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Checkbox(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Checkbox(theme, "Tints"), StoryLayout.Centered),
        new("With a note", theme => ComponentExamples.Checkbox(theme, "With a note"), StoryLayout.Centered),
        new("A checklist", theme => ComponentExamples.Checkbox(theme, "A checklist"), StoryLayout.Centered),
        new("In a form", theme => ComponentExamples.Checkbox(theme, "In a form"), StoryLayout.Centered),
    ]);
}
