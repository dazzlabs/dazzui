# Flutter port status

The implementation covers these 54 gallery entries:

| Area | Components |
| --- | --- |
| Actions and labels | Button, IconButton, Badge, Toggle, Link, Tag |
| Selection and input | Checkbox, CheckboxGroup, Radio, Switch, TextField, SearchField, NumberField, Slider, OtpField, Combobox, SegmentedControl, PillTabs |
| Display | Card, Divider, SectionLabel, KeyCap, Avatar, Callout, EmptyState, Code |
| Activity | Progress, Meter, Spinner, Skeleton |
| Navigation | Collapsible, Accordion, NavItem, Sidebar, Rail, Aside, Tree |
| Overlays | Popover, Dialog, Drawer, Menu, Menubar, ContextMenu, Tooltip, PreviewCard, Toast |
| Composition | Form, FormField, Preferences, ActionBar, StepList, OptionCard, Table, Splitter |

Additional exported building blocks include Pressable, FocusRing, FieldMetrics,
Fieldset, PreferenceGroup/Row, MenuPanel/Item, PopoverPanel, TableHead/Row/Cell,
SplitterPanel, and ToastManager/Viewport. Rust APIs use structs, enums, callbacks,
and native Inset state instead of reproducing Dart constructor signatures.

## Implemented behavior

Pressable manages hover, down, focus, disabled state, and Enter/Space activation.
Text editing delegates cursor movement, selection, IME, and clipboard operations
to Inset. Sliders support pointer drag and arrow/Home/End keys. Splitters enforce
both neighboring panels' limits for pointer and keyboard resize. Selection is
controlled by the caller; accordion, toggle, and disclosure also offer local state.

Popover uses OverlayPortal, linked composited positioning, outside dismissal, and
Escape. Dialog and drawer use native navigator routes. Menus dismiss after
selection and support Up/Down focus traversal. Hover popovers retain the panel while
the pointer crosses from the trigger. Toasts have independent IDs, dismissal, and
timed expiration. Spinner, skeleton, and indeterminate progress animate through
Inset's scheduler and animation controller.

## Remaining parity work

These differences are explicit follow-up work, not passing parity assertions:

- The gallery currently provides Overview specimens rather than every Flutter or
  React story and every axis/state permutation. There is no Inset reference
  extractor, drift ratchet, or screenshot comparison yet.
- Segmented controls, pill tabs, and trees use normal focus traversal; complete
  roving-keyboard navigation, tree semantics, and menu type-ahead/nested menus
  still need to be ported. Menu Up/Down uses the native focus traversal policy.
- Context menus anchor to their child rather than the exact pointer position.
  Popover positioning has explicit side/alignment but no collision flipping.
- Switches toggle on tap; Flutter drag physics are not reproduced. Some visual
  transitions are immediate. Skeleton pulses rather than reproducing Flutter's
  shimmer, and indeterminate progress is not pixel-matched.
- Toasts have no hover pause or swipe dismissal. CheckboxGroup select-all does not
  restore the previously selected subset after clearing all.
- Per-component Flutter ThemeExtension overrides and the entire Dart API surface
  are not reproduced. Generated ThemeVariables are available for custom themes.
- Accessibility and platform behavior depend on the current Inset implementation;
  no screen-reader audit, mobile run, Windows/Linux run, or web build has been
  completed for this package.

The native macOS gallery, actual text entry and theme changes were exercised.
The automated suite validates widget lifecycle and selected interactions, not the
absence of all visual or behavioral differences listed above.
