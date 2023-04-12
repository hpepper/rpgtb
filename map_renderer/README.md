# Map renderer

## Introduction

### References

* [Quick XML documentation](https://docs.rs/quick-xml/latest/quick_xml/#modules)

### Usage

#### Generate doors

* make && map_renderer --door-mode --door-width 200 --door-sections 2 && eog tmp_door.svg

#### test with the test map

* make &&  map_renderer && eog player_map.svg
