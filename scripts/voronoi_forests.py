#!/usr/bin/env python3
"""
Generates Voronoi paths among trees.
"""

import argparse
import matplotlib.pyplot as plt
import numpy as np
import random
from matplotlib.widgets import Slider


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--centroids", "-c", help="Number of centroids", type=int, default=30)
    parser.add_argument("--trees", "-t", help="Number of trees", type=int, default=1000)
    parser.add_argument("--path-width", "-p", help="Path width", type=float, default=0.02)

    return parser.parse_args()

def distance_to_points(point, points):
    px, py = point
    return [np.sqrt((px - x) ** 2 + (py - y) ** 2) for x, y in points]

def is_on_path(point, points, path_width):
    distances = distance_to_points(point, points)
    distances.sort()
    if abs(distances[0] - distances[1]) < path_width * random.gauss(1., 0.3):
        return True

    return False

def is_in_home(point, points):
    distances = distance_to_points(point, points)
    if min(distances) == distances[0]:
        return True
    return False


def main():
    args = parse_args()

    random.seed(42)

    centroids = [(0.5, 0.5)] # home
    centroids += [(random.random(), random.random()) for _ in range(args.centroids)]

    trees = []
    while len(trees) < args.trees:
        point = (random.random(), random.random())

        if is_on_path(point, centroids, args.path_width):
            continue

        if is_in_home(point, centroids):
            continue

        trees.append(point)

    fig, ax = plt.subplots()
    plt.subplots_adjust(left=0.25, bottom=0.25)
    #plt.plot([x for x, y in centroids], [y for x, y in centroids], 'rx')
    plt.plot([x for x, y in trees], [y for x, y in trees], 'g.')

    #plt.legend(['Centroids', 'Trees'])
    plt.xlim(0, 1)
    plt.ylim(0, 1)

    plt.show()

if __name__ == '__main__':
    main()
