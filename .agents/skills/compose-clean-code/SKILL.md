---
name: compose-clean-code
description: >-
  Enforces strict architecture, performance, Material 3 theming, and quality standards for Jetpack Compose (API 33+).
  Mandates hard limits on composable size/nesting/params, stateless screen/route separation, immutable state collections,
  zero side effects in composition bodies, zero emojis (always Google Material theme icons), type-safe navigation,
  and Compose linting/detekt enforcement.
---

# `compose-clean-code` Skill: Jetpack Compose Architecture, Performance & Quality Standards

Compose-only companion to general Kotlin & Android development rules (package structure, CI, and general Kotlin rules). Material 3, API 33 baseline. Drop in as `CLAUDE.md`, `.cursor/rules`, or `RULES.md`.

---

## 1. Hard Limits

| Metric | Limit | Action if Exceeded |
|---|---|---|
| **Lines per composable** | 40 soft, **60 hard** | Extract sub-composables or child presentation components |
| **Lines per file** | 300 soft, **400 hard** | Split into submodules, feature files, or helper files |
| **Composables per file** | **6–8** | Screen composable + private helper composables only |
| **Composable parameters** | **5 max** (excluding `modifier`) | Group into a dedicated immutable State or Config class |
| **Nesting depth in one composable body** | **3 levels** | Extract child composable at level 3 |
| **Modifier chain length** | **6 calls** | Extract a named `Modifier` extension function |

---

## 2. Zero Tolerance

- **Zero Emojis in UI / Layout**:
  - **Never** use raw emojis as UI icons, button graphics, or status indicators.
  - **Always** use official Google Material Theme icons (`Icons.Rounded.*` or `Icons.Outlined.*` from `androidx.compose.material.icons`) or Material vector drawables.
  - If no specific styling or icon is explicitly defined in the prompt, **always default to standard Material theme icons**.
- **No Business Logic, IO, or Calculation in Composition**:
  - Composables render state and emit events—nothing else. Heavy calculations belong in ViewModel/Domain layers.
- **Missing `modifier: Modifier = Modifier`**:
  - Every composable emitting layout must accept `modifier: Modifier = Modifier` as its **first optional parameter**, applied strictly to the **root** layout element.
- **Passing `ViewModel` into Child Composables**:
  - Only the screen-level `XRoute` touches the `ViewModel`. Child composables receive plain immutable state and lambda callbacks.
- **Mutable Collections in State or Parameters**:
  - Never pass `MutableList`, `ArrayList`, or mutable maps. Use `ImmutableList`/`PersistentList` (`kotlinx.collections.immutable`) or an `@Immutable` wrapper.
- **`mutableStateOf` without `remember`**:
  - Never call `mutableStateOf` inside a composable without wrapping it in `remember` or `rememberSaveable`.
- **Side Effects in Composition Body**:
  - Never launch coroutines, register listeners, or log analytics directly in the composition body. Use effect APIs (§4).
- **`GlobalScope` or Unbound `CoroutineScope`**:
  - Never use `GlobalScope` or instantiate `CoroutineScope(...)` inside a composable. Use `rememberCoroutineScope()` for UI event handlers.
- **Hardcoded Colors, Dimensions, or Raw Strings**:
  - No `Color(0x...)`, raw `.dp`/`.sp` spacing literals, or hardcoded strings outside `MaterialTheme`, `Dimens`, or `stringResource()`.
- **`@Suppress` on Compose Lint Rules**:
  - Never suppress `ComposableNaming`, `ModifierMissing`, `ModifierNotUsedAtRoot`, etc. Fix the underlying code.
- **Stale or Unused `@Preview` Functions**:
  - Keep previews actively maintained and runnable, targeting stateless screen representations.
- **Lazy Lists Without Stable Keys**:
  - `LazyColumn` and `LazyRow` items must define a stable `key` (and `contentType` for heterogeneous lists). Never use list index as key.
- **Nested Scrollables in Same Direction**:
  - Never nest a `LazyColumn` inside a `verticalScroll` container. Use a single lazy list with multiple item types.

---

## 3. State Rules

### Screen / ViewModel Contract:
```kotlin
@Composable
fun TasksRoute(viewModel: TasksViewModel = hiltViewModel()) {
    val state by viewModel.uiState.collectAsStateWithLifecycle()
    TasksScreen(state = state, onEvent = viewModel::onEvent)
}

@Composable
private fun TasksScreen(
    state: TasksUiState,
    onEvent: (TasksEvent) -> Unit,
    modifier: Modifier = Modifier,
) { 
    /* stateless, previewable */ 
}
```

- **Two Layers Per Screen**:
  - `XRoute`: Wires ViewModel, dependency injection, and collects state.
  - `XScreen`: Completely stateless, receives UI state and emits event lambdas. Previews and tests target `XScreen`.
- **Lifecycle-Aware State Collection**:
  - Always use `collectAsStateWithLifecycle()`, never bare `collectAsState()` for UI-facing flows.
- **Single Source of Truth**:
  - One `UiState` sealed interface or data class per screen exposed as `StateFlow<UiState>`.
  - One `UiEvent` sealed interface going back to the ViewModel, or individual lambdas for simple screens.
- **One-Shot Effects**:
  - Navigation, snackbars, and toasts must flow via `Channel` or `SharedFlow` consumed in `LaunchedEffect`, never as transient boolean flags in `UiState` that require manual reset.
- **State Hoisting**:
  - Hoist state to the lowest common parent that reads it. Local UI-only state (focus, transient expanded state) stays local using `remember` or `rememberSaveable`.
- **Process Death & Configuration Resiliency**:
  - Use `rememberSaveable` for user inputs, scroll positions, and selected tabs to survive process death and recreation.
- **Optimized Derivation (`derivedStateOf`)**:
  - Use `derivedStateOf` only when the derived calculation changes **less often** than its inputs (e.g. `listState.firstVisibleItemIndex > 0`). Never wrap trivial calculations.
- **Late State Reading**:
  - Defer reading state until the layout or draw phase by passing lambdas (`{ scrollOffset }`) or using `Modifier.graphicsLayer {}` / `Modifier.drawBehind {}` for high-frequency updates (e.g. scroll offsets, animations).

---

## 4. Side Effects API Matrix

| Need | API | Rules & Guardrails |
|---|---|---|
| **Suspend work on key change / enter** | `LaunchedEffect(key)` | Never use `LaunchedEffect(Unit)` when body depends on changing state; key on the dependency. |
| **Launch coroutine from UI event** | `rememberCoroutineScope()` | For click/gesture callbacks only; never launch during composition pass. |
| **Register / unregister listeners** | `DisposableEffect(key)` | Always implement `onDispose { }` cleanup. |
| **Push Compose state to non-Compose objects** | `SideEffect { }` | Runs after every successful recomposition. |
| **Convert non-Compose data to Compose state** | `produceState(...)` / `collectAsStateWithLifecycle` | Bridges external asynchronous data sources to Compose state. |
| **Capture latest lambda in long-lived effect** | `rememberUpdatedState(newValue)` | Prevents restarting long-running effects when a callback changes. |

- Never keep an effect key of `true` or a constant to fake "run once" unless truly intended once-per-composition lifetime.

---

## 5. Recomposition & Stability

- **Stable Parameters**:
  - Composable parameters must be stable: primitives, `String`, function types, `@Immutable`/`@Stable` classes, and immutable collections (`ImmutableList`).
  - Unstable parameters (e.g. standard `java.util.List`, external third-party classes) force recomposition of the callee.
- **Strong Skipping Mode & Compiler Metrics**:
  - Enable strong skipping mode (default in Kotlin 2.0.20+). Audit with compiler reports:
    ```kotlin
    composeCompiler {
        reportsDestination = layout.buildDirectory.dir("compose_reports")
        metricsDestination = layout.buildDirectory.dir("compose_metrics")
    }
    ```
  - Review `*-composables.txt` to eliminate `restartable` but not `skippable` composables on hot paths.
- **Lambda Stability**:
  - Prefer method references (`viewModel::onEvent`) or `remember`-ed lambdas to prevent reallocations and unnecessary recompositions.
- **Allocation Prevention on Hot Paths**:
  - Never allocate objects in the composition body on hot paths (`listOf(...)`, `Modifier` chains with lambdas, `Brush`, `Paint`) without `remember`.
- **Zero Backwards Writes**:
  - Never write to state that was already read in the same composition pass.

---

## 6. Lists & Scrolling

```kotlin
LazyColumn(contentPadding = PaddingValues(Dimens.md)) {
    items(
        items = tasks, 
        key = { it.id }, 
        contentType = { it.type }
    ) { task ->
        TaskRow(
            task = task, 
            modifier = Modifier.animateItem()
        )
    }
}
```

- **Stable Keys & Content Types**:
  - `key` must be a stable, unique ID (never list index).
  - Provide `contentType` when items have heterogeneous layouts to enable view recycling.
- **State Hoisting**:
  - Hoist `rememberLazyListState()` when parent controls need scroll position or list control.
- **Item Extraction for Skipping**:
  - Extract heavy item contents into standalone composables so Compose can skip unchanged rows individually.
- **Swipe-to-Dismiss**:
  - Use `SwipeToDismissBox` with state remembered per item **key**. Confirm changes in `confirmValueChange`, not by mutating collections in composition.
- **Item Animations**:
  - Use `Modifier.animateItem()` (or `animateItemPlacement` on older Compose) for smooth reordering and insertion animations.
- **Zero Unbounded Height Scrolling**:
  - Never place a `LazyColumn` inside a scrollable container of unbounded height.

---

## 7. Material 3 & Theming

- **Root Theme Hierarchy**:
  - One `AppTheme { }` wrapping `MaterialTheme` at the application root. Child composables consume `MaterialTheme.colorScheme`, `typography`, and `shapes`.
- **Dynamic Color Support (Android 12+)**:
  ```kotlin
  val colorScheme = when {
      dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S ->
          if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
      darkTheme -> DarkColors
      else -> LightColors
  }
  ```
- **Semantic Roles**:
  - Use semantic color tokens (`primary`, `onPrimary`, `surfaceContainer`, `onSurfaceVariant`), never raw color palettes or hardcoded hex values.
- **Standard M3 Components**:
  - Use official Material 3 components (`Button`, `FilledTonalButton`, `Card`, `NavigationBar`, `TopAppBar`, `ModalBottomSheet`). Wrap in `core/designsystem/components/` only when establishing an app-wide variant.
- **Design Tokens & Spacing Scale**:
  - Centralize spacing in a `Dimens` object (`xs = 4.dp`, `sm = 8.dp`, `md = 16.dp`, `lg = 24.dp`, `xl = 32.dp`). Zero stray dp literals.
- **Edge-to-Edge Compliance**:
  - Call `enableEdgeToEdge()` in the host Activity. Handle system bars and insets via `Scaffold` content padding or `WindowInsets` modifiers.
- **Typography & Scalable Text**:
  - Use `MaterialTheme.typography.*` styles with sizes in `sp`. Never wrap text in fixed-height containers that clip during font scaling.
- **Iconography Standards**:
  - **Zero Emojis**: Never use emojis in UI layouts or controls.
  - **Material Icons**: Always use `Icons.Rounded` or `Icons.Outlined` from `androidx.compose.material.icons` (or dedicated vector drawables).
  - **Default Styling**: If no specific styling or icon is defined in the prompt, **always default to standard Material theme icons**.

---

## 8. Android 13 (API 33) Specifics

- **Notification Permission (`POST_NOTIFICATIONS`)**:
  - Request at runtime at the contextual moment the user enables notifications/reminders (never on initial launch) via `rememberLauncherForActivityResult` or `rememberPermissionState`. Provide clear rationale on denial.
- **Predictive Back Navigation**:
  - Opt in via `android:enableOnBackInvokedCallback="true"` in `AndroidManifest.xml`.
  - Use Compose `BackHandler` or `PredictiveBackHandler` rather than overriding Activity `onBackPressed`.
- **Themed App Icon**:
  - Provide a monochrome layer in the adaptive icon definition for Material You dynamic theming.
- **Per-App Language Preferences**:
  - Supply `LocaleConfig` and manage locales via `AppCompatDelegate.setApplicationLocales`.
- **Modern Photo Picker**:
  - Use `ActivityResultContracts.PickVisualMedia` instead of requesting `READ_EXTERNAL_STORAGE`.
- **API Version Gating**:
  - Guard any API call above API 33 behind explicit `Build.VERSION.SDK_INT >= Build.VERSION_CODES...` checks.

---

## 9. Type-Safe Navigation

- **Navigation Compose 2.8+ Type-Safe Routes**:
  - Use `@Serializable` data classes and objects for routes instead of string paths and bundle parsing:
    ```kotlin
    @Serializable
    data class TaskDetailRoute(val taskId: String)
    ```
- **Feature Navigation Encapsulation**:
  - Define routes per feature in `feature/<name>/ui/<Name>Navigation.kt`, exposing `NavGraphBuilder.<name>Screen(...)` and `NavController.navigateTo<Name>(...)`.
  - The top-level `NavHost` only aggregates these modular extension functions.
- **Zero NavController Leaks**:
  - Composables and screens must never receive or hold a `NavController` reference. Screens receive pure lambda callbacks (`onNavigateToDetail: (String) -> Unit`).
- **Pass IDs, Not Complex Data Models**:
  - Pass lightweight IDs across navigation routes. The destination screen's ViewModel loads its own data.

---

## 10. Animations

- **High-Level Declarative APIs**:
  - Prefer `animate*AsState`, `AnimatedVisibility`, `AnimatedContent`, and `updateTransition` over manual `Animatable` unless handling direct gesture tracking.
- **Centralized Animation Specs**:
  - Durations, easings, and spring specs come from centralized animation constants, not per-call literals.
- **Render Phase Animations**:
  - Execute continuous animations in the draw or layout phase (`Modifier.graphicsLayer {}`, `Modifier.offset {}`) to avoid triggering recomposition on every frame.
- **Reduced Motion Support**:
  - Respect system animation scaling and user accessibility preferences when rendering decorative animations.

---

## 11. Previews & Testing

- **Comprehensive Screen Previews**:
  - Every stateless `XScreen` must provide preview variations for:
    - Loading state
    - Content populated state
    - Empty state
    - Error state
  - Include dark-theme and large-font (`fontScale = 1.5f`) previews for complex layouts.
- **Multipreview Annotations**:
  - Define custom multipreview annotations (`@PreviewLightDark`, `@PreviewFontScale`, `@PreviewScreenSizes`) to avoid redundant annotation blocks.
- **Fake Preview Data**:
  - Preview data must originate from static `PreviewData` providers or mock classes, never real network or repository calls.
- **Semantics-First UI Testing**:
  - Target `XScreen` with `createComposeRule()`. Query nodes by accessibility semantics (`onNodeWithText`, `onNodeWithContentDescription`). Use `testTag` strictly as a last resort.
- **ViewModel State Testing**:
  - Test ViewModels using `runTest` and `Turbine` on `uiState`, configuring `Dispatchers.setMain(StandardTestDispatcher())`.
- **Screenshot Regression Testing**:
  - Maintain Paparazzi or Roborazzi snapshot tests for core design system components.

---

## 12. Accessibility (a11y)

- **Descriptive Clickables**:
  - Every icon button and interactive element must provide a clear `contentDescription` (or `null` if purely decorative).
- **Minimum Touch Targets**:
  - All interactive elements must satisfy minimum touch target sizing (≥ 48dp) via `Modifier.minimumInteractiveComponentSize()`.
- **Semantic Grouping**:
  - Group related content using `Modifier.semantics(mergeDescendants = true)`. Clear decorative clutter with `clearAndSetSemantics`.
- **Multi-Modal State Indication**:
  - Never convey state, selection, or status by color alone. Combine color with text labels or iconography.
- **Scaling Audits**:
  - Verify layout integrity with TalkBack enabled and system font scaling set to 200%.

---

## 13. Lint & Automated Enforcement

Configure Compose-specific linting in build scripts:

```kotlin
// build.gradle.kts
dependencies {
    detektPlugins("io.nlopez.compose.rules:detekt:<latest>")
    lintChecks("com.slack.lint.compose:compose-lint-checks:<latest>")
}
```

### Mandatory Error-Level Rules:
- `ModifierMissing`
- `ModifierNotUsedAtRoot`
- `ModifierReused`
- `ViewModelForwarding`
- `MutableParams`
- `UnstableCollections`
- `ComposableNaming`
- `LambdaParameterInRestartableEffect`
- `RememberMissing`
- `ContentEmitterReturningValues`
- `MultipleEmitters`

### CI Grep Gates:
```bash
# Prohibit bare collectAsState
grep -rn "collectAsState()" --include="*.kt" app/src/main && exit 1

# Prohibit GlobalScope
grep -rn "GlobalScope" --include="*.kt" app/src/main && exit 1

# Prohibit raw hex color instantiation outside design system
grep -rnE "Color\(0x" --include="*.kt" app/src/main | grep -v "designsystem/Color.kt" && exit 1
```

---

## 14. Pre-PR Verification Checklist

- [ ] **Architecture**: Screen cleanly split into `XRoute` (ViewModel wiring) and stateless `XScreen`.
- [ ] **Modifiers**: Every layout composable provides `modifier: Modifier = Modifier` as first-optional param, applied to root element only.
- [ ] **Zero Logic in Composition**: No business logic, IO, calculations, or coroutine launches in the composition body.
- [ ] **Zero Emojis**: No emojis used for icons, buttons, or indicators; Google Material Theme icons (`Icons.Rounded/Outlined` or vector drawables) used exclusively.
- [ ] **Default Icon Styling**: Standard Google Material Theme icons used when no explicit custom styling is requested.
- [ ] **Lazy Lists**: Stable `key` (and `contentType` where applicable) provided; no nested same-direction scroll containers.
- [ ] **Stability**: Composable parameters are stable or `@Immutable`; `collectAsStateWithLifecycle()` used on UI flows.
- [ ] **Design Tokens**: All colors, typography styles, spacing, and strings sourced from `MaterialTheme`, `Dimens`, or `stringResource`.
- [ ] **Previews**: Stateless `XScreen` covered by content, loading, empty, error, dark theme, and font scale previews.
- [ ] **Accessibility**: Touch targets ≥ 48dp; all interactive icons possess meaningful `contentDescription`.
- [ ] **Linters & CI**: Zero Compose lint or Detekt warnings, zero `@Suppress` annotations, CI grep gates pass.
- [ ] **Dead Code**: No unused composables, orphaned models, or stale previews.
