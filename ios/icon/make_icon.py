"""Draws the RawPad app icon (original work, MIT OR Apache-2.0). Run: python3 make_icon.py
Writes AppIcon-1024.png and the iPad sizes the app bundle needs."""
from PIL import Image, ImageDraw, ImageFilter

S = 1024
img = Image.new("RGB", (S, S))
d = ImageDraw.Draw(img)
# vertical dusk gradient: deep indigo to teal
top, bottom = (24, 22, 54), (18, 96, 110)
for y in range(S):
    t = y / (S - 1)
    d.line([(0, y), (S, y)], fill=tuple(round(a + (b - a) * t) for a, b in zip(top, bottom)))
# warm sun with a soft glow
glow = Image.new("L", (S, S), 0)
ImageDraw.Draw(glow).ellipse([292, 232, 732, 672], fill=150)
glow = glow.filter(ImageFilter.GaussianBlur(70))
img = Image.composite(Image.new("RGB", (S, S), (255, 170, 80)), img, glow)
d = ImageDraw.Draw(img)
d.ellipse([362, 302, 662, 602], fill=(255, 196, 102))
# horizon: a dark band cuts the lower half of the sun
d.rectangle([0, 560, S, S], fill=(14, 40, 58))
# three develop-slider tracks with knobs
for i, (y, x) in enumerate([(680, 620), (770, 400), (860, 520)]):
    d.rounded_rectangle([170, y - 9, 854, y + 9], radius=9, fill=(70, 120, 140))
    d.ellipse([x - 34, y - 34, x + 34, y + 34], fill=(240, 240, 235))
img.save("AppIcon-1024.png")
for name, px in [("AppIcon76x76@2x~ipad.png", 152), ("AppIcon83.5x83.5@2x~ipad.png", 167), ("AppIcon60x60@2x.png", 120)]:
    img.resize((px, px), Image.LANCZOS).save(name)
