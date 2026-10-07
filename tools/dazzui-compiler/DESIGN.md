# dazzui-compiler：Rust-first 原生 UI 代码生成器

Rust 是 UI 的源语言，SwiftUI 和 WinUI 3 是编译目标。构建时把 `ui!` 写的
页面编译成可读的平台源码；运行时只有真正的 SwiftUI / WinUI 3，加一条 UniFFI
生成的桥回到 Rust（Phase 2–5 时是一条手写的 C ABI，见 §8）。没有运行时 renderer、没有 Virtual DOM、没有自绘控件、没有
layout engine。

DSL 的词汇是 React 的：组件名就是 kit 里的名字，prop 和事件就是 React 的 prop 换成 snake case，
枚举就是 prop 的 TypeScript 联合类型。写法是 Flutter 的：widget 像构造函数一样调用，参数都带名字，
层级用 `child:` / `children: [ … ]`；页面只拿 model，读它的值、调它的方法，就像 `build` 里的 widget。编译器不规定 model 内部怎么写，Elm、MVVM、
Clean Architecture 都可以接在这条接缝后面（§8）。

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
| 组件、prop、事件、枚举的名字一律照 React | React 是这个仓库里所有平台照着画的模型（`specs/components.md`），故事也只用 kit 组件和舞台的 `Row`/`Col`/`Frame`/`Text`（`specs/playgrounds.md`）。编译器另起一套名字，就是第五种设计。`tests/react.rs` 直接读 `.tsx` 核对。 |
| 组件树的写法照 Flutter：构造函数 + 命名参数，`child:` / `children: [ … ]`，`Text("…")`，集合 `if`/`for` | 这是用户选定的风格；名字仍是 React 的，只有结构是 Flutter 的。 |
| `ui!` 页面只声明 model：`page SettingsPage(model: SettingsModel) { … }` | `model.field` 读 model 的值（它 `ValueListenable` 的 `Value`），`model.method` 是事件；宏和 CLI 都从 model 推出 state 类型。 |
| 宏展开成**只在编译期存在的类型检查代码**，运行时零产物 | 交给 rustc 去检查 `model.saving` 是不是 `bool`、`model.set_auto_update` 能不能接 `bool`，错误直接标在 DSL 上，rust-analyzer 也能看到。见 §2。 |
| CLI 用 syn 从 Rust 源码里读 state struct 和 model 的 impl | Rust 是唯一的源头：桥的镜像类型、UniFFI 生成的 Swift / C# 都由 Rust 类型推出来，不需要手写第二份。 |
| 编译器只管 UI 和逻辑之间的接缝，不绑架构；接缝用 Flutter 的说法 | 它是底层能力。页面读一个值、调 model 的方法；model 是任何带 `#[page_model] impl`、实现了 `ValueListenable` 的类型。页面只经过方法的协议碰 model，宿主可以给它别的实现。见 §8。 |
| 位置：先放在 `tools/dazzui-compiler/`，是一个独立 Cargo workspace，pnpm 看不到它 | 它是 DazzUI 组件库的消费者，和 `tools/codegen` 地位相近。以后要拆成独立仓库，整个目录搬走即可。 |

---

## 1. Rust DSL 语法

```rust
use dazzui_runtime::ui;

ui! {
    page SettingsPage(model: SettingsModel) {
        Col(gap: "6", children: [
            Preferences(children: [
                PreferenceSection(label: "General", children: [
                    PreferenceRow(
                        title: "Launch at startup",
                        child: Switch(
                            checked: model.launch_at_startup,
                            on_checked_change: model.set_launch_at_startup,
                        ),
                    ),
                ]),
            ]),
            if model.signed_in {
                Text(model.user_name, strong: true)
            } else {
                Button(on_click: model.sign_in, child: Text("Sign in"))
            },
            for item in model.items {
                Button(on_click: model.open(item.id), child: Text(item.title))
            },
            Row(gap: "2", children: [
                Button(
                    variant: ButtonVariant::Filled,
                    disabled: model.saving,
                    on_click: model.save,
                    child: Text("Save"),
                ),
                Text(model.status, tone: TextTone::Secondary),
            ]),
        ])
    }
}
```

结构是 Flutter 的：widget 像构造函数一样调用，参数都带名字，一个子节点是 `child:`，多个是
`children: [ … ]`，文字是 `Text("…")`，`children` 列表里可以写集合 `if`/`for`。名字是 React 的：
`Button`、`variant`、`on_click`、`ButtonVariant::Filled`。页面只拿 model：`model.field` 读它的值，
`model.method` 是事件调的方法。

语法（EBNF，`Ident` / `Lit` / `Path` 沿用 Rust 的 token）：

```
page      = "page" Ident "(" "model" ":" Path ")" "{" widget "}"
widget    = Ident "(" [ arg { "," arg } [","] ] ")"
arg       = Ident ":" value          -- prop，名字是 React 的 prop 换成 snake case
          | "child" ":" widget       -- 一个子节点
          | "children" ":" "[" [ item { "," item } [","] ] "]"
          | value                    -- 只有 Text 收：第一个参数是它的文字
item      = widget | if | for
value     = Lit                      -- "x" / 16 / true；间距档位是字符串 "3"、"0-5"
          | Path "::" Ident          -- 枚举值 ButtonVariant::Filled
          | path                     -- model.x.y（model 的值）/ item.x / model.save（事件）
          | path "(" value ")"       -- model.pick(item.id)（事件，带一个参数）
          | widget                   -- 给接收组件的 prop：icon: Icon(…)
path      = Ident { "." Ident }      -- 首段是 model 或循环变量
if        = "if" cond "{" items "}" [ "else" ( "{" items "}" | if ) ]
cond      = [ "!" ] path
for       = "for" Ident "in" path [ "," "key" ":" path ] "{" items "}"
```

不是这样写的，parser 直接拒绝，并告诉你该怎么写：花括号里的子节点（→ `children: [ … ]`）、
`children` 里的裸字符串（→ `Text("…")`）、除 `Text` 以外的位置参数（→ 带名字）、`.modifier(…)` 链
（→ 它是一个 prop）、`bind(…)`（→ 受控组件：值和变化分开给）、页面头里的 `state:` 或 `action:`
（→ 只写 `model:`）。`child:` 和 `children:` 也要用对：`PreferenceRow`、`Button` 只收一个 `child`，
`Col`、`Row`、`Preferences` 收 `children`。

**命名规则。** 组件名就是 React kit 里的名字（`Button`、`Switch`、`PreferenceRow`），布局只用 React
示例舞台的 `Row`、`Col`、`Text`。prop 和事件是 React 的 prop 换成 snake case（`onCheckedChange` →
`on_checked_change`）。枚举名是 prop 的 TypeScript 类型名，值换成 PascalCase（`'filled'` →
`ButtonVariant::Filled`）；联合类型没有名字时取组件名加 prop 名（`TextTone`）。React 有、编译器还没画的
prop 列在 registry 的 `unsupported` 里，写了会得到"React 有这个 prop，但还没在原生侧画"的错误，而不是"没有这个 prop"。

**刻意不支持的：** 算术、比较、闭包、任意 Rust 表达式。DSL 只用来指称值，不负责计算值；事件只能调 model 的一个方法。
`model.x` 读的是值的快照：整份值一次过桥，比每个字段一次调用便宜得多。

**派生数据由 Rust 计算。** `ui!` 会被编译成 Swift / C#，所以它不能捕获
`let visible = state.items.iter().filter(..)` 这种 Rust 局部变量：到了运行时，
那个迭代器存在于 Rust 进程里，而 SwiftUI 只拿得到快照。派生值写成 state 上的
selector，进入快照：

```rust
impl SettingsState {
    #[dazzui_runtime::selector]
    pub fn visible_items(&self) -> Vec<Item> {
        self.items.iter().filter(|i| i.visible).cloned().collect()
    }
}
// ui! 里：for item in model.visible_items { … }
```

`for` 的元素需要身份：元素类型有 `id` 字段时直接用；否则写 `for item in state.items, key: item.uuid`。

**规则：**

- `for` 只遍历 state 上的 `Vec<结构体>`（字段或 selector）。标量的 Vec 要先包成带 `id` 的结构体。
  暂不支持 `for` 嵌套 `for`，要嵌套就在 selector 里摊平。
- 循环体里只读循环变量，不读 `model.` 的值（调方法可以）：WinUI 的 DataTemplate 里 x:Bind 够不到页面的 `State`，
  所以每个元素需要的值由 Rust 的 selector 放到元素上。这条对所有平台都一样，页面才能在每个平台都编得过。
- `if` 的条件是一个 bool 路径（`model.x`、`item.x`），可以取反，可以 `else if`；比较和组合都放到 selector 里。
- `Text` 的文字是一个字符串或一个字符串值：`Text("Saved")`、`Text(model.status)`；要拼接就在 selector 里拼。
- `#[selector]` 标在 `impl` 里的 `fn name(&self) -> T` 上；页面把它当字段读，快照时由 Rust 计算后过桥。
  `ui!` 宏也读 crate 的 `src/`，只为了知道路径里哪一段是 selector，要写成方法调用。

---

## 2. proc macro 方案

`ui!` 做三件事，都不产生运行时代码：

1. 用 `dazzui_compiler::syntax` 解析，语法错误变成 `compile_error!` 并标在对应 span 上。
2. 用 registry 做语义检查（未知组件、未知 prop、还没画的 React prop、缺少必填 prop、类型不符、重复 prop、children 用错），产出带 span 的错误。
3. 生成类型检查代码，让 rustc 核对 DSL 引用的 Rust 名字：

```rust
#[doc(hidden)]
const _: () = {
    #[allow(dead_code, unused)]
    fn __dazzui_compiler_check_SettingsPage(model: &SettingsModel) {
        // model.field 读的是这份值
        let state = <SettingsModel as ::dazzui_runtime::ValueListenable>::value(model);
        let _: bool = state.saving;                                         // disabled: model.saving
        let _: fn(&SettingsModel, bool) = SettingsModel::set_auto_update;   // on_checked_change
        let _: fn(&SettingsModel) = SettingsModel::save;                    // on_click
        let _: () = SettingsModel::open(model, { let value = ::core::clone::Clone::clone(&item.id); value });
        let _: &str = &state.status;                                        // Text(model.status)
    }
};
```

每一行都用 `quote_spanned!` 带上 DSL 里的 span，所以 `model.savng` 报的是
rustc 自己的 `no field savng on type SettingsState`，`model.sav` 报的是 rustc 的
"no associated function named `sav`… there is a method `save`"，都标在 DSL 那一行。prop
期望的 Rust 类型来自 registry（`Bool` → `bool`）。

宏读不到别的文件，所以 registry 的位置约定为 crate 根目录的
`dazzui-compiler.toml`（通过 `CARGO_MANIFEST_DIR` 找到）。

CLI 不经过 rustc：它 `syn::parse_file` 每个 `.rs`，找到条目位置上的 `ui!` 调用，
把其中的 token 交给同一个 parser。需要开启 proc-macro2 的 `span-locations`，
这样 span 里带行列号，可以填进 `SourceLocation`。

---

## 3. Semantic IR

IR 只描述 UI 的语义，用 React 的名字：Button、Switch、Row、PreferenceRow。不出现
StackPanel、TextBlock、ToggleSwitch、SwiftUI.Button 这类平台控件名。只读值和事件调用
是两种不同的类型，都不是字符串。没有双向绑定：一个控件的值从不在原生侧自己改，变化交给
model，新值随下一份快照回来。

```rust
pub struct Page {
    pub name: String,                // SettingsPage
    pub state_type: String,          // SettingsState
    pub model_type: String,          // SettingsModel
    pub root: UiNode,
    pub source: SourceLocation,
}

pub enum UiNode {
    Component(Component),
    If { condition: Condition, then_branch: Vec<UiNode>, else_branch: Vec<UiNode>, source: SourceLocation },
    ForEach { collection: StatePath, item: String, element: String, key: LocalPath, children: Vec<UiNode>, source: SourceLocation },
}

pub struct Component {
    pub id: ComponentId,             // React 的组件名，registry 的 key
    pub props: Vec<UiProp>,          // 只读值，默认值已经填好
    pub events: Vec<UiEvent>,        // on_click → model 的方法
    pub slots: Vec<UiSlot>,          // 接收组件的 prop：icon: Icon(…)
    pub children: Vec<UiNode>,
    pub text: Vec<Expr>,             // Text 的文字
    pub source: SourceLocation,
}

pub struct UiProp  { pub name: String, pub value: Expr, pub source: SourceLocation }
pub struct UiEvent { pub name: String, pub call: MethodCall, pub source: SourceLocation }
pub struct UiSlot  { pub name: String, pub node: UiNode, pub source: SourceLocation }

pub enum Expr {
    Literal(Value),
    State(StatePath),                // state.user.name
    Local(LocalPath),                // item.title（循环变量）
    EnumCase { ty: String, case: String },  // ButtonVariant::Filled
}

pub struct MethodCall {
    pub method: String,              // set_auto_update
    pub param: Option<String>,       // on：Swift 用它做参数标签
    pub argument: Argument,
}
pub enum Argument {
    None,                            // on_click: model.save
    EventValue,                      // on_checked_change: model.set_on —— 事件携带的值作为参数
    Expr(Expr),                      // on_click: model.open(item.id)
}
```

DSL 里的 `model.field` 在 IR 里是 `Expr::State`：它读的是 model 的值，生成的原生页面里就是 `state.field`。
带值的事件（Switch 的 `on_checked_change`）必须接一个带同类型参数的方法，不能悄悄丢掉那个值。
枚举 prop 和间距档位只接字面量；Integer prop 在 registry 映射的组件上可以读 state（映射原样传过去）。
每个节点都带 `SourceLocation`，诊断、源码映射、生成代码里的 `// source:` 注释都从这里来。

---

## 4. Component registry

格式选 TOML：Cargo 生态的默认格式，有注释，不像 YAML 那样有缩进陷阱。一个
组件描述它的接口（名字照 React），再给每个平台一段映射。新增组件只需要改 registry，不需要
改 compiler。内建组件在 `src/registry.toml`，随 compiler 一起编进去；app 在 crate
根目录的 `dazzui-compiler.toml` 里增加组件，或按名字整个替换内建组件。`ui!` 宏
经 `CARGO_MANIFEST_DIR` 读同一个文件，并用 `include_bytes!` 让 cargo 在它改动后重编。
平台映射照 SwiftUI 和 WinUI 示例里对同一批故事的映射（`Example/Sources/Example/Lib/Mapping.swift`、
`example/Lib/Mapping.cs`）来写。

```toml
[enum.ButtonVariant]                 # button.tsx 的 ButtonVariant，值换成 PascalCase
cases = ["Recessed", "Normal", "Filled", "Tinted", "Outlined", "Plain"]

[component.Button]
children = "text"                    # "none" | "one" | "text" | "many" | { allow = ["PreferenceRow"] }
props.variant  = { type = "enum:ButtonVariant", default = "Recessed" }
props.size     = { type = "enum:ButtonSize", default = "Small" }
props.tint     = { type = "enum:Tint", default = "Primary" }
props.disabled = { type = "bool", default = false }
events.on_click = {}                 # 带值的写 { payload = "bool" }
unsupported = ["full_width", "shortcut", "type"]   # React 有、还没画的

# SwiftUI：一个调用，参数按初始化器声明的顺序排，外加一个尾随闭包
[component.Button.swiftui]
call = "DazzUI.Button"
args = [
  { arg = "size", prop = "size", enum = { Tiny = ".tiny", Small = ".small", … }, omit = "Small" },
  { arg = "variant", prop = "variant", enum = { Recessed = ".recessed", … }, omit = "Recessed" },
  { arg = "enabled", prop = "disabled", invert = true, omit = false },
  { arg = "action", event = "on_click", fallback = "{}" },
]
trailing = { text = true }           # 或 { children = true } / { prop = "x" } / { slot = "x" }

# WinUI：一个元素、属性、事件，children 和 slot 放进哪个属性
[component.Button.winui]
element = "dazz:Button"
attrs = [
  { attr = "Variant", prop = "variant", enum = { Recessed = "Recessed", … } },
  { attr = "IsEnabled", prop = "disabled", invert = true, omit = false },
]
events.on_click = { event = "Click", args = "RoutedEventArgs" }
children = { text = "Content" }      # 文字 children 写进哪个属性

[component.Switch.winui]
events.on_checked_change = { event = "Click", value = "((ToggleButton)sender).IsChecked == true", using = "…Primitives" }

[component.PreferenceRow]
children = "one"                     # 行的控件，React 里是 children
[component.PreferenceRow.swiftui]
args = [{ arg = "title", prop = "title" }, { arg = "trailing", children = true }]   # children 写成 `trailing: { … }`
[component.PreferenceRow.winui]
children = { property = "Control" }  # <dazz:PreferenceRow.Control>…</…>

[component.PreferenceSection.winui]
children = { property = "Body", wrap = "dazz:PreferenceGroup", wrap_property = "Rows" }
```

一个值从哪里来：`prop`（可带 `enum` 映射、`invert` 取反、`omit`：值等于它时平台默认值
已经画对了，省略这个参数）、`event`（SwiftUI 写成闭包，WinUI 写成事件处理器）、`slot`、`children`。

默认值是 React 的，不继承平台的。`omit` 只写在平台默认值和 React 默认值一致的地方：DazzUI 的
Swift Button 默认就是 recessed，所以省略；C# 的默认是 Filled，所以 WinUI 一侧总是写出 Variant。

Row、Col、Text 没有平台表，由各 backend 照示例画舞台组件的方式自己 lower（§6、§7）：
布局和字的颜色、粗细不是参数映射能说清的。只要 app 给它们写了平台表，映射优先。

registry 加载时就校验：未知的键、未知的类型、映射引用了不存在的 prop/event/slot/children、
名字同时是 prop 和事件或 slot、`unsupported` 里列了已经声明的名字、映射了文字 children 却不收文字、
`allow` 里的名字不是组件、非 backend 自带的组件缺少 `swiftui` 或 `winui` 表、枚举的某个值没有平台代码、
带值的事件在 WinUI 缺 `value`。错误带文件名和组件名。

**和 React 对齐的测试。** `tests/react.rs` 读 kit 和舞台的 `.tsx`：registry 里每个组件都要在 React 里存在，
每个 prop、事件和 `unsupported` 里的名字都要是那个组件的 React prop（Base UI、`<button>` 继承来的几个列在测试里），
每个枚举的值都要等于对应的 TypeScript 联合类型。自造一个名字，或 React 改了名而 registry 没跟上，这个测试都会失败。

**registry 的边界：** 平台映射是数据，不是代码。映射写不出来的情况，比如
WinUI 的 `if` 要生成 `Visibility` 绑定，属于 backend 自身的逻辑，不进 registry。

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
  · 每个 prop 按名字归类：React prop → Expr；事件 → MethodCall；接收组件的 prop → Slot；
    unsupported 里的 → "React 有但还没画"；其余 → 未知，带拼写建议和 snake case 提示
  · 类型检查：prop 类型 vs Expr；state 路径 vs Rust struct 字段类型；
    方法是否在 `#[page_model] impl` 里、参数有无和类型是否等于事件的值；页面的 state 是否等于 model 的 Value
  · 必填检查、children 合法性（文字 / 一个 / 多个 / allow）、默认值填进 IR
  ▼
ir::Page ─┬─▶ swiftui::generate(&CompiledPage) → 页面
          └─▶ winui::generate(&CompiledPage, namespace) → 页面
Bridge   ─┬─▶ swiftui::model(&CompiledModel) → 方法协议 + store
          └─▶ winui::model(&CompiledModel, namespace) → 方法接口 + store
```

诊断：semantic 产出 `(span, message, label, help)`；宏把它变成 `syn::Error`，
CLI 用 span 的行列号渲染成 rustc 的样式：

```
error: unknown property `varient` on component `Button`
  --> src/views/settings.rs:18:20
   |
18 |             Button(varient: ButtonVariant::Filled, child: Text("Save"))
   |                    ^^^^^^^
   |
help: did you mean `variant`?
```

拼写建议用 Jaro-Winkler（strsim），阈值 0.8。

**错误一览（Phase 5 定稿）。** 每一类在 `tests/diagnostics/<类>/` 有一份 CLI 输出的快照
（`DAZZUI_BLESS=1` 重写），宏一侧在 `macro/tests/ui/` 有 trybuild 用例（`TRYBUILD=overwrite` 重写）：

| 类 | 例子 | 谁报 |
|---|---|---|
| 语法 | 页面头写 `state:`/`action:`、花括号里的子节点、`children` 里的裸字符串、`.modifier(…)`、`bind(…)`、一个 page 两个根 | parser：宏和 CLI 同一条，告诉你该怎么写 |
| 名字 | 未知组件 / prop，camelCase 写法，重复给值，除 `Text` 外的位置参数，`child:` 和 `children:` 用反 | semantic，带"did you mean" |
| React | React 有、还没在原生侧画的 prop（`full_width`、`default_checked`、`justify`） | semantic |
| 类型 | prop 类型不符，枚举值不存在或类型不对，间距档位不存在，枚举 prop 或间距读 state | semantic |
| 值路径 | 没有这个字段，根不是 `model` 或循环变量，整个 model 当值 | CLI：semantic；宏：rustc 的 E0609 |
| 方法 | model 没有这个方法，参数有无或类型不符，事件值和参数类型不符，事件的值不是 `model.…` | CLI：semantic；宏：rustc 的 E0599/E0308 |
| 控制流 | 条件不是 bool，`for` 不是 Vec<结构体>、元素没有 `id`、嵌套 `for`、循环体读 state | semantic |
| slot / children | 接收组件的 prop 给了别的、必填缺失，只收一个 child 却给了两个，子组件不在 `allow` 里 | semantic |
| 过桥 | `#[page_model]` 标的不是 impl、公开方法形状不对、缺 `new()`、model 不是 `ValueListenable`，参数或字段类型过不了桥，两个 model 导出同名的 store | CLI：project + bridge；宏：`#[page_model]` / `#[selector]` |
| 项目 | 同名 page | project |
| registry | TOML 语法错误（指到行列），映射引用不存在的东西，枚举值缺平台代码，带值事件缺 `value` | registry 加载时 |

宏在 stable Rust 上不能给错误挂 help，于是 help 写成消息的下一行 `= help: …`，样子和 rustc 自己的一样。
CLI 的诊断按源码位置排序；终端上用 rustc 的配色（`NO_COLOR` 关掉）。
生成阶段不再 panic：凡是 backend 会碰到的缺口，都在 registry 加载或 semantic 时报出来；
`dazzui-compiler check` 也会把两个 backend 跑一遍再丢掉结果，所以 `check` 过了，`generate` 一定能写出来。

---

## 6. SwiftUI lowering

| IR | SwiftUI |
|---|---|
| `Row(gap:, align:)` | `HStack(alignment:, spacing: theme.vars.spacing3)`：flex 行默认居中对齐，HStack 也是 |
| `Col(gap:, align:)` | `VStack(alignment: .leading, spacing: …)`；`align` 默认 stretch 时整列 `.frame(maxWidth: .infinity, alignment: .leading)`，像网页上的块一样占满宽度 |
| `Text(tone:, strong:, tint:) { … }` | `Text(…)`，几段用 `+` 连起来；`secondary` → `theme.vars.colorContentSecondary`，`tint` → `theme.vars.ramp(.x)[500]`，`strong` → `labelStrong` 的字重。和示例画舞台 `Text` 的方式一样 |
| DazzUI 组件 | 一律写成 `DazzUI.Button`、`DazzUI.Switch` 这样的全限定名，因为 DazzUI 的 `Button`/`Toggle` 和 SwiftUI 同名 |
| `Expr::State(p)` | `state.p`（camelCase） |
| `UiEvent` | 闭包 `{ model.save() }`、`{ model.setAutoUpdate(on: $0) }`：参数标签是 UniFFI 给的，即 Rust 参数名的 camelCase |
| `If` | `if state.x { … } else { … }` |
| `ForEach` | `ForEach(state.items, id: \.id) { item in … }` |

生成的文件：

- 每个页面一个 `SettingsPage.swift`：`struct SettingsPage: View { let state: SettingsState; let model: any SettingsActions … }`。
  页面只经过方法的协议碰 model，不知道它是不是 Rust 的。
- 每个 page model 一个 `SettingsActions.swift`：`protocol SettingsActions { func save(); func setAutoUpdate(on: Bool) … }`，
  不依赖 UniFFI，纯 Swift 的宿主或测试可以自己实现。
- 每个 page model 一个 `SettingsStore.swift`：`extension SettingsModel: SettingsActions {}`（UniFFI 的对象本来就有这些方法），
  加上 `@MainActor final class SettingsStore: ObservableObject`，持有 `model` 和 `@Published private(set) var state`；
  宿主写 `SettingsPage(state: store.state, model: store.model)`，就像 `ListenableBuilder` 交给 widget 的东西。

文件头统一是 `// AUTO-GENERATED FILE. DO NOT EDIT. Source: …`，每个组件前一行
写 `// source: settings.rs:23`。

---

## 7. WinUI 3 lowering（C#）

| IR | WinUI 3 |
|---|---|
| `Row` / `Col` | `<StackPanel [Orientation="Horizontal"] Loaded="OnGap3Loaded">`；间距是主题的 token，不是 XAML 资源，所以 code-behind 在元素加载时从 `Appearance.GetTheme(this).Number("spacing.3")` 取。`align` 写成每个子元素的 `VerticalAlignment`（Row）或 `HorizontalAlignment`（Col），stretch 不写 |
| `Text` | `<TextBlock TextWrapping="Wrap" Text=…/>`，几段写成 `<Run>`；有 tone/strong/tint 时加 `Loaded="OnTextNLoaded"`，在加载时设主题的颜色和字重。和示例画舞台 `Text` 的方式一样 |
| DazzUI 组件 | `xmlns:dazz="using:DazzUI"`，写成 `<dazz:Button …/>`；Button 的文字写进 `Content` |
| `Expr::State(p)` | `{x:Bind State.P, Mode=OneWay}` |
| `UiEvent` | `Click="OnSaveClick"`，code-behind 里生成 `OnSaveClick`，内容只有一行 `Model?.Save()` / `Model?.SetAutoUpdate(((ToggleButton)sender).IsChecked == true)`。控件的值用 `OneWay` 读，从不 `TwoWay`：变化交给 model，新值随快照回来 |
| `If` | 分支里每个节点设 `Visibility="{x:Bind local:Page.VisibleN(…), Mode=OneWay}"`，`VisibleN` 是按条件取反组合生成的静态函数，嵌套的 `if` 合成一个。不用 `x:Load`：它要 `x:Name`，在 DataTemplate 里也不好用 |
| `ForEach` | `<ItemsRepeater ItemsSource="{x:Bind State.Items, Mode=OneWay}" Loaded="OnRepeaterGap3Loaded">`，Layout 用和外层 stack 同方向的 `StackLayout`，间距加载时从主题取，加上 `<DataTemplate x:DataType="local:Item">`；模板里多个节点时包一层同方向的 StackPanel。事件处理器从 `sender` 往上找到 ItemsRepeater，用 `GetElementIndex` 取元素（ItemsRepeater 不设 DataContext）|

`Loaded` 处理器在 DataTemplate 里也能用，所以间距和字的样式在循环里外一样处理。它们只在加载时设一次，
不跟随运行中的主题切换，和 Phase 1–5 的做法一样。

生成的文件：每个页面一个 `SettingsPage.xaml` 和 `SettingsPage.xaml.cs`（`partial class`，只有
依赖属性 `State`、`ISettingsActions? Model`、事件处理器和加载处理器）；每个 page model 一个
`SettingsActions.cs`（`interface ISettingsActions`，不依赖 UniFFI）和 `SettingsStore.cs`（实现
`INotifyPropertyChanged`，持有 `SettingsModel`，`Actions` 是转发到它的 `ISettingsActions`）。宿主写
`new SettingsPage { State = store.State, Model = store.Actions }`，并在 `PropertyChanged` 里把新快照
赋给 `page.State`。

---

## 8. 接缝：页面、page model 与桥

编译器是底层能力，不规定 app 怎么组织逻辑。它只管页面和驱动页面的东西之间的那条接缝，
用 Flutter 的说法：页面读一个值、调 model 的方法，就像 `ValueListenableBuilder` 下的 widget。

| Flutter | dazzui |
|---|---|
| `ChangeNotifier`（`addListener` / `removeListener` / `notifyListeners`） | `dazzui_runtime::ChangeNotifier`（`add_listener` / `remove_listener` / `notify_listeners`） |
| `ValueListenable<T>.value` | `trait ValueListenable { type Value; fn value(); fn notifier(); }` |
| `ValueNotifier<T>` | `ValueNotifier<T>`：一个整体设置的值 |
| model 的方法，`onPressed: model.save` | `#[page_model] impl` 里的公开方法，`on_click: model.save` |
| `ListenableBuilder` 把值交给 widget | 生成的 `SettingsStore` 把 `state` 和 `model` 交给页面 |

```rust
pub struct SettingsModel { state: Arc<ValueNotifier<SettingsState>>, /* … */ }

#[page_model]                       // 公开方法就是页面能调的；无参的 new() 是原生侧的构造函数
impl SettingsModel {
    pub fn new() -> Self { … }
    pub fn set_auto_update(&self, on: bool) { self.state.update(|s| s.auto_update = on) }
    pub fn save(&self) { … }        // 可以在别的线程上完成，再 notify
    pub fn toggle_plugin(&self, id: u32) { … }
}

impl ValueListenable for SettingsModel {
    type Value = SettingsState;
    fn value(&self) -> SettingsState { self.state.value() }
    fn notifier(&self) -> &ChangeNotifier { self.state.notifier() }
}
```

公开方法的形状只有两种：`pub fn name(&self)` 和 `pub fn name(&self, value: T)`，不返回值；
别的写成私有，或者放进另一个没标 `#[page_model]` 的 impl。`value`、`add_listener`、`remove_listener`、
`with_model` 是桥给原生对象用的名字，方法不能叫这些。

`#[page_model]` 在 impl 旁边生成（示例里的形状）：

```rust
#[derive(uniffi::Record)] pub struct Plugin { pub id: u32, pub name: String, pub enabled: bool }
#[derive(uniffi::Record)] pub struct SettingsState {      // 快照：字段 + selector
    pub launch_at_startup: bool, /* … */ pub visible_plugins: Vec<Plugin>, pub has_hidden_plugins: bool,
}
#[uniffi::export(with_foreign)] pub trait SettingsListener: Send + Sync { fn changed(&self); }
#[derive(uniffi::Object)] pub struct SettingsModel { model: crate::model::SettingsModel }
#[uniffi::export] impl SettingsModel {
    #[uniffi::constructor] pub fn new() -> Arc<Self>;   // crate::model::SettingsModel::new()
    pub fn value(&self) -> SettingsState;               // selector 在这里算
    pub fn add_listener(&self, listener: Arc<dyn SettingsListener>) -> u64;
    pub fn remove_listener(&self, id: u64);
    pub fn set_auto_update(&self, on: bool);            // 逐个转发 app 的方法
    pub fn save(&self);
    /* … */
}
// 不导出：SettingsModel::with_model(model)，给 Rust 测试用
```

- **桥不持有状态，也不跑逻辑。** 原生对象和 app 的 model 同名，只转发方法、读值、挂监听。model 的
  state 类型从它的 `impl ValueListenable`（`type Value = …;`）读出；桥的其他名字由 model 名去掉 `Model`
  推出（`SettingsStore`、`SettingsActions`、`SettingsListener`），所以两个 model 不能推出同一个名字。
- **页面不需要 Rust model。** 页面只经过 `SettingsActions` 协议 / `ISettingsActions` 接口碰 model。
  生成的 store 交给它 UniFFI 的对象；纯原生的宿主或测试可以自己实现这个协议。
- **Elm 也能接。** `Reducer<S, A>` 是一个 `ValueListenable`，`dispatch(action)` 跑 `update` 再通知；
  model 的方法 dispatch 到它，`value`/`notifier` 转给它（`counter_example`）。
- **镜像类型。** app 的类型不派生任何 UniFFI 的东西，`#[page_model]` 为它们生成同名的镜像
  并只导出镜像，所以原生侧看到的名字和 Rust 里一样，app 仍然是不认识任何 UI 的纯 Rust。
  镜像引用 app 类型时用完整路径（`crate::state::Plugin`），在哪个模块展开都能编过。
- **crate 根上要有 `uniffi::setup_scaffolding!();`**（宏不能替你写到那里）；缺了，`#[page_model]` 直接报错。
  app 依赖 `dazzui-runtime`（它重新导出三个宏）和 `uniffi`。
- **数据流**：页面调方法 → model 自己处理 → model 在任意线程 `notify_listeners()` → listener 回调 →
  store 再取一份值 → `@Published` / `PropertyChanged`。listener 在 store 自己的线程（Swift 是主线程，
  C# 是建 store 的那个 DispatcherQueue）上就当场刷新，所以同步改值的方法返回时页面已经显示新值；
  在别的线程上就投递回去再刷新。listener 弱引用 store，因为 Rust 持有 listener。
- 能过桥的类型暂时还是：`bool`、各宽度整数、`f32`/`f64`、`String`、嵌套结构体、`Vec<结构体>`；
  方法最多带一个参数。UniFFI 本身支持 `Option`、带数据的枚举、`Result`、`HashMap`，以后放开只需改
  `bridge.rs` 的检查和镜像生成。
- **生成。** `dazzui-compiler generate` 先 `cargo build` 出 cdylib，再从库里读出导出：Swift 在进程内调用
  `uniffi_bindgen`，写出 `<lib>.swift` 和 C 模块 `<lib>FFI/`；C# 调用 `uniffi-bindgen-cs`，写出 `<lib>.cs`，
  namespace 和库名由 CLI 传入，和页面一致。`ui!` 宏依赖编译器时关掉 `bindgen` feature，不编译 bindgen。
- **每次构建都重新生成。** Xcode 的构建前脚本、MSBuild 的 `GenerateBridge` target 都先跑 `generate`。
  这是必须的：UniFFI 的校验和只覆盖函数签名，不覆盖 record 的字段。原型里实测，Rust 里交换两个
  bool 字段的顺序而不重新生成绑定，旧的 Swift 会悄悄把两个值读反；加一个字段则在解码时
  `bufferOverflow` 崩溃。只有构建时重新生成，才能保证绑定和库来自同一份 Rust。
- **panic** 在边界上被 UniFFI 捕获，变成宿主语言的错误；不抛错的函数在 Swift 侧是 `try!`，会带着 panic
  信息崩溃，而不是未定义行为。
- golden test 只核对编译器自己写的文件（页面、协议和 store），UniFFI 的输出由上一条保证新鲜。

**示例。** `settings_example` 的 `SettingsModel` 是手写的，保存放在自己的线程上完成（演示异步通知和切回
UI 线程）。`counter_example` 只有 Rust，没有宿主，用 `Reducer` 驱动一个页面，证明 Elm 那条路能编译、能跑。
`tests/protocol_only` 核对页面只经过协议碰 model。

**历史。** Phase 1–5 和 UniFFI 迁移时，入口是 `#[app] fn update(&mut State, Action)`：桥自己持有
`Mutex<State>` 并调用 `update`，页面绑死在生成的 `SettingsViewModel` 上，等于把 Elm 架构写进了编译器。
2026-10-05 先拆成 `(state, send)` 加 `PageModel`，随即改成现在的 Flutter 写法：action 枚举去掉，事件直接调
model 的方法；同时组件和 prop 的名字改成 React 的（此前是自造的 `Toggle { value, on_change }`、
`Button("Save").style(ButtonStyle::Primary)`、`VStack(spacing: 24)`、`.modifier(…)` 链、`bind(…)`）。

**历史。** Phase 2–5 用的是一条手写的 C ABI（`#[repr(C)]` DTO、`NuiStr`、C 头文件、`LibraryImport`）。
2026-10-05 用原型验证后迁到 UniFFI：现有生成的页面一行不改就能跑在 UniFFI 的类型上（WinUI smoke test
13/13），换来 `Option`/枚举/`Result`/async 的类型覆盖、Android 的 Kotlin 绑定、panic 处理，并和
dazzcal、dazzclock、beyondtranslate 用同一套桥。代价是生成的绑定大一个数量级，C# 依赖第三方生成器，
复合类型多一次序列化。

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
  macro/                     dazzui-compiler-macro：proc macro ui!、#[page_model]、#[selector]
  runtime/                   dazzui-runtime：app 依赖的那一个 crate，ChangeNotifier、ValueListenable、ValueNotifier、Reducer，并重新导出三个宏
  tests/golden.rs            对示例的 macos/Generated、windows/Generated 做 golden test
  tests/diagnostics/         每类错误一份 CLI 输出快照
  tests/protocol_only/       页面只经过方法的协议碰 model
  tests/react.rs             registry 的名字和 React 的 .tsx 逐项核对
  examples/counter_example/            只有 Rust：Reducer 驱动的页面，测试经 UniFFI 对象跑一遍
  examples/settings_example/           一个示例是一个工程，像 Flutter 的 example/：共享的部分在根上，平台是子目录
    Cargo.toml                         settings-example：app core（相当于 Flutter 的 lib/）
    src/state.rs model.rs              纯 Rust，不依赖任何 UI；model.rs 是手写的 model：#[page_model] impl 和 ValueListenable
    src/views/settings.rs              ui! 页面
    macos/
      project.yml                      XcodeGen 的输入；SettingsExample.xcodeproj 由它生成并提交
      Generated/                       `generate --target swiftui` 的输出：页面、方法协议、store、UniFFI 的 Swift 和 C 模块，提交
      SettingsExample/                 app：编译 Generated/，链接 Rust 静态库；构建前的脚本阶段先跑 generate
      SettingsExampleTests/            页面能布局、方法经 UniFFI 改变 Rust 的 state、后台线程的变化回到主线程、页面不需要 Rust model
    windows/
      SettingsExample.sln
      Generated/                       `generate --target winui` 的输出：页面、方法接口、store、uniffi-bindgen-cs 的 C#，提交
      SettingsExample/                 WinUI 3 app：构建前先跑 generate，编译 ../Generated，引用真实的 DazzUI，带上 Rust 的 cdylib；
                                       --smoke-test 用真实鼠标点击走完 XAML → 生成的处理器 → Model → store → UniFFI → model → x:Bind
      smoke.sh                         从 Mac 经 SSH 在 Windows 上同步、构建、在登录的桌面里跑 smoke test，取回结果和截图

生成的代码放在用它的平台目录里，和 Flutter 把插件注册代码放进 `macos/`、`windows/` 一样。
`dazzui-compiler generate` 不带 `--target` 时，给存在的每个平台目录各生成一份；
C# namespace 默认取包名（`settings-example` → `SettingsExample`）。以后加 iOS / Android，就是加 `ios/`、`android/`。
```

编译器本身两个 crate：proc macro 的 crate 不能导出别的东西，所以宏单独在
`macro/`，其余都是 `dazzui-compiler` 的模块。app 只依赖 `runtime/`：它很小，不带 syn，只有接缝的类型，
并重新导出宏。
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
| **Flutter 结构** ✅ | 组件树改成 Flutter 的构造函数写法：命名参数，`child:` / `children: [ … ]`，`Text("…")`，集合 `if`/`for`；页面只拿 model，`model.field` 读值 | xcodebuild test 6/6；WinUI smoke 待 Windows 机器上线后重跑 |
| **React + Flutter** ✅ | 组件、prop、事件、枚举改用 React 的名字（`tests/react.rs` 读 `.tsx` 核对），舞台的 Row/Col/Text 照示例画；事件调 model 的方法，`ChangeNotifier`/`ValueListenable`/`ValueNotifier` 替代 `PageModel`；页面经方法协议碰 model | xcodebuild test 6/6（含纯 Swift 实现协议），WinUI smoke 15/15（含 Row 间距取自主题）；诊断 12 类、trybuild 7+1 重写 |
| **接缝** ✅ | 页面改成 `(state, send)`；`PageModel` + `#[page_model]` 替代 `#[app]`；桥包装 app 的 model；原生 store 切回 UI 线程；`Reducer` 降为可选 | 示例的手写 model 在后台线程保存：xcodebuild test 6/6，WinUI smoke 14/14；`counter_example` 经 UniFFI 对象跑 Reducer；无 model 页面的 golden |
| **5** ✅ | rustc 样式的诊断渲染（配色、tab、TOML 行列）、所有类型的错误、`dazzui-compiler check`、trybuild 测试 compile-fail | 12 类 CLI 诊断快照；7 个 trybuild compile-fail 用例 + 1 个 pass；生成阶段的 panic 都改成了 registry/semantic 诊断 |

MVP 之外（不做）：animation、gesture、复杂导航、runtime diff、Virtual DOM、自绘、layout engine、hot reload、accessibility 抽象。

CLI：

```
dazzui-compiler check    [--src <dir>]
dazzui-compiler generate --target swiftui|winui [--src <dir>] [--output <dir>] [--namespace <C# ns>]
```
