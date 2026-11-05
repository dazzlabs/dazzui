"""Click a point inside one uniquely named test window on $DISPLAY."""
import ctypes as c
import sys

x = c.CDLL('libX11.so.6')
t = c.CDLL('libXtst.so.6')
x.XOpenDisplay.argtypes = [c.c_char_p]
x.XOpenDisplay.restype = c.c_void_p
x.XDefaultRootWindow.argtypes = [c.c_void_p]
x.XDefaultRootWindow.restype = c.c_ulong
x.XQueryTree.argtypes = [
    c.c_void_p, c.c_ulong, c.POINTER(c.c_ulong), c.POINTER(c.c_ulong),
    c.POINTER(c.POINTER(c.c_ulong)), c.POINTER(c.c_uint),
]
x.XFetchName.argtypes = [c.c_void_p, c.c_ulong, c.POINTER(c.c_char_p)]
x.XFree.argtypes = [c.c_void_p]
x.XTranslateCoordinates.argtypes = [
    c.c_void_p, c.c_ulong, c.c_ulong, c.c_int, c.c_int,
    c.POINTER(c.c_int), c.POINTER(c.c_int), c.POINTER(c.c_ulong),
]
x.XGetGeometry.argtypes = [
    c.c_void_p, c.c_ulong, c.POINTER(c.c_ulong), c.POINTER(c.c_int),
    c.POINTER(c.c_int), c.POINTER(c.c_uint), c.POINTER(c.c_uint),
    c.POINTER(c.c_uint), c.POINTER(c.c_uint),
]
x.XSync.argtypes = [c.c_void_p, c.c_int]
x.XCloseDisplay.argtypes = [c.c_void_p]
t.XTestFakeMotionEvent.argtypes = [c.c_void_p, c.c_int, c.c_int, c.c_int, c.c_ulong]
t.XTestFakeButtonEvent.argtypes = [c.c_void_p, c.c_uint, c.c_int, c.c_ulong]

display = x.XOpenDisplay(None)
assert display, 'X11 display unavailable'
screen = x.XDefaultRootWindow(display)
root, parent, count = c.c_ulong(), c.c_ulong(), c.c_uint()
children = c.POINTER(c.c_ulong)()
assert x.XQueryTree(display, screen, c.byref(root), c.byref(parent), c.byref(children), c.byref(count))
targets = []
for i in range(count.value):
    name = c.c_char_p()
    if x.XFetchName(display, children[i], c.byref(name)) and name.value:
        if name.value.decode() == sys.argv[1]:
            targets.append(children[i])
        x.XFree(name)
x.XFree(children)
assert len(targets) == 1, f'Expected one test window, found {len(targets)}'
target = targets[0]
dx, dy, width, height, border, depth = c.c_int(), c.c_int(), c.c_uint(), c.c_uint(), c.c_uint(), c.c_uint()
assert x.XGetGeometry(display, target, c.byref(root), c.byref(dx), c.byref(dy), c.byref(width), c.byref(height), c.byref(border), c.byref(depth))
px, py = int(sys.argv[2]), int(sys.argv[3])
assert 0 <= px < width.value and 0 <= py < height.value, 'Point is outside test window'
assert x.XTranslateCoordinates(display, target, screen, px, py, c.byref(dx), c.byref(dy), c.byref(parent))
t.XTestFakeMotionEvent(display, -1, dx.value, dy.value, 0)
if '--move-only' not in sys.argv[4:]:
    t.XTestFakeButtonEvent(display, 1, 1, 0)
    t.XTestFakeButtonEvent(display, 1, 0, 0)
x.XSync(display, 0)
x.XCloseDisplay(display)
