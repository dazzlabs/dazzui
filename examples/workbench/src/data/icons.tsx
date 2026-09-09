/**
 * The icon set, which is Fluent — the same one the React package and the Flutter
 * port draw their own three glyphs from, so the window is one hand from the
 * library's panel toggle to the last row in a list.
 *
 * Each is re-exported under the name the workbench reads it by, which is the
 * *role* rather than the picture: a glyph is the one thing a product replaces
 * wholesale, and a call site that says `SendGlyph` survives the swap where
 * one that said `ArrowUpRegular` would not. The unsized variants are used
 * throughout — they draw on a 20-unit grid at whatever `Icon` is given —
 * so one import serves an 11px badge and a 28px version mark alike.
 */
export {
  AddRegular as PlusGlyph,
  ArchiveRegular as ArchiveGlyph,
  ArrowClockwiseRegular as RefreshGlyph,
  ArrowDownloadRegular as DownloadGlyph,
  ArrowRightRegular as ArrowRightGlyph,
  ArrowUndoRegular as UndoGlyph,
  ArrowUpRegular as SendGlyph,
  AttachRegular as ClipGlyph,
  BeakerRegular as FlaskGlyph,
  BranchCompareRegular as DiffGlyph,
  BranchForkLinkRegular as TreeGlyph,
  BranchRegular as BranchGlyph,
  BranchRequestRegular as PullGlyph,
  ChatRegular as ChatGlyph,
  CheckmarkCircleRegular as CheckCircleGlyph,
  CheckmarkRegular as CheckGlyph,
  ChevronRightRegular as ChevronGlyph,
  ClockRegular as ClockGlyph,
  CloudRegular as CloudGlyph,
  CodeCircleRegular as GithubGlyph,
  CodeRegular as CodeGlyph,
  CommentRegular as CommentGlyph,
  CompassNorthwestRegular as CompassGlyph,
  CopyRegular as CopyGlyph,
  DismissCircleRegular as CrossCircleGlyph,
  DismissRegular as CrossGlyph,
  DocumentRegular as DocGlyph,
  EditRegular as PencilGlyph,
  EyeRegular as EyeGlyph,
  FolderOpenRegular as FolderOpenGlyph,
  FolderRegular as FolderGlyph,
  HandRightRegular as HandGlyph,
  KeyboardRegular as KeyboardGlyph,
  LayerRegular as LayersGlyph,
  MailInboxRegular as TrayGlyph,
  MentionRegular as AtGlyph,
  MergeRegular as MergeGlyph,
  MoreHorizontalRegular as MoreGlyph,
  OpenRegular as ExternalGlyph,
  OptionsRegular as CommandGlyph,
  PanelLeftRegular as SidebarGlyph,
  PauseRegular as PauseGlyph,
  PinRegular as PinGlyph,
  PlayRegular as PlayGlyph,
  RadioButtonRegular as IssueGlyph,
  RecordStopRegular as StopGlyph,
  RocketRegular as RocketGlyph,
  RocketRegular as RocketShipGlyph,
  SearchRegular as SearchGlyph,
  SettingsRegular as GearGlyph,
  ShieldRegular as ShieldGlyph,
  SparkleRegular as SparkleGlyph,
  SplitVerticalRegular as SplitGlyph,
  TaskListLtrRegular as QueueGlyph,
  TextBulletListLtrRegular as ListGlyph,
  WarningRegular as WarningGlyph,
  WindowConsoleRegular as TerminalGlyph,
  WrenchRegular as WrenchGlyph,
} from '@fluentui/react-icons'
