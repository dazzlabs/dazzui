"""Build an original, system-independent color-font regression fixture.

The only character is A: one square layer filled with half-transparent red.
No outlines or font data are copied from another font. Regenerate with Python 3.
Table formats: https://learn.microsoft.com/typography/opentype/spec/
"""

from pathlib import Path
from struct import pack, pack_into, unpack


def words(*values):
    return pack(">" + "H" * len(values), *(v & 0xFFFF for v in values))


def checksum(data):
    data += bytes(-len(data) % 4)
    return sum(unpack(">" + "I" * (len(data) // 4), data)) & 0xFFFFFFFF


tables = {}
head = bytearray(54)
pack_into(">IIIIHH", head, 0, 0x10000, 0x10000, 0, 0x5F0F3CF5, 3, 1000)
pack_into(">hhhhHHhhh", head, 36, 0, 0, 1000, 1000, 0, 8, 2, 0, 0)
tables[b"head"] = bytes(head)
hhea = bytearray(36)
pack_into(">IhhhHhhh", hhea, 0, 0x10000, 1000, 0, 0, 1000, 0, 0, 1000)
pack_into(">hh", hhea, 18, 1, 0)
pack_into(">H", hhea, 34, 3)
tables[b"hhea"] = bytes(hhea)
maxp = bytearray(32)
pack_into(">IHHH", maxp, 0, 0x10000, 3, 4, 1)
pack_into(">H", maxp, 14, 1)
tables[b"maxp"] = bytes(maxp)
tables[b"hmtx"] = words(1000, 0) * 3
# .notdef is empty. The base and its COLR layer share a clockwise square.
square = words(1, 0, 0, 1000, 1000, 3, 0)
square += bytes([1, 1, 1, 1])
square += words(0, 0, 1000, 0, 0, 1000, 0, -1000)
tables[b"glyf"] = square * 2
tables[b"loca"] = words(0, 0, len(square) // 2, len(square))
cmap4 = words(4, 32, 0, 4, 4, 1, 0, 0x41, 0xFFFF, 0,
              0x41, 0xFFFF, -64, 1, 0, 0)
tables[b"cmap"] = words(0, 1, 3, 1) + pack(">I", 12) + cmap4
names = {
    1: "DazzUI Alpha Probe", 2: "Regular", 4: "DazzUI Alpha Probe Regular",
    6: "DazzUIAlphaProbe-Regular", 16: "DazzUI Alpha Probe", 17: "Regular",
}
name_records = b""
name_strings = b""
for name_id, text in names.items():
    encoded = text.encode("utf-16-be")
    name_records += words(3, 1, 0x409, name_id, len(encoded), len(name_strings))
    name_strings += encoded
tables[b"name"] = words(0, len(names), 6 + len(name_records))
tables[b"name"] += name_records + name_strings
os2 = bytearray(78)
pack_into(">HhHH", os2, 0, 0, 1000, 400, 5)
pack_into(">I", os2, 42, 1)
os2[58:62] = b"DAZZ"
pack_into(">HHHhhhHH", os2, 62, 0x40, 0x41, 0x41,
          1000, 0, 0, 1000, 0)
tables[b"OS/2"] = bytes(os2)
tables[b"post"] = pack(">I", 0x30000) + bytes(28)
tables[b"COLR"] = words(0, 1) + pack(">II", 14, 20) + words(1, 1, 0, 1, 2, 0)
tables[b"CPAL"] = words(0, 1, 1, 1) + pack(">I", 14) + words(0)
tables[b"CPAL"] += bytes([0, 0, 255, 128])
count = len(tables)
power = 1 << (count.bit_length() - 1)
font = pack(">IHHHH", 0x10000, count, power * 16,
            power.bit_length() - 1, count * 16 - power * 16)
offset = 12 + count * 16
payload = b""
for tag, data in sorted(tables.items()):
    font += tag + pack(">III", checksum(data), offset, len(data))
    if tag == b"head":
        head_offset = offset
    padded = data + bytes(-len(data) % 4)
    offset += len(padded)
    payload += padded
font = bytearray(font + payload)
pack_into(">I", font, head_offset + 8, (0xB1B0AFBA - checksum(font)) & 0xFFFFFFFF)
Path(__file__).with_suffix(".ttf").write_bytes(font)
