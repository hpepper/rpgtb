#ifndef SVG_RENDERER_H
#define SVG_RENDERER_H

#include "map_manager.h"

#include <fstream>

class SvgRenderer
{
public:
    SvgRenderer(MapManager *);
    ~SvgRenderer();

    int renderUserMap(std::string);
    int renderGameMasterMap(std::string);

private:
    MapManager *m_mapMananger = nullptr;
    std::ofstream m_svgFile;
    int m_wallStrokeWidth = 2;
    int m_doorStrokeWidth = 1;

    int leadIn();
    int leadOut();
    void renderDoor(Door *, int, int);
    void renderRoom(Room *);
};

#endif
