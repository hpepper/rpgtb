# Map renderer

render a map as SVG from a description in an XML file.

* e.g. for use on Roll20

The base unit i a doors width.

How to position the door?

* in a relative coordinate system, meaning you can place the door in the middle of the room.

## Build

make && ./maprenderer && cat first_map.svg

## TODO

* Be able to generate a house with rooms
* Be able to draw a fence
* Be able to draw a dungeon (with rought walls)
* Be able to have tiles, like roads, grass etc.
* Documentation

## Overview

### Directory structure

* doc - documentations
* test - unit tests
* src - source code
  * xmlreader
  * renderer
