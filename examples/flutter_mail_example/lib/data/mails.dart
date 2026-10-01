import 'qianziwen.dart';

enum Folder { inbox, sent, drafts, archive, trash }

enum Label { work, personal, notice }

/// The day the mailbox was written on. Dates are shown relative to it, so
/// "today" stays the day the weekly notes went out whenever the app runs.
final DateTime mailboxToday = DateTime(2026, 10, 1);

DateTime _at(int daysAgo, int hour, int minute) => mailboxToday
    .subtract(Duration(days: daysAgo))
    .add(Duration(hours: hour, minutes: minute));

class Person {
  const Person(this.name, this.address);

  final String name;
  final String address;

  bool get isMe => address == me.address;
}

/// The account's own address. Its name is drawn in the window's language.
const Person me = Person('', 'me@dazzlabs.dev');

/// One message. The body is plain text: paragraphs are separated by a blank
/// line, a paragraph whose lines start with `> ` is a quotation, and one whose
/// lines start with `$ ` is set in the code face.
class Mail {
  Mail({
    required this.id,
    required this.folder,
    required this.from,
    this.to = const [me],
    required this.subject,
    required this.sent,
    required this.body,
    this.unread = false,
    this.starred = false,
    this.labels = const {},
    this.attachments = const [],
  });

  final String id;
  Folder folder;
  final Person from;
  final List<Person> to;
  final String subject;
  final DateTime sent;
  final String body;
  bool unread;
  bool starred;
  final Set<Label> labels;
  final List<String> attachments;

  /// The first lines of the body run together, the way a list row shows it.
  String get preview => body
      .split('\n')
      .map((line) => line.replaceFirst(RegExp(r'^(> |\$ )'), '').trim())
      .where((line) => line.isNotEmpty)
      .join(' ');
}

/// The mailbox. Every message hides a case a Chinese desktop gets wrong:
/// mixed scripts and numbers, punctuation, Traditional and Hong Kong forms,
/// Japanese glyphs that share a code point with Chinese ones, characters
/// outside the BMP, monospace alignment, and a thousand-character text.
List<Mail> seedMails() => [
  Mail(
    id: 'weekly',
    folder: Folder.inbox,
    from: const Person('周雨桐', 'yutong.zhou@dazzlabs.dev'),
    to: const [me, Person('林冠宇', 'kuanyu.lin@dazzlabs.tw')],
    subject: '第 42 周产品周会纪要（含 Q4 排期）',
    sent: _at(0, 14, 32),
    unread: true,
    starred: true,
    labels: {Label.work},
    body: '''各位好：

以下是本周（9 月 28 日—10 月 2 日）产品周会的纪要，请在周五 18:00 前回复确认。

一、进度
1. DazzUI 1.0 已完成 87% 的组件迁移，剩余的 WindowFrame、Sidebar 仍在评审中；
2. Flutter 端在 Hyprland（Wayland）下偶发“输入法候选框错位”的问题，详见 #1287；
3. 设计稿已更新到 Figma 的《DazzUI 2026 秋季》页面。

二、待定事项
——是否在 10 月中旬发布 Beta？
——中文字体是否随包分发（约 16 MB）？……我倾向于不打包，但要先看看各平台的回退效果。

三、下周安排
· 周一：与 Windows 组对齐 DirectWrite 的回退表
· 周三：「中文排版检查」专项评审
· 周五：发布候选版（RC1）

如有遗漏，欢迎补充。

雨桐''',
  ),
  Mail(
    id: 'taipei',
    folder: Folder.inbox,
    from: const Person('林冠宇', 'kuanyu.lin@dazzlabs.tw'),
    subject: '關於下週發表會的簡報與「直播」安排',
    sent: _at(0, 11, 5),
    unread: true,
    labels: {Label.work},
    body: '''雨桐、各位好：

下週三（10/7）的發表會，我們這邊已經把簡報改好了，請幫忙確認以下幾點：

1. 第 12 頁的標題「直覺、骨架與角色」是否沿用？
2. 直播平臺改用 YouTube，備援為 Twitch；
3. 會後茶敘的點心，我訂了鳳梨酥與牛軋糖，共 40 份。

另外，台北辦公室的螢幕上，標點符號（，。、；：）是置中的，跟上海那邊的截圖不太一樣——這是字型的差異，不是 bug，請大家不要再開 issue 了 😅

冠宇 敬上''',
  ),
  Mail(
    id: 'hongkong',
    folder: Folder.inbox,
    from: const Person('陳嘉欣', 'kayan.chan@dazzlabs.hk'),
    subject: '搬辦公室嘅安排（請睇附件）',
    sent: _at(1, 18, 47),
    labels: {Label.notice},
    attachments: ['觀塘新辦公室平面圖.pdf', '座位表_v3.xlsx'],
    body: '''大家好：

下個禮拜五我哋會搬去觀塘新辦公室，搭𨋢上 23 樓就係。有幾樣嘢想提一提：

• 啲私人物品請喺星期四之前執好，貼返自己個名；
• 新地址：九龍觀塘道 388 號 23 樓；
• 嗰日冇得食晏，我哋會叫外賣，有咩唔食記得話我知㗎。

唔該晒！

Kayan''',
  ),
  Mail(
    id: 'tokyo',
    folder: Folder.inbox,
    from: const Person('山田 花子', 'hanako.yamada@example.co.jp'),
    subject: '【ご確認】フォント表示の件について',
    sent: _at(1, 9, 20),
    labels: {Label.work},
    body: '''李様

いつもお世話になっております。山田です。

先日お送りいただいたスクリーンショットについて、いくつか気になる点がございました。

・「直」「骨」「角」「次」「令」などの漢字が、日本語の字形ではなく中国語の字形で表示されています。
・句読点（、。）の位置は問題ありません。
・「〜」と「～」の違いにもご注意ください。

ロケールの設定をご確認いただけますと幸いです。

何卒よろしくお願いいたします。

山田 花子
株式会社サンプル 開発部''',
  ),
  Mail(
    id: 'welcome',
    folder: Folder.inbox,
    from: const Person('张敏', 'hr@dazzlabs.dev'),
    subject: '欢迎新同事：刘龑、𠮷田阳翔与王喆昇加入！',
    sent: _at(2, 16, 0),
    unread: true,
    labels: {Label.notice},
    body: '''大家好！

本周有三位新同事加入，请多多关照：

· 刘龑（yǎn）——设计系统组，来自广州；
· 𠮷田阳翔（よしだ はると）——东京分部，前端工程师；
· 王喆昇（zhé shēng）——平台组，负责 Linux 与 Windows 的发布。

周五晚上 7 点，我们在公司楼下的“𰻝𰻝面”小馆聚餐，欢迎大家参加 🍜🎉 人均约 ￥80，费用由部门承担。

另：下面这些字如果显示成方框，说明当前字体缺字，请截图发给我：
䶮 㐀 𠀀 𪚥 𰻝 龘 靐 齉 𨋢 㗎

张敏''',
  ),
  Mail(
    id: 'bookclub',
    folder: Folder.inbox,
    from: const Person('读书会', 'bookclub@dazzlabs.dev'),
    subject: '读书会第 9 期：《千字文》全文（简体）',
    sent: _at(5, 20, 0),
    labels: {Label.personal},
    body: [
      '各位书友：',
      '本期共读《千字文》。全文四段，一千字左右，几乎不重复——拿来检查字体覆盖和行高再合适不过。',
      ...qianziwen,
      '下期预告：《声律启蒙》。',
    ].join('\n\n'),
  ),
  Mail(
    id: 'ci',
    folder: Folder.inbox,
    from: const Person('CI 机器人', 'ci@dazzlabs.dev'),
    subject: '[dazzlabs/dazzui] 构建失败：flutter test (linux-x64) · a12b5fb',
    sent: _at(6, 3, 12),
    labels: {Label.notice},
    body: r'''main 分支上的工作流「CI / flutter」运行失败。

提交：a12b5fb Add ten components to the mobile package
耗时：4 分 37 秒

失败的步骤：

$ flutter test test/text_field_test.dart
$ Expected: '粘贴'
$   Actual: 'Paste'
$    Which: is different.

查看运行记录：https://ci.dazzlabs.dev/runs/123456789

你收到这封邮件，是因为你订阅了该仓库的工作流通知。''',
  ),
  Mail(
    id: 'bill',
    folder: Folder.inbox,
    from: const Person('账单中心', 'billing@cloud.example'),
    subject: '您的 9 月账单：￥1,280.00（已自动扣款）',
    sent: _at(11, 8, 0),
    labels: {Label.personal},
    body: r'''尊敬的客户：

您 2026 年 9 月的云服务账单已出，明细如下（等宽字体里，中文是否恰好占两个西文字符宽？）：

$ 项目              数量        金额
$ 计算实例          ×3       ￥  860.00
$ 对象存储          1.2 TB   ￥  312.50
$ 公网流量          ５００ GB ￥  107.50
$ 合计                       ￥1,280.00

本月费用较上月增长 12.5％（全角百分号）——也有人写成 12.5%（半角），两种我们都见过。

账单周期：2026/09/01 – 2026/09/30
如有疑问，请拨打 400-000-0000（工作日 9:00–18:00）。''',
  ),
  Mail(
    id: 'reply-weekly',
    folder: Folder.sent,
    from: me,
    to: const [Person('周雨桐', 'yutong.zhou@dazzlabs.dev')],
    subject: 'Re: 第 41 周产品周会纪要',
    sent: _at(10, 22, 40),
    labels: {Label.work},
    body: '''收到，第二条我来跟进。

> 2. Linux 下的中文默认回退到了 Noto Sans CJK JP，导致“直”“骨”显示成日文字形；
> 请平台组确认 locale 是否传到了引擎。''',
  ),
  Mail(
    id: 'draft',
    folder: Folder.drafts,
    from: me,
    to: const [Person('山田 花子', 'hanako.yamada@example.co.jp')],
    subject: 'Re: 【ご確認】フォント表示の件について',
    sent: _at(1, 21, 13),
    body: '''山田さん

ご指摘ありがとうございます。ロケールを ja-JP に切り替えたところ……''',
  ),
  Mail(
    id: 'archived',
    folder: Folder.archive,
    from: const Person('周雨桐', 'yutong.zhou@dazzlabs.dev'),
    subject: '国庆假期值班安排（10月1日—10月7日）',
    sent: _at(16, 17, 30),
    labels: {Label.work},
    body: '''假期值班表已经排好，见附件。

值班期间请保持手机畅通；遇到线上故障，先在「值班群」里同步，再按流程升级。

祝大家节日快乐！''',
    attachments: ['国庆值班表.xlsx'],
  ),
];
