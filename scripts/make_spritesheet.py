#!/usr/bin/env python3
"""
Generates a spritesheet from a bunch of pngs.
"""

import argparse
from PIL import Image

HEIGHT, WIDTH = 256, 256

DEBUG_COLOR = (193, 96, 137, 255)

def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("image", nargs="+", type=Image.open)
    parser.add_argument("--output", "-o", type=argparse.FileType('wb'), required=True)
    parser.add_argument("--ron", "-r", type=argparse.FileType('w'), required=True)
    parser.add_argument("--debug-alpha", "-d", action="store_true", help="Make alpha violet, so we can see it")

    return parser.parse_args()

def main():
    args = parse_args()

    dst_image = Image.new('RGBA', (WIDTH, HEIGHT * len(args.image)))

    if args.debug_alpha:
        dst_image.paste(DEBUG_COLOR, [0, 0, dst_image.size[0], dst_image.size[1]])

    for i, image in enumerate(args.image):
        layer = Image.new('RGBA', (WIDTH, HEIGHT * len(args.image)))
        layer.paste(image, (0, HEIGHT * i))
        dst_image = Image.alpha_composite(dst_image, layer)

    dst_image.save(args.output.name)

    args.ron.write("""(
    spritesheet_width: {width},
    spritesheet_height: {height},
    sprites: [""".format(width=WIDTH, height=HEIGHT * len(args.image)))

    for i in range(len(args.image)):
        args.ron.write("""
        (
            x: {x},
            y: {y},
            width: {width},
            height: {height},
        ),""".format(width=WIDTH, height=HEIGHT, x=0, y=HEIGHT * i))

    args.ron.write("""
    ],
)""".format(width=WIDTH, height=HEIGHT))


if __name__ == '__main__':
    main()
