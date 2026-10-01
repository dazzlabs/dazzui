import 'package:flutter/widgets.dart';

import 'data/mails.dart';
import 'settings.dart';

/// Every word the window says, in the four languages the locale switch
/// offers. Hong Kong reads the Traditional set; anything unrecognised reads
/// English.
///
/// The messages themselves are not translated: they are the test data, each
/// written in the language whose setting it checks.
abstract class S {
  const S();

  static S of(BuildContext context) =>
      forLocale(Localizations.localeOf(context));

  static S forLocale(Locale locale) {
    switch (locale.languageCode) {
      case 'zh':
        final bool traditional =
            locale.scriptCode == 'Hant' ||
            const {'TW', 'HK', 'MO'}.contains(locale.countryCode);
        return traditional ? const _ZhHant() : const _ZhHans();
      case 'ja':
        return const _Ja();
      default:
        return const _En();
    }
  }

  String get appTitle;

  // Folders and labels.
  String folder(Folder folder);
  String label(Label label);
  String get mailboxes;
  String get labels;
  String labelTitle(String name);

  // The list.
  String get compose;
  String count(int total, int unread);
  String get searchPlaceholder;
  String get clearSearch;
  String get emptyBox;
  String noMatch(String query);

  // Actions.
  String get markRead;
  String get markUnread;
  String get star;
  String get unstar;
  String get archive;
  String get delete;
  String get reply;
  String get forward;
  String get more;
  String get moveToInbox;
  String get showOriginal;
  String get print;

  // The reader.
  String get me;
  String toLine(List<String> names);
  String replyPlaceholder(String name);
  String get send;
  String get selectMail;
  String wrote(String name);

  // Composing.
  String get composeSubtitle;
  String get recipient;
  String get recipientPlaceholder;
  String get subject;
  String get subjectPlaceholder;
  String get bodyPlaceholder;
  String get cancel;
  String get noSubject;
  String get noRecipient;

  // Dates.
  String get today;
  String get yesterday;
  String get justNow;
  String monthDay(int month, int day);

  // The status bar.
  String get language;
  String get font;
  String get appearance;
  String get followSystem;
  String get themeFont;
  String get systemFont;
  String get light;
  String get dark;
  String get saveSnapshot;
  String snapshotSaved(String path);
  String snapshotFailed(String message);

  // The environment report.
  String get environment;
  String get environmentSubtitle;
  String get copy;
  String get ok;
  String get platform;
  String get osVersion;
  String get desktop;
  String get session;
  String get inputMethod;
  String get systemLocale;
  String get systemLocales;
  String get chosenLocale;
  String get resolvedLocale;
  String get pixelRatio;
  String get textScale;

  /// A person as the window names them: the account's own address is "me"
  /// in the window's language, and a message sent to nobody says so.
  String nameOf(Person person) {
    if (person.isMe) return me;
    return person.name.isEmpty ? noRecipient : person.name;
  }

  String subjectOf(Mail mail) =>
      mail.subject.trim().isEmpty ? noSubject : mail.subject;

  /// One line of the environment report.
  String field(String key, String value) => '$key：$value';

  String localeName(LocaleChoice choice) => choice.label ?? followSystem;

  String fontName(FontChoice choice) =>
      choice.label ??
      switch (choice) {
        FontChoice.system => systemFont,
        _ => themeFont,
      };

  String appearanceName(AppearanceChoice choice) => switch (choice) {
    AppearanceChoice.system => followSystem,
    AppearanceChoice.light => light,
    AppearanceChoice.dark => dark,
  };

  /// The date a list row and a reader header show: the time alone today and
  /// yesterday, the month and day before that.
  String date(DateTime when) {
    final DateTime now = DateTime.now();
    if (now.difference(when).inMinutes.abs() < 1) return justNow;
    final DateTime day = DateTime(when.year, when.month, when.day);
    final int ago = mailboxToday.difference(day).inDays;
    final String time =
        '${when.hour}:${when.minute.toString().padLeft(2, '0')}';
    if (ago <= 0) return '$today $time';
    if (ago == 1) return '$yesterday $time';
    return monthDay(when.month, when.day);
  }
}

class _ZhHans extends S {
  const _ZhHans();

  @override
  String get appTitle => '邮件 · 中文显示检查';

  @override
  String folder(Folder folder) => switch (folder) {
    Folder.inbox => '收件箱',
    Folder.sent => '已发送',
    Folder.drafts => '草稿箱',
    Folder.archive => '归档',
    Folder.trash => '已删除',
  };
  @override
  String label(Label label) => switch (label) {
    Label.work => '工作',
    Label.personal => '个人',
    Label.notice => '通知',
  };
  @override
  String get mailboxes => '邮箱';
  @override
  String get labels => '标签';
  @override
  String labelTitle(String name) => '标签：$name';

  @override
  String get compose => '写邮件';
  @override
  String count(int total, int unread) =>
      unread > 0 ? '$total 封，$unread 封未读' : '$total 封';
  @override
  String get searchPlaceholder => '搜索发件人、主题或正文';
  @override
  String get clearSearch => '清除搜索';
  @override
  String get emptyBox => '这里还没有邮件';
  @override
  String noMatch(String query) => '没有找到“$query”';

  @override
  String get markRead => '标为已读';
  @override
  String get markUnread => '标为未读';
  @override
  String get star => '加星标';
  @override
  String get unstar => '取消星标';
  @override
  String get archive => '归档';
  @override
  String get delete => '删除';
  @override
  String get reply => '回复';
  @override
  String get forward => '转发';
  @override
  String get more => '更多';
  @override
  String get moveToInbox => '移到收件箱';
  @override
  String get showOriginal => '显示原始邮件';
  @override
  String get print => '打印……';

  @override
  String get me => '我';
  @override
  String toLine(List<String> names) => '收件人：${names.join('、')}';
  @override
  String replyPlaceholder(String name) => '回复 $name……（候选框是否紧贴光标？）';
  @override
  String get send => '发送';
  @override
  String get selectMail => '选择一封邮件来阅读';
  @override
  String wrote(String name) => '$name 写道：';

  @override
  String get composeSubtitle => '用拼音、注音、仓颉或五笔各输入一段，看看预编辑文本和候选框。';
  @override
  String get recipient => '收件人';
  @override
  String get recipientPlaceholder => '姓名或邮箱地址';
  @override
  String get subject => '主题';
  @override
  String get subjectPlaceholder => '例如：关于“中文排版检查”的反馈';
  @override
  String get bodyPlaceholder => '正文……';
  @override
  String get cancel => '取消';
  @override
  String get noSubject => '（无主题）';
  @override
  String get noRecipient => '（无收件人）';

  @override
  String get today => '今天';
  @override
  String get yesterday => '昨天';
  @override
  String get justNow => '刚刚';
  @override
  String monthDay(int month, int day) => '$month月$day日';

  @override
  String get language => '语言';
  @override
  String get font => '字体';
  @override
  String get appearance => '外观';
  @override
  String get followSystem => '跟随系统';
  @override
  String get themeFont => '主题默认';
  @override
  String get systemFont => '系统默认';
  @override
  String get light => '浅色';
  @override
  String get dark => '深色';
  @override
  String get saveSnapshot => '保存窗口截图';
  @override
  String snapshotSaved(String path) => '截图已保存到 $path';
  @override
  String snapshotFailed(String message) => '截图保存失败：$message';

  @override
  String get environment => '当前环境';
  @override
  String get environmentSubtitle => '截图对比时，把这一段一起贴上。';
  @override
  String get copy => '复制';
  @override
  String get ok => '好';
  @override
  String get platform => '平台';
  @override
  String get osVersion => '系统版本';
  @override
  String get desktop => '桌面';
  @override
  String get session => '会话';
  @override
  String get inputMethod => '输入法';
  @override
  String get systemLocale => '系统语言';
  @override
  String get systemLocales => '系统语言列表';
  @override
  String get chosenLocale => '选择的语言';
  @override
  String get resolvedLocale => '实际生效';
  @override
  String get pixelRatio => '设备像素比';
  @override
  String get textScale => '文字缩放';
}

class _ZhHant extends S {
  const _ZhHant();

  @override
  String get appTitle => '郵件 · 中文顯示檢查';

  @override
  String folder(Folder folder) => switch (folder) {
    Folder.inbox => '收件匣',
    Folder.sent => '寄件備份',
    Folder.drafts => '草稿',
    Folder.archive => '封存',
    Folder.trash => '垃圾桶',
  };
  @override
  String label(Label label) => switch (label) {
    Label.work => '工作',
    Label.personal => '個人',
    Label.notice => '通知',
  };
  @override
  String get mailboxes => '信箱';
  @override
  String get labels => '標籤';
  @override
  String labelTitle(String name) => '標籤：$name';

  @override
  String get compose => '撰寫郵件';
  @override
  String count(int total, int unread) =>
      unread > 0 ? '$total 封，$unread 封未讀' : '$total 封';
  @override
  String get searchPlaceholder => '搜尋寄件者、主旨或內文';
  @override
  String get clearSearch => '清除搜尋';
  @override
  String get emptyBox => '這裡還沒有郵件';
  @override
  String noMatch(String query) => '找不到「$query」';

  @override
  String get markRead => '標為已讀';
  @override
  String get markUnread => '標為未讀';
  @override
  String get star => '加上星號';
  @override
  String get unstar => '移除星號';
  @override
  String get archive => '封存';
  @override
  String get delete => '刪除';
  @override
  String get reply => '回覆';
  @override
  String get forward => '轉寄';
  @override
  String get more => '更多';
  @override
  String get moveToInbox => '移至收件匣';
  @override
  String get showOriginal => '顯示原始郵件';
  @override
  String get print => '列印⋯⋯';

  @override
  String get me => '我';
  @override
  String toLine(List<String> names) => '收件者：${names.join('、')}';
  @override
  String replyPlaceholder(String name) => '回覆 $name⋯⋯（候選視窗是否緊貼游標？）';
  @override
  String get send => '傳送';
  @override
  String get selectMail => '選擇一封郵件來閱讀';
  @override
  String wrote(String name) => '$name 寫道：';

  @override
  String get composeSubtitle => '用注音、倉頡、速成或拼音各輸入一段，看看組字區與候選視窗。';
  @override
  String get recipient => '收件者';
  @override
  String get recipientPlaceholder => '姓名或電子郵件地址';
  @override
  String get subject => '主旨';
  @override
  String get subjectPlaceholder => '例如：關於「中文排版檢查」的回饋';
  @override
  String get bodyPlaceholder => '內文⋯⋯';
  @override
  String get cancel => '取消';
  @override
  String get noSubject => '（無主旨）';
  @override
  String get noRecipient => '（無收件者）';

  @override
  String get today => '今天';
  @override
  String get yesterday => '昨天';
  @override
  String get justNow => '剛剛';
  @override
  String monthDay(int month, int day) => '$month月$day日';

  @override
  String get language => '語言';
  @override
  String get font => '字型';
  @override
  String get appearance => '外觀';
  @override
  String get followSystem => '跟隨系統';
  @override
  String get themeFont => '主題預設';
  @override
  String get systemFont => '系統預設';
  @override
  String get light => '淺色';
  @override
  String get dark => '深色';
  @override
  String get saveSnapshot => '儲存視窗截圖';
  @override
  String snapshotSaved(String path) => '截圖已儲存至 $path';
  @override
  String snapshotFailed(String message) => '截圖儲存失敗：$message';

  @override
  String get environment => '目前環境';
  @override
  String get environmentSubtitle => '比對截圖時，請連同這一段一起貼上。';
  @override
  String get copy => '拷貝';
  @override
  String get ok => '好';
  @override
  String get platform => '平台';
  @override
  String get osVersion => '系統版本';
  @override
  String get desktop => '桌面';
  @override
  String get session => '工作階段';
  @override
  String get inputMethod => '輸入法';
  @override
  String get systemLocale => '系統語言';
  @override
  String get systemLocales => '系統語言列表';
  @override
  String get chosenLocale => '選擇的語言';
  @override
  String get resolvedLocale => '實際生效';
  @override
  String get pixelRatio => '裝置像素比';
  @override
  String get textScale => '文字縮放';
}

class _Ja extends S {
  const _Ja();

  @override
  String get appTitle => 'メール · 中国語表示チェック';

  @override
  String folder(Folder folder) => switch (folder) {
    Folder.inbox => '受信トレイ',
    Folder.sent => '送信済み',
    Folder.drafts => '下書き',
    Folder.archive => 'アーカイブ',
    Folder.trash => 'ゴミ箱',
  };
  @override
  String label(Label label) => switch (label) {
    Label.work => '仕事',
    Label.personal => '個人',
    Label.notice => 'お知らせ',
  };
  @override
  String get mailboxes => 'メールボックス';
  @override
  String get labels => 'ラベル';
  @override
  String labelTitle(String name) => 'ラベル：$name';

  @override
  String get compose => '新規メール';
  @override
  String count(int total, int unread) =>
      unread > 0 ? '$total 件、未読 $unread 件' : '$total 件';
  @override
  String get searchPlaceholder => '差出人・件名・本文を検索';
  @override
  String get clearSearch => '検索をクリア';
  @override
  String get emptyBox => 'メールはありません';
  @override
  String noMatch(String query) => '「$query」に一致するメールはありません';

  @override
  String get markRead => '既読にする';
  @override
  String get markUnread => '未読にする';
  @override
  String get star => 'スターを付ける';
  @override
  String get unstar => 'スターを外す';
  @override
  String get archive => 'アーカイブ';
  @override
  String get delete => '削除';
  @override
  String get reply => '返信';
  @override
  String get forward => '転送';
  @override
  String get more => 'その他';
  @override
  String get moveToInbox => '受信トレイに移動';
  @override
  String get showOriginal => 'ソースを表示';
  @override
  String get print => '印刷…';

  @override
  String get me => '自分';
  @override
  String toLine(List<String> names) => '宛先：${names.join('、')}';
  @override
  String replyPlaceholder(String name) => '$name さんに返信…（候補ウィンドウはカーソルの近くに出ますか？）';
  @override
  String get send => '送信';
  @override
  String get selectMail => 'メールを選択してください';
  @override
  String wrote(String name) => '$name さんのメール：';

  @override
  String get composeSubtitle => 'ローマ字入力・かな入力で入力して、変換中の文字と候補ウィンドウを確認してください。';
  @override
  String get recipient => '宛先';
  @override
  String get recipientPlaceholder => '名前またはメールアドレス';
  @override
  String get subject => '件名';
  @override
  String get subjectPlaceholder => '例：「中国語組版チェック」へのフィードバック';
  @override
  String get bodyPlaceholder => '本文…';
  @override
  String get cancel => 'キャンセル';
  @override
  String get noSubject => '（件名なし）';
  @override
  String get noRecipient => '（宛先なし）';

  @override
  String get today => '今日';
  @override
  String get yesterday => '昨日';
  @override
  String get justNow => 'たった今';
  @override
  String monthDay(int month, int day) => '$month月$day日';

  @override
  String get language => '言語';
  @override
  String get font => 'フォント';
  @override
  String get appearance => '外観';
  @override
  String get followSystem => 'システムに合わせる';
  @override
  String get themeFont => 'テーマの既定';
  @override
  String get systemFont => 'システムの既定';
  @override
  String get light => 'ライト';
  @override
  String get dark => 'ダーク';
  @override
  String get saveSnapshot => 'ウィンドウのスクリーンショットを保存';
  @override
  String snapshotSaved(String path) => 'スクリーンショットを保存しました：$path';
  @override
  String snapshotFailed(String message) => '保存できませんでした：$message';

  @override
  String get environment => '現在の環境';
  @override
  String get environmentSubtitle => 'スクリーンショットを比較するときは、これも一緒に貼り付けてください。';
  @override
  String get copy => 'コピー';
  @override
  String get ok => 'OK';
  @override
  String get platform => 'プラットフォーム';
  @override
  String get osVersion => 'OS バージョン';
  @override
  String get desktop => 'デスクトップ';
  @override
  String get session => 'セッション';
  @override
  String get inputMethod => '入力メソッド';
  @override
  String get systemLocale => 'システムの言語';
  @override
  String get systemLocales => 'システムの言語一覧';
  @override
  String get chosenLocale => '選択した言語';
  @override
  String get resolvedLocale => '適用中';
  @override
  String get pixelRatio => 'デバイスピクセル比';
  @override
  String get textScale => '文字の拡大率';
}

class _En extends S {
  const _En();

  @override
  String field(String key, String value) => '$key: $value';

  @override
  String get appTitle => 'Mail · CJK Text Check';

  @override
  String folder(Folder folder) => switch (folder) {
    Folder.inbox => 'Inbox',
    Folder.sent => 'Sent',
    Folder.drafts => 'Drafts',
    Folder.archive => 'Archive',
    Folder.trash => 'Trash',
  };
  @override
  String label(Label label) => switch (label) {
    Label.work => 'Work',
    Label.personal => 'Personal',
    Label.notice => 'Notices',
  };
  @override
  String get mailboxes => 'Mailboxes';
  @override
  String get labels => 'Labels';
  @override
  String labelTitle(String name) => 'Label: $name';

  @override
  String get compose => 'New Message';
  @override
  String count(int total, int unread) {
    final String messages = total == 1 ? '1 message' : '$total messages';
    return unread > 0 ? '$messages, $unread unread' : messages;
  }

  @override
  String get searchPlaceholder => 'Search sender, subject or body';
  @override
  String get clearSearch => 'Clear search';
  @override
  String get emptyBox => 'No messages here';
  @override
  String noMatch(String query) => 'No results for “$query”';

  @override
  String get markRead => 'Mark as Read';
  @override
  String get markUnread => 'Mark as Unread';
  @override
  String get star => 'Star';
  @override
  String get unstar => 'Unstar';
  @override
  String get archive => 'Archive';
  @override
  String get delete => 'Delete';
  @override
  String get reply => 'Reply';
  @override
  String get forward => 'Forward';
  @override
  String get more => 'More';
  @override
  String get moveToInbox => 'Move to Inbox';
  @override
  String get showOriginal => 'Show Original';
  @override
  String get print => 'Print…';

  @override
  String get me => 'Me';
  @override
  String toLine(List<String> names) => 'To: ${names.join(', ')}';
  @override
  String replyPlaceholder(String name) =>
      'Reply to $name… (does the candidate window follow the caret?)';
  @override
  String get send => 'Send';
  @override
  String get selectMail => 'Select a message to read';
  @override
  String wrote(String name) => '$name wrote:';

  @override
  String get composeSubtitle =>
      'Type a line with each input method you have, and watch the preedit '
      'text and the candidate window.';
  @override
  String get recipient => 'To';
  @override
  String get recipientPlaceholder => 'Name or email address';
  @override
  String get subject => 'Subject';
  @override
  String get subjectPlaceholder => 'e.g. Feedback on the CJK text check';
  @override
  String get bodyPlaceholder => 'Message…';
  @override
  String get cancel => 'Cancel';
  @override
  String get noSubject => '(No subject)';
  @override
  String get noRecipient => '(No recipient)';

  @override
  String get today => 'Today';
  @override
  String get yesterday => 'Yesterday';
  @override
  String get justNow => 'Just now';
  @override
  String monthDay(int month, int day) => '${_months[month - 1]} $day';

  static const List<String> _months = [
    'Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', //
    'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec',
  ];

  @override
  String get language => 'Language';
  @override
  String get font => 'Font';
  @override
  String get appearance => 'Appearance';
  @override
  String get followSystem => 'System';
  @override
  String get themeFont => 'Theme default';
  @override
  String get systemFont => 'System default';
  @override
  String get light => 'Light';
  @override
  String get dark => 'Dark';
  @override
  String get saveSnapshot => 'Save window snapshot';
  @override
  String snapshotSaved(String path) => 'Snapshot saved to $path';
  @override
  String snapshotFailed(String message) => 'Could not save snapshot: $message';

  @override
  String get environment => 'Environment';
  @override
  String get environmentSubtitle =>
      'Paste this along with your screenshots when comparing.';
  @override
  String get copy => 'Copy';
  @override
  String get ok => 'OK';
  @override
  String get platform => 'Platform';
  @override
  String get osVersion => 'OS version';
  @override
  String get desktop => 'Desktop';
  @override
  String get session => 'Session';
  @override
  String get inputMethod => 'Input method';
  @override
  String get systemLocale => 'System locale';
  @override
  String get systemLocales => 'System locales';
  @override
  String get chosenLocale => 'Chosen locale';
  @override
  String get resolvedLocale => 'Resolved locale';
  @override
  String get pixelRatio => 'Device pixel ratio';
  @override
  String get textScale => 'Text scale';
}
