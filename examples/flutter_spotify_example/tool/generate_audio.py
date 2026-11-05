"""Rebuild the six original 24-second ambient demo tracks (Python stdlib only).

These synthesized recordings are part of this example and share its MIT licence.
No sampled, downloaded or Spotify audio is used.
"""

from array import array
from math import exp, pi, sin
from pathlib import Path
import sys
import wave


RATE = 22050
SECONDS = 24
TRACKS = [
    ("first_light", (48, 55, 59, 64), (72, 76, 79, 83, 79, 76, 74, 71)),
    ("weightless", (50, 57, 60, 65), (77, 81, 84, 81, 79, 77, 76, 72)),
    ("slow_motion", (45, 52, 55, 59), (71, 74, 76, 79, 76, 74, 71, 67)),
    ("warmth", (53, 60, 64, 69), (81, 84, 88, 84, 81, 79, 76, 72)),
    ("night_swim", (43, 50, 53, 57), (69, 72, 74, 77, 74, 72, 69, 65)),
    ("somewhere_else", (48, 55, 62, 67), (79, 83, 86, 83, 81, 79, 76, 74)),
]


def frequency(note):
    return 440 * 2 ** ((note - 69) / 12)


def render(name, chord, melody):
    bass = [frequency(note) for note in chord]
    lead = [frequency(note) for note in melody]
    samples = array("h")
    for frame in range(RATE * SECONDS):
        t = frame / RATE
        # Slow pad with a gentle stereo-independent pulse, plus a bell melody.
        pad = sum(
            sin(2 * pi * f * t + 0.16 * sin(2 * pi * 0.12 * t))
            + 0.18 * sin(2 * pi * f * 2 * t)
            for f in bass
        ) / len(bass)
        beat = t % 1.5
        note = lead[int(t / 1.5) % len(lead)]
        bell = (
            sin(2 * pi * note * beat)
            + 0.23 * sin(2 * pi * note * 2 * beat)
        ) * (1 - exp(-beat * 60)) * exp(-beat * 3)
        envelope = min(1, t / 1.2, (SECONDS - t) / 1.8)
        value = (pad * 0.28 + bell * 0.14) * envelope
        samples.append(round(max(-1, min(1, value)) * 32767))
    if sys.byteorder != "little":
        samples.byteswap()
    target = Path(__file__).resolve().parent.parent / "assets" / "audio"
    target.mkdir(parents=True, exist_ok=True)
    with wave.open(str(target / f"{name}.wav"), "wb") as output:
        output.setparams((1, 2, RATE, 0, "NONE", "not compressed"))
        output.writeframes(samples.tobytes())


if __name__ == "__main__":
    for track in TRACKS:
        render(*track)
    print(f"Wrote {len(TRACKS)} original tracks ({SECONDS}s, mono PCM, {RATE} Hz).")
