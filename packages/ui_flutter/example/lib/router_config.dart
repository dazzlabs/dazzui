import 'package:flutter/widgets.dart';
import 'package:go_router/go_router.dart';
import 'routes/page.dart';
import 'routes/widgets/alert_banner.dart';
import 'routes/widgets/badge.dart';
import 'routes/widgets/button.dart';
import 'routes/widgets/card.dart';
import 'routes/widgets/checkbox.dart';
import 'routes/widgets/divider.dart';
import 'routes/widgets/icon_button.dart';
import 'routes/widgets/kbd.dart';
import 'routes/widgets/loader.dart';
import 'routes/widgets/radio.dart';
import 'routes/widgets/switch.dart';
import 'routes/widgets/text_field.dart';

part 'router_config.g.dart';

final GoRouter routerConfig = GoRouter(
  routes: $appRoutes,
);

@TypedGoRoute<HomeRoute>(path: '/', name: 'Home')
class HomeRoute extends GoRouteData with $HomeRoute {
  const HomeRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const HomePage();
}

@TypedGoRoute<AlertBannerRoute>(path: '/alert-banner', name: 'Alert Banner')
class AlertBannerRoute extends GoRouteData with $AlertBannerRoute {
  const AlertBannerRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) =>
      const AlertBannerPage();
}

@TypedGoRoute<BadgeRoute>(path: '/badge', name: 'Badge')
class BadgeRoute extends GoRouteData with $BadgeRoute {
  const BadgeRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const BadgePage();
}

@TypedGoRoute<ButtonRoute>(path: '/button', name: 'Button')
class ButtonRoute extends GoRouteData with $ButtonRoute {
  const ButtonRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const ButtonPage();
}

@TypedGoRoute<CardRoute>(path: '/card', name: 'Card')
class CardRoute extends GoRouteData with $CardRoute {
  const CardRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const CardPage();
}

@TypedGoRoute<CheckboxRoute>(path: '/checkbox', name: 'Checkbox')
class CheckboxRoute extends GoRouteData with $CheckboxRoute {
  const CheckboxRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) =>
      const CheckboxPage();
}

@TypedGoRoute<DividerRoute>(path: '/divider', name: 'Divider')
class DividerRoute extends GoRouteData with $DividerRoute {
  const DividerRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) =>
      const DividerPage();
}

@TypedGoRoute<IconButtonRoute>(path: '/icon-button', name: 'Icon Button')
class IconButtonRoute extends GoRouteData with $IconButtonRoute {
  const IconButtonRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) =>
      const IconButtonPage();
}

@TypedGoRoute<KbdRoute>(path: '/kbd', name: 'Kbd')
class KbdRoute extends GoRouteData with $KbdRoute {
  const KbdRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const KbdPage();
}

@TypedGoRoute<LoaderRoute>(path: '/loader', name: 'Loader')
class LoaderRoute extends GoRouteData with $LoaderRoute {
  const LoaderRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const LoaderPage();
}

@TypedGoRoute<RadioRoute>(path: '/radio', name: 'Radio')
class RadioRoute extends GoRouteData with $RadioRoute {
  const RadioRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const RadioPage();
}

@TypedGoRoute<SwitchRoute>(path: '/switch', name: 'Switch')
class SwitchRoute extends GoRouteData with $SwitchRoute {
  const SwitchRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) => const SwitchPage();
}

@TypedGoRoute<TextFieldRoute>(path: '/text-field', name: 'Text Field')
class TextFieldRoute extends GoRouteData with $TextFieldRoute {
  const TextFieldRoute();

  @override
  Widget build(BuildContext context, GoRouterState state) =>
      const TextFieldPage();
}
