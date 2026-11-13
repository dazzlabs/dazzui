"""Join a normal Example.exe trace, guarded SendInput log and DXGI observations."""

import collections
import csv
import itertools
import json
import math
import statistics
import sys
from pathlib import Path


def stats(values):
    values = sorted(values)
    if not values:
        return {"count": 0}
    return {
        "count": len(values),
        "mean_ms": statistics.mean(values),
        "p95_ms": values[math.ceil(len(values) * 0.95) - 1],
        "p99_ms": values[math.ceil(len(values) * 0.99) - 1],
        "max_ms": values[-1],
    }


def analyze(prefix):
    prefix = str(prefix)
    driver = Path(prefix + "-input.csv").read_text().splitlines()
    trace = Path(prefix + "-trace.csv").read_text().splitlines()
    capture = [
        json.loads(line)
        for line in Path(prefix + "-capture.jsonl").read_text().splitlines()
    ]
    if (
        not driver[-1].startswith("complete,")
        or trace[-1] != "complete"
        or capture[-1].get("type") != "complete"
    ):
        raise ValueError(
            "Input, application trace and capture must all finish successfully"
        )
    meta = dict(zip(trace[0].split(",")[::2], map(int, trace[0].split(",")[1::2])))
    driver_meta = dict(
        zip(driver[0].split(",")[::2], map(int, driver[0].split(",")[1::2]))
    )
    frequency = meta["frequency"]
    if (
        meta["overflow"]
        or frequency != driver_meta["frequency"]
        or frequency != capture[0]["frequency"]
        or meta["pid"] != capture[0]["pid"]
    ):
        raise ValueError("Trace overflow or mismatched clocks/processes")
    # Exclude the first two seconds of input as warm-up, retaining the tail.
    start = driver_meta["start_qpc"] + 2 * frequency
    end = int(driver[-1].split(",")[1])
    duration = (end - start) / frequency
    sent = [
        {k: (v if k == "kind" else int(v)) for k, v in r.items()}
        for r in csv.DictReader(driver[1:-1])
    ]
    by_tag = {r["tag"]: r for r in sent}
    records = [{k: int(v) for k, v in r.items()} for r in csv.DictReader(trace[1:-1])]
    records.sort(key=lambda r: r["start_qpc"])
    steady = [r for r in records if start <= r["start_qpc"] <= end]
    input_records = [r for r in steady if r["kind"] == 1]
    pubs = [r for r in steady if r["kind"] == 3]
    paints = [r for r in steady if r["kind"] == 4]
    roots = [r for r in paints if r["detail"] == 1]
    observations = [r for r in capture if r.get("type") == "frame"]
    if (
        not observations
        or observations[0]["present_qpc"] > start
        or observations[-1]["present_qpc"] < end
    ):
        raise ValueError("Capture does not cover steady input interval")
    changed = {r["frame_id"]: r for r in pubs if r["detail"] and r["frame_id"]}
    matched = {}
    for r in observations:
        source = changed.get(r.get("frame_id"))
        if source and r["present_qpc"] >= source["start_qpc"]:
            matched.setdefault(r["frame_id"], r["present_qpc"])
    times = sorted(matched.values())
    intervals = [(b - a) * 1000 / frequency for a, b in itertools.pairwise(times)]
    # Each displayed marker identifies the newest published canvas, including
    # versions superseded before DWM ever observed them.
    displayed_versions = sorted(matched, key=matched.get)
    per_display = [b - a for a, b in itertools.pairwise(displayed_versions)]
    # Native controls can forward one wheel message through multiple procedures;
    # count the earliest entry of each tagged input, with its expected type.
    first_input = {}
    for r in records:
        original = by_tag.get(r["tag"])
        if (
            r["kind"] == 1
            and original
            and r["detail"] == (522 if original["kind"] == "wheel" else 512)
        ):
            first_input.setdefault(r["tag"], r)
    steady_inputs = [r for r in first_input.values() if start <= r["start_qpc"] <= end]
    handled_tags = {r["tag"] for r in steady_inputs}
    wheel_sent = [r for r in sent if r["kind"] == "wheel" and start <= r["qpc"] <= end]
    # Match wheel to the first published state incorporating its input serial,
    # then the first observed canvas at least that new. Non-mutating moves are
    # excluded from this latency metric; their queue/handler times remain below.
    all_inputs = [r for r in first_input.values() if r["detail"] == 522]
    display_by_frame = sorted(matched.items())
    wheel_latency = []
    for r in all_inputs:
        original = by_tag[r["tag"]]
        if not start <= original["qpc"] <= end:
            continue
        target = next(
            (
                p
                for p in pubs
                if p["input_id"] >= r["input_id"] and p["end_qpc"] >= r["start_qpc"]
            ),
            None,
        )
        if target:
            presented = next(
                (
                    t
                    for f, t in display_by_frame
                    if f >= target["frame_id"] and t >= target["end_qpc"]
                ),
                None,
            )
            if presented:
                wheel_latency.append((presented - original["qpc"]) * 1000 / frequency)
    result = {
        "scope": "normal Example.exe + native Application.Run + guarded SendInput, moving pointer and wheel",
        "steady_seconds": duration,
        "metadata": capture[0],
        "received_input_messages": len(input_records),
        "sent_wheels": len(wheel_sent),
        "wheel_send_intervals": stats([
            (b["qpc"] - a["qpc"]) * 1000 / frequency
            for a, b in itertools.pairwise(wheel_sent)
        ]),
        "wheel_tags_not_handled_in_steady_window": sum(
            r["tag"] not in handled_tags for r in wheel_sent
        ),
        "input_queue_delay": stats(
            [
                (r["start_qpc"] - by_tag[r["tag"]]["qpc"]) * 1000 / frequency
                for r in steady_inputs
            ]
        ),
        "input_handler_time": stats(
            [(r["end_qpc"] - r["start_qpc"]) * 1000 / frequency for r in input_records]
        ),
        "native_event_batch_time": stats(
            [
                (r["end_qpc"] - r["start_qpc"]) * 1000 / frequency
                for r in steady
                if r["kind"] == 2
            ]
        ),
        "raster_time": stats(
            [
                (r["end_qpc"] - r["start_qpc"]) * 1000 / frequency
                for r in steady
                if r["kind"] == 6
            ]
        ),
        "blit_time": stats(
            [
                (r["end_qpc"] - r["start_qpc"]) * 1000 / frequency
                for r in steady
                if r["kind"] == 7
            ]
        ),
        "marker_time": stats(
            [
                (r["end_qpc"] - r["start_qpc"]) * 1000 / frequency
                for r in steady
                if r["kind"] == 8
            ]
        ),
        "wheel_tags_not_handled_total": sum(
            r["tag"] not in first_input for r in wheel_sent
        ),
        "publications": len(pubs),
        "publications_per_second": len(pubs) / duration,
        "canvas_paints": len(roots),
        "child_paints": len(paints) - len(roots),
        "changed_published_versions": len(changed),
        "observed_versions": len(matched),
        "superseded_or_unobserved_versions": len(changed) - len(matched),
        "composed_fps": (len(times) - 1) * frequency / (times[-1] - times[0])
        if len(times) > 1
        else 0,
        "composed_frame_intervals": stats(intervals),
        "published_version_steps_between_displayed_frames": dict(
            collections.Counter(per_display)
        ),
        "render_stages": {
            name: stats(
                [
                    r["tag"] / 1_000_000
                    for r in steady
                    if r["kind"] == 9 and r["detail"] == stage
                ]
            )
            for stage, name in {
                0: "evaluate",
                1: "environment",
                2: "layout",
                3: "finalize",
                4: "diff",
                5: "backend",
                6: "scroll.translate",
                10: "prepare",
                11: "draw",
                12: "controls",
                13: "publish",
            }.items()
        },
        "wheel_send_to_composed_state": stats(wheel_latency),
        "observer_accumulated_updates": sum(
            max(0, r["accumulated"] - 1)
            for r in observations
            if start <= r["present_qpc"] <= end
        ),
        "frames_per_second": [
            sum(
                times[0] + i * frequency <= t < times[0] + (i + 1) * frequency
                for t in times
            )
            for i in range(int((times[-1] - times[0]) / frequency))
        ]
        if times
        else [],
        "limitations": [
            "Composed desktop output, not physical scanout.",
            "Input timestamps are from the independent SendInput producer; Windows can coalesce mouse moves.",
            "Wheel latency maps to the first observed newer canvas, not a pixel-level assertion of scroll position.",
        ],
    }
    Path(prefix + "-summary.json").write_text(json.dumps(result, indent=2) + "\n")
    return result


if __name__ == "__main__":
    print(json.dumps(analyze(sys.argv[1]), indent=2))
