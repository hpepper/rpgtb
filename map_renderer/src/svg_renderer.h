#ifndef SVG_RENDERER_H
#define SVG_RENDERER_H

#include "map_manager.h"

#include <fstream>

class SvgRenderer
{
public:
    SvgRenderer(MapManager *);
    ~SvgRenderer();

    void renderRoom(Room *);
    int renderUserMap(std::string);
    int renderGameMasterMap(std::string);

private:
    MapManager *m_mapMananger = nullptr;
    std::ofstream m_svgFile;

    int leadIn();
    int leadOut();
};

#endif
