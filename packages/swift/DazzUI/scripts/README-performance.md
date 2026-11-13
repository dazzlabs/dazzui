# Windows Gallery interaction measurement

Build the normal Example product in Release with `NATIVEAPI_SWIFT_PATH` pointing at the nativeapi workspace binding. Close older copies before rebuilding. The measurement script launches that product through its ordinary application loop; it does not invoke an XCTest benchmark or call the renderer directly.

```powershell
./measure-interaction.ps1 -NativeApiRoot <nativeapi-workspace> -OutputPrefix <artifact-prefix> -Seconds 20
python ./analyze-interaction.py <artifact-prefix>
```

The driver uses guarded native SendInput: continuous 125 Hz pointer movement interleaved with 60 Hz wheel events inside the Gallery viewport. It checks idle state before starting, derives the viewport from the actual mounted tree, and aborts if another application owns the input point, foreground changes, a mouse button is pressed, or the physical cursor deviates from the scripted path. Do not use the mouse during this roughly 25-second measurement. No keyboard input or clicks are sent.

`SWIFTUI_PORT_TRACE_PATH` enables generic buffered input/publication/paint tracing in the normal Windows backend. The marker is written after root painting. Nativeapi's DXGI duplication probe samples the composed desktop, matching those markers to publication versions. The analyzer discards the first two seconds and reports input queue latency, publications, canvas paints, observed versions, composed frame intervals, and wheel-to-observed-state latency. All three logs must end cleanly and the capture must cover the input interval.

A fast CPU callback or a nominal 60 Hz producer is not proof of 60 fps. Inspect the composed frame interval P99 and per-second counts, and report accumulated observer updates separately. On a 165 Hz monitor, a 60 Hz producer naturally spans approximately 12.12 and 18.18 ms display intervals. The probe observes compositor output, not physical scanout. Wheel latency matches the next observed newer canvas; it is not a pixel-level scroll-position assertion.
