import 'omarchy_palette.dart';

/// Where there is no file system — the web — there is no Omarchy to read.
Future<OmarchyPalette?> readOmarchyPalette({
  String? directory,
  String? config,
}) async => null;

/// Where there is no file system, a stream of the one answer: none.
Stream<OmarchyPalette?> watchOmarchyPalette({
  String? directory,
  String? config,
}) => Stream.value(null);
