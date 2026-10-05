# dazzui-compiler：Rust-first 原生 UI 代码生成器

Rust 是 UI 的源语言，SwiftUI 和 WinUI 3 是编译目标。构建时把 `ui!` 写的
页面编译成可读的平台源码；运行时只有真正的 SwiftUI / WinUI 3，加一条窄的
C ABI 回到 Rust。没有运行时 renderer、没有 Virtual DOM、没有自绘控件、没有
layout engine。

```
ui! { … }  ──parse──▶ AST ──semantic──▶ Semantic IR ──lower──▶ SwiftUI (.swift)
 (Rust 源码)          (syn)   (registry +        (平台无关)           WinUI 3 (.xaml + .xaml.cs)
                              Rust 类型)                              未来: GTK …
```

本文是方案；实现按 Phase 1–5 推进，每个 Phase 结束可以单独评审。

---

## 0. 几个先定下来的决定

| 决定 | 原因 |
|---|---|
| WinUI 目标生成 **C#**（`.xaml` + `.xaml.cs`），不是 C++/WinRT | 现有组件库 `packages/csharp/DazzUI` 是 C#。生成 C++ 就得跨语言消费 C# 控件，得不偿失。 |
| 解析器是 `dazzui-compiler` 里的 `syntax` 模块，proc macro 和 CLI 共用 | CLI 在构建时要从 `.rs` 文件里读出同一份 DSL，必须和宏用同一个 parser，否则两边语法会慢慢分叉。宏本身只是一层薄壳。 |
| `ui!` 页面声明 state 和 action 类型：`page SettingsPage(state: SettingsState, action: SettingsAction) { … }` | 宏需要知道 `state.x` 和 `Action::X` 指向哪里，才能生成类型检查代码；CLI 需要据此找到 Rust 类型定义。DSL 里的 `Action::` 是这个类型的别名。 |
| 宏展开成**只在编译期存在的类型检查代码**，运行时零产物 | 交给 rustc 去检查 `state.saving` 是不是 `bool`、`Action::SetAutoUpdate` 能不能接 `bool`，错误直接标在 DSL 上，rust-analyzer 也能看到。见 §2。 |
| CLI 用 syn 从 Rust 源码里读 state struct 和 action enum | Rust 是唯一的源头：Swift 的 `SettingsAction`、C# 的 record、C ABI 的 DTO 都由 Rust 类型推出来，不需要手写第二份。 |
| 位置：先放在 `tools/dazzui-compiler/`，是一个独立 Cargo workspace，pnpm 看不到它 | 它是 DazzUI 组件库的消费者，和 `tools/codegen` 地位相近。以后要拆成独立仓库，整个目录搬走即可。 |

---

## 1. Rust DSL 语法

```rust
use dazzui_compiler_macro::ui;

ui! {
    page SettingsPage(state: SettingsState, action: SettingsAction) {
        VStack(spacing: 16) {
            Text("General")
                .style(TextStyle::Title)

            SettingsSection {
                SettingsRow {
                    title: "Launch at startup",
                    trailing: Toggle {
                        value: bind(state.launch_at_startup),
                        on_change: Action::SetLaunchAtStartup,
                    }
                }
            }

            if state.signed_in {
                AccountCard { user: state.user }
            } else {
                Button("Sign in").on_click(Action::SignIn)
            }

            for item in state.items {
                ItemRow { item: item }
            }

            Button("Save")
                .style(ButtonStyle::Primary)
                .disabled(state.saving)
                .on_click(Action::Save)
        }
    }
}
```

语法（EBNF，`Ident` / `Lit` / `Path` 沿用 Rust 的 token）：

```
page      = "page" Ident "(" "state" ":" Path "," "action" ":" Path ")" "{" node "}"
node      = component | if | for
component = Ident [ "(" args ")" ] [ "{" body "}" ] { "." Ident "(" args ")" }
args      = [ arg { "," arg } [","] ]
arg       = Ident ":" value          -- named
          | value                    -- positional，按 registry 的 positional 顺序对位
body      = { Ident ":" value [","]  -- named prop 或 named slot（值是 component 时是 slot）
            | node }                 -- child
value     = Lit                      -- "x" / 16 / true
          | "bind" "(" path ")"      -- 双向绑定，只允许 state 路径
          | "Action" "::" Ident [ "(" value ")" ]   -- action
          | Path "::" Ident          -- 枚举值 TextStyle::Title
          | path                     -- state.x.y / item.x
          | component                -- slot
path      = Ident { "." Ident }      -- 首段必须是 state 或循环变量
if        = "if" cond "{" { node } "}" [ "else" "{" { node } "}" ]
cond      = [ "!" ] path
for       = "for" Ident "in" path "{" { node } "}"
```

区分 body 里的 prop 和 child：`Ident :`（且不是 `::`）就是 prop，其余都是 node。

**刻意不支持的：** 算术、比较、闭包、方法调用、任意 Rust 表达式。DSL 只用来指称值，不负责计算值。

**派生数据由 Rust 计算。** `ui!` 会被编译成 Swift / C#，所以它不能捕获
`let visible = state.items.iter().filter(..)` 这种 Rust 局部变量：到了运行时，
那个迭代器存在于 Rust 进程里，而 SwiftUI 只拿得到快照。派生值写成 state 上的
selector，进入快照：

```rust
impl SettingsState {
    #[dazzui_compiler_macro::selector]
    pub fn visible_items(&self) -> Vec<Item> {
        self.items.iter().filter(|i| i.visible).cloned().collect()
    }
}
// ui! 里：for item in state.visible_items { … }
```

`for` 的元素需要身份：元素类型有 `id` 字段时直接用；否则写 `for item in state.items, key: item.uuid`。

**Phase 3 定稿的规则：**

- `for` 只遍历 state 上的 `Vec<结构体>`（字段或 selector）。标量的 Vec 要先包成带 `id` 的结构体。
  暂不支持 `for` 嵌套 `for`，要嵌套就在 selector 里摊平。
- 循环体里只读循环变量，不读 `state.`：WinUI 的 DataTemplate 里 x:Bind 够不到页面的 ViewModel，
  所以每个元素需要的值由 Rust 的 selector 放到元素上。这条对所有平台都一样，页面才能在每个平台都编得过。
- `if` 的条件是一个 bool 路径（`state.x`、`item.x`），可以取反，可以 `else if`；比较和组合都放到 selector 里。
- `#[selector]` 标在 `impl` 里的 `fn name(&self) -> T` 上；页面把它当字段读，快照时由 Rust 计算后过桥。
  `ui!` 宏也读 crate 的 `src/`，只为了知道路径里哪一段是 selector，要写成方法调用。

---

## 2. proc macro 方案

`ui!` 做三件事，都不产生运行时代码：

1. 用 `dazzui_compiler::syntax` 解析，语法错误变成 `compile_error!` 并标在对应 span 上。
2. 用 registry 做语义检查（未知组件、未知 prop、缺少必填 prop、类型不符、重复 prop、slot 用错），产出带 span 的错误。
3. 生成类型检查代码，让 rustc 核对 DSL 引用的 Rust 名字：

```rust
#[doc(hidden)]
const _: () = {
    #[allow(dead_code, unused)]
    fn __dazzui_compiler_settings_page(state: &SettingsState) {
        let _: bool = state.saving;                                  // .disabled(state.saving)
        let _: bool = state.launch_at_startup;                       // bind(state.launch_at_startup)
        let _: fn(bool) -> SettingsAction = SettingsAction::SetLaunchAtStartup; // on_change
        let _: SettingsAction = SettingsAction::Save;                // on_click
    }
};
```

每一行都用 `quote_spanned!` 带上 DSL 里的 span，所以 `state.savng` 报的是
rustc 自己的 `no field savng on type &SettingsState`，并标在 DSL 那一行。prop
期望的 Rust 类型来自 registry（`Bool` → `bool`）。

宏读不到别的文件，所以 registry 的位置约定为 crate 根目录的
`dazzui-compiler.toml`（通过 `CARGO_MANIFEST_DIR` 找到）；Phase 1–3 用内建 registry。

CLI 不经过 rustc：它 `syn::parse_file` 每个 `.rs`，找到条目位置上的 `ui!` 调用，
把其中的 token 交给同一个 parser。需要开启 proc-macro2 的 `span-locations`，
这样 span 里带行列号，可以填进 `SourceLocation`。

---

## 3. Semantic IR

IR 只描述 UI 的语义：VStack、Text、Toggle、SettingsRow 这类名字。不出现
StackPanel、TextBlock、ToggleSwitch、SwiftUI.Button 这类平台控件名。只读值、
双向绑定、事件 action 是三种不同的类型，都不是字符串。

```rust
pub struct Page {
    pub name: String,                // SettingsPage
    pub state_type: String,          // SettingsState
    pub action_type: String,         // SettingsAction
    pub root: UiNode,
    pub source: SourceLocation,
}

pub enum UiNode {
    Component(Component),
    If { condition: Condition, then_branch: Vec<UiNode>, else_branch: Vec<UiNode>, source: SourceLocation },
    ForEach { collection: StatePath, item: String, key: Option<LocalPath>, children: Vec<UiNode>, source: SourceLocation },
}

pub struct Component {
    pub id: ComponentId,             // 语义名，registry 的 key
    pub props: Vec<UiProp>,          // 只读值
    pub bindings: Vec<UiBinding>,    // 双向
    pub events: Vec<UiEvent>,        // 事件 → action
    pub slots: Vec<UiSlot>,          // 具名子节点
    pub children: Vec<UiNode>,
    pub modifiers: Vec<UiModifier>,
    pub source: SourceLocation,
}

pub struct ComponentId(pub String);

pub struct UiProp     { pub name: String, pub value: Expr, pub source: SourceLocation }
pub struct UiBinding  { pub name: String, pub path: StatePath, pub on_change: ActionRef, pub source: SourceLocation }
// on_change 必填：写回只能经过 Rust。`bind(state.x)` 没有配 `on_change: Action::…` 是编译错误。
// 带值的事件（Toggle 的 change）必须接一个带同类型值的变体，不能悄悄丢掉那个值。
// Integer / 枚举 prop 在 Phase 2 只接字面量；Text 的 style 没有默认值，不写就是平台正文。
pub struct UiEvent    { pub name: String, pub action: ActionRef, pub source: SourceLocation }
pub struct UiSlot     { pub name: String, pub node: Box<UiNode>, pub source: SourceLocation }
pub struct UiModifier { pub name: ModifierId, pub args: Vec<Expr>, pub source: SourceLocation }

pub enum Expr {
    Literal(Value),
    State(StatePath),                // state.user.name
    Local(LocalPath),                // item.title（循环变量）
    EnumCase { ty: String, case: String },  // TextStyle::Title
}

pub enum Value { String(String), Bool(bool), Int(i64) }

pub struct StatePath { pub segments: Vec<String> }  // 不含开头的 state
pub struct LocalPath { pub local: String, pub segments: Vec<String> }

pub struct ActionRef {
    pub variant: String,             // Save / SetLaunchAtStartup
    pub payload: ActionPayload,
}
pub enum ActionPayload {
    None,                            // Action::Save
    EventValue,                      // on_change: Action::SetX —— 事件携带的值作为载荷
    Expr(Expr),                      // Action::Open(item.id)
}

pub enum Condition { Truthy(StatePath), Not(StatePath) }   // Local 同理

pub struct SourceLocation { pub file: String, pub line: u32, pub column: u32 }
```

每个节点都带 `SourceLocation`，诊断、源码映射、生成代码里的 `// source:` 注释都从这里来。

---

## 4. Component registry

格式选 TOML：Cargo 生态的默认格式，有注释，不像 YAML 那样有缩进陷阱。一个
组件只描述语义接口，再给每个平台一段映射。新增组件只需要改 registry，不需要
改 compiler。内建组件在 `src/registry.toml`，随 compiler 一起编进去；app 在 crate
根目录的 `dazzui-compiler.toml` 里增加组件，或按名字整个替换内建组件。`ui!` 宏
经 `CARGO_MANIFEST_DIR` 读同一个文件，并用 `include_bytes!` 让 cargo 在它改动后重编。

```toml
[enum.ButtonStyle]
cases = ["Primary", "Secondary"]

[component.Button]
positional = ["title"]
children = "none"                    # "none" | "many" | { allow = ["SettingsRow"] }
props.title    = { type = "string", required = true }
props.style    = { type = "enum:ButtonStyle", default = "Secondary", modifier = true }
props.disabled = { type = "bool", default = false, modifier = true }   # 也能写成 .disabled(…)
events.click   = { modifier = "on_click" }

# SwiftUI：一个调用，参数按初始化器声明的顺序排，外加一个尾随闭包
[component.Button.swiftui]
call = "DazzUI.Button"
args = [
  { arg = "variant", prop = "style", enum = { Primary = ".filled", Secondary = ".recessed" }, omit = "Secondary" },
  { arg = "enabled", prop = "disabled", invert = true, omit = false },
  { arg = "action", event = "click", fallback = "{}" },
]
trailing = { prop = "title", view = "Text" }   # 或 { children = true } / { slot = "x" }

# WinUI：一个元素、属性、事件，slot 和 children 放进哪个属性
[component.Button.winui]
element = "dazz:Button"
attrs = [
  { attr = "Content", prop = "title" },
  { attr = "Variant", prop = "style", enum = { Primary = "Filled", Secondary = "Recessed" } },
  { attr = "IsEnabled", prop = "disabled", invert = true, omit = false },
]
events.click = { event = "Click", args = "RoutedEventArgs" }

[component.Toggle]
props.value   = { type = "bool", required = true, bindable = true }
events.change = { modifier = "on_change", payload = "bool", binds = "value" }   # value 的写回通道
[component.Toggle.winui]
events.change = { event = "Click", value = "((ToggleButton)sender).IsChecked == true", using = "Microsoft.UI.Xaml.Controls.Primitives" }

[component.SettingsRow]
slots.trailing = {}                  # { required = true } 时必填
[component.SettingsRow.swiftui]
args = [{ arg = "title", prop = "title" }, { arg = "trailing", slot = "trailing" }]   # slot 写成 `trailing: { … }`
[component.SettingsRow.winui]
slots.trailing = { property = "Control" }        # <dazz:PreferenceRow.Control>…</…>

[component.SettingsSection.winui]
children = { property = "Body", wrap = "dazz:PreferenceGroup", wrap_property = "Rows" }
```

一个值从哪里来：`prop`（可带 `enum` 映射、`invert` 取反、`omit`：语义值等于它时平台默认值
已经画对了，省略这个参数）、`event`（SwiftUI 写成闭包，WinUI 写成事件处理器）、`slot`。
prop 被 `bind(…)` 时读的是 state，写回走事件里 `binds` 指向它的那个。

`omit` 只写在平台默认值和语义默认值一致的地方：DazzUI 的 Swift Button 默认就是
recessed，所以省略；C# 的默认是 Filled，所以 WinUI 一侧总是写出 Variant。

VStack、HStack、Text 没有平台表，由各 backend 自己 lower：stack 的布局和 Text
的字体 token 不是参数映射能说清的。只要 app 给它们写了平台表，映射优先。

registry 加载时就校验：未知的键、未知的类型、映射引用了不存在的 prop/event/slot、
slot 和 prop 同名、`binds` 指向非 bindable 的 prop、`allow` 里的名字不是组件、
非 backend 自带的组件缺少 `swiftui` 或 `winui` 表。错误带文件名和组件名。

**registry 的边界：** 平台映射是数据，不是代码。映射写不出来的情况，比如
WinUI 的 `if` 要生成 `x:Load` 加一个生成的属性，属于 backend 自身的逻辑，不进 registry。

---

## 5. Compiler pipeline

```
.rs 文件
  │  syn::parse_file，找条目位置上的 ui!            ── project.rs
  ▼
TokenStream ──syntax──▶ PageAst（带 span）
  ▼
semantic::lower(ast, &Registry, &RustTypes)         ── semantic.rs
  · 解析组件名 → ComponentSpec
  · positional 对位到 prop 名；body/args/modifier 合并后查重
  · 值的分类：Lit/EnumCase/State/Local → Expr；bind() → Binding；Action:: → ActionRef；component → Slot
  · 类型检查：prop 类型 vs Expr；state 路径 vs Rust struct 字段类型；
    action 变体是否存在、载荷类型是否等于事件载荷类型
  · 必填检查、children/slot 合法性
  ▼
ir::Page ─┬─▶ swiftui::generate(&Page, &RustTypes) → Vec<GeneratedFile>
          └─▶ winui::generate(&Page, &RustTypes, namespace) → Vec<GeneratedFile>
```

诊断：semantic 产出 `(span, message, label, help)`；宏把它变成 `syn::Error`，
CLI 用 span 的行列号渲染成 rustc 的样式：

```
error: property `titl` does not exist on component `Button`
  --> src/views/settings.rs:18:13
   |
18 |             titl: "Save"
   |             ^^^^
   |
help: did you mean `title`?
```

拼写建议用 Jaro-Winkler（strsim），阈值 0.8。

**错误一览（Phase 5 定稿）。** 每一类在 `tests/diagnostics/<类>/` 有一份 CLI 输出的快照
（`DAZZUI_BLESS=1` 重写），宏一侧在 `macro/tests/ui/` 有 trybuild 用例（`TRYBUILD=overwrite` 重写）：

| 类 | 例子 | 谁报 |
|---|---|---|
| 语法 | 少了 `action:`、`bind` 里不是路径、一个 page 两个根 | parser：宏和 CLI 同一条 |
| 名字 | 未知组件 / prop / modifier / slot，位置参数过多，重复给值 | semantic，带“did you mean” |
| 类型 | prop 类型不符，枚举 case 不存在或类型不对，Integer/枚举 prop 读 state | semantic |
| state 路径 | 没有这个字段，根不是 `state` 或循环变量，整个 state 当值 | CLI：semantic；宏：rustc 的 E0609 |
| action | 没有这个变体，载荷有无或类型不符，事件值和变体载荷类型不符 | CLI：semantic；宏：rustc 的 E0599/E0308 |
| 绑定 | `bind` 缺 `on_change`，prop 不能绑定，绑定循环变量 | semantic |
| 控制流 | 条件不是 bool，`for` 不是 Vec<结构体>、元素没有 `id`、嵌套 `for`、循环体读 state | semantic |
| slot / children | slot 不是组件、重复、必填缺失，子组件不在 `allow` 里 | semantic |
| 过桥 | 缺 `#[app]`，`#[app]`/`#[selector]` 签名不对，字段或载荷类型过不了桥 | CLI：project + bridge；宏：`#[app]` / `#[selector]` |
| 项目 | 同名 page，两个 page 共用一个 state | project |
| registry | TOML 语法错误（指到行列），映射引用不存在的东西，枚举 case 缺平台代码，带值事件缺 `value` | registry 加载时 |

宏在 stable Rust 上不能给错误挂 help，于是 help 写成消息的下一行 `= help: …`，样子和 rustc 自己的一样。
CLI 的诊断按源码位置排序；终端上用 rustc 的配色（`NO_COLOR` 关掉）。
生成阶段不再 panic：凡是 backend 会碰到的缺口，都在 registry 加载或 semantic 时报出来；
`dazzui-compiler check` 也会把两个 backend 跑一遍再丢掉结果，所以 `check` 过了，`generate` 一定能写出来。

---

## 6. SwiftUI lowering

| IR | SwiftUI |
|---|---|
| `VStack(spacing:)` / `HStack` / `Spacer` | SwiftUI 原生的 `VStack(spacing:)` / `HStack` / `Spacer()` |
| `Text` | `Text("…")`；`.style(TextStyle::X)` → DazzUI 的字体 token |
| DazzUI 组件 | 一律写成 `DazzUI.Button`、`DazzUI.Switch` 这样的全限定名，因为 DazzUI 的 `Button`/`Toggle` 和 SwiftUI 同名 |
| `Expr::State(p)` | `model.state.p`（camelCase） |
| `Binding` | `Binding(get: { model.state.x }, set: { model.send(.setX($0)) })`；组件接受 `value + onChange` 时（DazzUI.Switch 就是）直接传两个参数，不包 Binding |
| `UiEvent` | 尾随闭包 `{ model.send(.save) }` |
| `If` | `if model.state.x { … } else { … }` |
| `ForEach` | `ForEach(model.state.items) { item in … }`（元素 Identifiable，或 `id: \.key`） |
| modifier | 能映射到组件参数的就作为参数（`disabled` → `enabled: !…`），否则用 view modifier（`.padding(16)`） |

生成的文件：

- `SettingsPage.swift`：`struct SettingsPage: View { @ObservedObject var model: SettingsViewModel … }`
- `SettingsViewModel.swift`：由 Rust 类型推出的 `enum SettingsAction`、`struct SettingsState`，以及 `@MainActor final class SettingsViewModel: ObservableObject`，持有 `@Published private(set) var state` 和 `send(_:)`。

文件头统一是 `// AUTO-GENERATED FILE. DO NOT EDIT. Source: …`，每个组件前一行
写 `// source: settings.rs:23`。

---

## 7. WinUI 3 lowering（C#）

| IR | WinUI 3 |
|---|---|
| `VStack(spacing:)` / `HStack` | `<StackPanel Spacing=… [Orientation="Horizontal"]>` |
| `Spacer` | HStack/VStack 中有 Spacer 时整体改成 `Grid`，Spacer 占 `*` 列/行，其余占 `Auto`。这是 WinUI 里表达“撑开”的自然写法 |
| `Text` | `<TextBlock Text=…/>` |
| DazzUI 组件 | `xmlns:dazz="using:DazzUI"`，写成 `<dazz:Button …/>` |
| `Expr::State(p)` | `{x:Bind ViewModel.State.P, Mode=OneWay}` |
| `Binding` | DazzUI 控件的 `IsChecked` 等属性是普通 DP，但写回要经过 Rust：用 `OneWay` 绑定读，写回走生成的事件处理器 `ViewModel.Send(new SetX(value))`。不用 `TwoWay`，因为 `TwoWay` 会绕过 Rust 直接改 C# 侧的状态 |
| `UiEvent` | `Click="OnSaveClick"`，code-behind 里生成 `OnSaveClick`，内容只有一行 `ViewModel.Send(…)` |
| `If` | 分支里每个节点设 `Visibility="{x:Bind local:Page.VisibleN(…), Mode=OneWay}"`，`VisibleN` 是按条件取反组合生成的静态函数，嵌套的 `if` 合成一个。不用 `x:Load`：它要 `x:Name`，在 DataTemplate 里也不好用 |
| `ForEach` | `<ItemsRepeater ItemsSource="{x:Bind ViewModel.State.Items, Mode=OneWay}">`，Layout 用和外层 stack 同方向、同间距的 `StackLayout`，加上 `<DataTemplate x:DataType="local:Item">`；模板里多个节点时包一层同方向的 StackPanel。事件处理器从 `sender` 往上找到 ItemsRepeater，用 `GetElementIndex` 取元素（ItemsRepeater 不设 DataContext）|
| modifier | `disabled` → `IsEnabled`，`padding` → `Padding`/`Margin`，`style` → 组件的枚举属性 |

生成的文件：`SettingsPage.xaml`、`SettingsPage.xaml.cs`（`partial class`，
只有构造函数注入的 ViewModel 和事件转发）、`SettingsViewModel.cs`（
`abstract record SettingsAction` 加嵌套的 `sealed record`，以及实现了
`INotifyPropertyChanged` 的 ViewModel）。

**语义默认值由 registry 决定，不继承平台库的默认值。** 例子：DazzUI 的 Button
默认 variant，React 和 Swift 是 `recessed`，C# 是 `Filled`。registry 定为
`recessed`，WinUI backend 遇到平台默认值和语义默认值不一致时就显式写出
`Variant="Recessed"`。

---

## 8. Rust ↔ native 状态桥

一条窄的 C ABI，不暴露 Rust ABI。由 CLI 根据 Rust 类型生成，Phase 2 起接入：

```c
typedef struct NuiApp NuiApp;                    // opaque handle

NuiApp* settings_create(void);
void    settings_destroy(NuiApp*);

// 快照：Rust 拥有内存，用 free 归还
typedef struct { bool launch_at_startup; bool auto_update; bool saving; } SettingsStateDto;
SettingsStateDto settings_get_state(const NuiApp*);

// action：tag + 联合体载荷
typedef enum { SETTINGS_ACTION_SET_LAUNCH_AT_STARTUP, SETTINGS_ACTION_SET_AUTO_UPDATE, SETTINGS_ACTION_SAVE } SettingsActionTag;
typedef struct { SettingsActionTag tag; union { bool set_launch_at_startup; bool set_auto_update; }; } SettingsActionDto;
void settings_send(NuiApp*, SettingsActionDto);

// 订阅：state 变化后回调，在调用 settings_send 的线程上触发
typedef void (*NuiStateChanged)(void* context);
void settings_subscribe(NuiApp*, NuiStateChanged, void* context);
```

- 字符串和 Vec 用 `{ const uint8_t* ptr; size_t len; }` 返回，内存属于 Rust，配一个 `settings_state_free`。
- 第一版的数据流：`send` → Rust `update` → 回调 → ViewModel 调 `get_state` 取整份快照 → `@Published` / `PropertyChanged` → 平台框架自己做 diff。不做细粒度响应式。
- Swift 侧通过 module map 引入 C 头文件；C# 侧用 `[LibraryImport]` 加 source-generated marshalling。两边的胶水代码由 CLI 一起生成。
- Rust 侧：`#[dazzui_compiler_macro::app]` 标在 `update` 上，生成 `extern "C"` 包装；业务逻辑仍然是普通的 `fn update(&mut State, Action)`。

**Phase 2 落地时定下的细节：**

- ABI 只有一份描述：`src/bridge.rs` 的 `Bridge`。`#[app]` 的 Rust 侧（`Bridge::rust`）、
  C 头文件、Swift 胶水、C# 胶水都从它推出。`#[app]` 和 CLI 一样，经 `CARGO_MANIFEST_DIR`
  读 crate 的 `src/` 拿 state struct 和 action enum；crate 每次编译都会重跑宏，所以不会读到旧类型。
- 命名取自 state 类型：`SettingsState` → 前缀 `settings_`、`SettingsApp`、`SettingsStateDto`。
  CLI 要求每个页面的 state 都有一个 `#[app]`，且它接的 action 就是页面的 action。
- tag 是 `uint32_t`，按变体声明顺序编号，C 里给 `#define`，不用 C `enum`（大小随编译器）。
  没有任何变体带载荷时，DTO 里没有 union。变体最多带一个值。
- 能过桥的类型：`bool`、各宽度整数（`usize`/`isize` → `uintptr_t`/`intptr_t`）、`f32`/`f64`、
  `String`（`NuiStr`）。快照里的字符串属于 Rust，由 `*_state_free` 归还；action 里的字符串
  属于调用方，只借到调用结束。Phase 3 起，嵌套结构体按值过桥，`Vec<结构体>` 以 `{ ptr, len }` 的类型化列表过桥，`state_free` 递归归还；selector 当字段过桥。
- 回调里不持有 app 的借用：先复制订阅者列表再逐个调用，订阅者可以安全地重新 `get_state`。
- Swift：生成 `CSettings/`（头文件 + module map），宿主的 `Package.swift` 用 `.systemLibrary`
  引入，并按完整路径链接 `libsettings_example.a`（`-l` 会选中旁边的 dylib）。Cargo 不由
  SwiftPM 驱动，先 `cargo build -p settings-example`。
- C#：`[LibraryImport]` 加函数指针回调（`[UnmanagedCallersOnly]` + weak `GCHandle`）；DTO 全部
  blittable，`bool` 存成 `byte`。CLI 的 `--library` 默认取 `./Cargo.toml` 的包名。

---

## 9. Workspace 结构

```
tools/dazzui-compiler/
  Cargo.toml                 workspace，也是 dazzui-compiler 这个 package（lib + 二进制）
  DESIGN.md
  src/
    ir/                      IR、组件规格类型、诊断、SourceLocation
    syntax/                  DSL AST + syn parser（宏和 CLI 共用）
    registry.rs semantic.rs  registry、semantic、读取 Rust 类型（rust_types.rs）、项目扫描（project.rs）
    swiftui.rs               IR → .swift
    winui.rs                 IR → .xaml / .xaml.cs / .cs
    generate.rs main.rs      CLI：check / generate
  macro/                     dazzui-compiler-macro：proc macro ui!，以后还有 #[selector]、#[app]
  tests/golden.rs            对 examples/settings/generated 做 golden test
  examples/settings/
    src/state.rs action.rs update.rs    app core：纯 Rust，不依赖任何 UI
    src/views/settings.rs              ui! 页面
    generated/apple/                    CLI 输出，提交进仓库；golden test 核对它
    generated/windows/
    Package.swift                       SwiftPM 宿主：target `SettingsUI` 直接指向 generated/apple
                                        （SwiftPM 的 target 不能出包根，所以它和 Cargo.toml 并排）
    apple/SettingsHost/                 运行生成页面的 macOS app
    apple/SettingsUITests/              xcodebuild test：页面能布局、ViewModel 转发 action
    windows/SettingsHost/               WinUI 3 宿主：直接编译 generated/windows，引用真实的 DazzUI，带上 Rust 的 cdylib；
                                        --smoke-test 用真实鼠标点击走完 XAML → 生成的处理器 → C ABI → update → x:Bind
    windows/smoke.sh                    从 Mac 经 SSH 在 Windows 上同步、构建、在登录的桌面里跑 smoke test，取回结果和截图
```

只有两个 crate：proc macro 的 crate 不能导出别的东西，所以 `ui!` 单独在
`macro/`，其余都是 `dazzui-compiler` 的模块。app 依赖 `dazzui-compiler-macro`。
模块之间的方向不变：`ir ← syntax ← semantic ← {swiftui, winui, macro, cli}`；
backend 只读 `ir`（加上 `RustTypes`），互相不引用。

---

## 10. MVP 范围与阶段

| Phase | 内容 | 验证 |
|---|---|---|
| **1** ✅ | Text、Button、VStack；字面量 prop；children；click → action；读取 Rust action enum | parser/semantic 单测；对 `generated/` 做 golden test；example crate 能编译（宏的类型检查生效）；生成的 Swift 能和真实的 DazzUI 一起用 xcodebuild 编过 |
| **2** ✅ | state 表达式、Toggle、HStack、binding、读取 Rust state struct、`.style` / `.disabled` modifier、带载荷的 action、C ABI 桥 | 宿主 app 点开关后 Rust 的 state 发生变化（xcodebuild test 经 C ABI 往返；Rust 侧单测直接调 `extern "C"`）|
| **3** ✅ | if、for、selector；`Vec<结构体>` 和嵌套结构体过桥 | 宿主 app 渲染 `for` + selector 列表和条件分支；xcodebuild test 验证列表和 selector 经 C ABI 往返 |
| **4** ✅ | TOML registry、自定义组件、SettingsSection/SettingsRow、slot、children 白名单；Button/Toggle 也改为由映射驱动 | 完整的 demo SettingsPage：示例的 `dazzui-compiler.toml` 定义 Settings/SettingsSection/SettingsRow，生成的 SwiftUI 用真实的 DazzUI 偏好设置控件编过并运行 |
| **5** ✅ | rustc 样式的诊断渲染（配色、tab、TOML 行列）、所有类型的错误、`dazzui-compiler check`、trybuild 测试 compile-fail | 12 类 CLI 诊断快照；7 个 trybuild compile-fail 用例 + 1 个 pass；生成阶段的 panic 都改成了 registry/semantic 诊断 |

MVP 之外（不做）：animation、gesture、复杂导航、runtime diff、Virtual DOM、自绘、layout engine、hot reload、accessibility 抽象。

CLI：

```
dazzui-compiler check    [--src <dir>]
dazzui-compiler generate --target swiftui|winui [--src <dir>] [--output <dir>] [--namespace <C# ns>]
```
