
/**
 * Description
 */

#include <iostream>
#include "map_manager.h"
#include "svg_renderer.h"
#include "trace.h"

/**
 * fileName - relative (in the asset directory)
 */
// TODO get xml filename as an XML
int main()
{
    std::cout << "Version: 0.2.0" << std::endl;

    MapManager *mapMananger = new MapManager();

    ABORT_IF_FALSE(mapMananger->populateFromFile("unittests/test_lab.xml", "./") == 0, "!!! Map load failed.");

    SvgRenderer * svgRenderer = new SvgRenderer(mapMananger);
    svgRenderer->renderUserMap("first_map.svg");

    return (0);
}
