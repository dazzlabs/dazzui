// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class FormFieldStories
{
    public static StoryPage Page { get; } = new("Components", "FormField", [
        new("Overview", theme => ComponentExamples.FormField(theme, "Overview"), StoryLayout.Page),
        new("With a hint", theme => ComponentExamples.FormField(theme, "With a hint"), StoryLayout.Centered),
        new("Without a hint", theme => ComponentExamples.FormField(theme, "Without a hint"), StoryLayout.Centered),
        new("Error state", theme => ComponentExamples.FormField(theme, "Error state"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.FormField(theme, "Tints"), StoryLayout.Centered),
        new("Over a select", theme => ComponentExamples.FormField(theme, "Over a select"), StoryLayout.Centered),
        new("Over a multiline field", theme => ComponentExamples.FormField(theme, "Over a multiline field"), StoryLayout.Centered),
        new("With a field value", theme => ComponentExamples.FormField(theme, "With a field value"), StoryLayout.Centered),
        new("Over any control", theme => ComponentExamples.FormField(theme, "Over any control"), StoryLayout.Centered),
        new("A full form", theme => ComponentExamples.FormField(theme, "A full form"), StoryLayout.Centered),
    ]);
}
