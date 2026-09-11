// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class StepListStories
{
    public static StoryPage Page { get; } = new("Components", "StepList", [
        new("Overview", theme => ComponentExamples.StepList(theme, "Overview"), StoryLayout.Page),
        new("In flight", theme => ComponentExamples.StepList(theme, "In flight"), StoryLayout.Centered),
        new("Statuses", theme => ComponentExamples.StepList(theme, "Statuses"), StoryLayout.Centered),
        new("Finished", theme => ComponentExamples.StepList(theme, "Finished"), StoryLayout.Centered),
        new("Timed detail", theme => ComponentExamples.StepList(theme, "Timed detail"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.StepList(theme, "Tints"), StoryLayout.Centered),
        new("Advancing", theme => ComponentExamples.StepList(theme, "Advancing"), StoryLayout.Centered),
        new("In a card", theme => ComponentExamples.StepList(theme, "In a card"), StoryLayout.Centered),
    ]);
}
