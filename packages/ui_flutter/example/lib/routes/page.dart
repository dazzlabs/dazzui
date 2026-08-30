import 'package:dazzui/dazzui.dart';
import 'package:flutter/material.dart' show AppBar, Icons, Scaffold;
import 'package:go_router/go_router.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return SidebarProvider(
      child: Builder(
        builder: (context) {
          final theme = Theme.of(context);

          return Scaffold(
            appBar: AppBar(
              leading: const SidebarTrigger(),
              title: const Text('DazzUI Playground'),
            ),
            body: Row(
              children: [
                Sidebar(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      SidebarHeader(
                        child: Text(
                          'Components',
                          style: theme.vars.titleSmall.copyWith(
                            color: theme.vars.colorContentMuted,
                          ),
                        ),
                      ),
                      SidebarContent(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            SidebarGroup(
                              child: Column(
                                crossAxisAlignment: CrossAxisAlignment.stretch,
                                children: [
                                  const SidebarGroupLabel(
                                    child: Text('Widgets'),
                                  ),
                                  SidebarGroupContent(
                                    child: SidebarMenu(
                                      children: [
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.notification_important_outlined),
                                            label: const Text('Alert Banner'),
                                            tooltip: 'Alert Banner',
                                            onPressed: () => context.go('/alert-banner'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.sell_outlined),
                                            label: const Text('Badge'),
                                            tooltip: 'Badge',
                                            onPressed: () => context.go('/badge'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.smart_button_outlined),
                                            label: const Text('Button'),
                                            tooltip: 'Button',
                                            onPressed: () => context.go('/button'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.crop_square_outlined),
                                            label: const Text('Card'),
                                            tooltip: 'Card',
                                            onPressed: () => context.go('/card'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.check_box_outlined),
                                            label: const Text('Checkbox'),
                                            tooltip: 'Checkbox',
                                            onPressed: () => context.go('/checkbox'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.horizontal_rule),
                                            label: const Text('Divider'),
                                            tooltip: 'Divider',
                                            onPressed: () => context.go('/divider'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.ads_click_outlined),
                                            label: const Text('Icon Button'),
                                            tooltip: 'Icon Button',
                                            onPressed: () => context.go('/icon-button'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.keyboard_outlined),
                                            label: const Text('Kbd'),
                                            tooltip: 'Kbd',
                                            onPressed: () => context.go('/kbd'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.autorenew),
                                            label: const Text('Loader'),
                                            tooltip: 'Loader',
                                            onPressed: () => context.go('/loader'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.radio_button_checked),
                                            label: const Text('Radio'),
                                            tooltip: 'Radio',
                                            onPressed: () => context.go('/radio'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.toggle_on_outlined),
                                            label: const Text('Switch'),
                                            tooltip: 'Switch',
                                            onPressed: () => context.go('/switch'),
                                          ),
                                        ),
                                        SidebarMenuItem(
                                          child: SidebarMenuButton(
                                            icon: const Icon(Icons.text_fields),
                                            label: const Text('Text Field'),
                                            tooltip: 'Text Field',
                                            onPressed: () => context.go('/text-field'),
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                      SidebarFooter(
                        child: Text(
                          'Collapse with the chevron in the app bar.',
                          style: theme.vars.captionMedium.copyWith(
                            color: theme.vars.colorContentMuted,
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
                const VerticalDivider(width: 1),
                Expanded(
                  child: Center(
                    child: Text(
                      'Select a component from the sidebar',
                      style: theme.vars.bodyLarge.copyWith(
                        color: theme.vars.colorContentMuted,
                      ),
                    ),
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
