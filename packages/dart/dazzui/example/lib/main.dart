import 'package:dazzui/dazzui.dart';

import 'app.dart';
import 'lib/location.dart';

void main() {
  final location = readLocation(Uri.base);
  runApp(App(page: location.page, theme: location.theme));
}
