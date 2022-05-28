
/**
 * Description
 */

#include "room.h"

#include "trace.h"

/**
 * fileName - relative (in the asset directory)
 */
Room::Room(int baseUnitInSvg)
{
    m_baseUnitInSvg = baseUnitInSvg;
}

int Room::loadXmlRoom(tinyxml2::XMLElement *xmlRoom)
{
    int nStatus = 0;
    ABORT_IF_TRUE(xmlRoom == nullptr, "!!! Room XML pointer is a nullptr");
    m_startX = xmlRoom->IntAttribute("xpos") * m_baseUnitInSvg;
    m_startY = xmlRoom->IntAttribute("ypos") * m_baseUnitInSvg;
    m_width = xmlRoom->IntAttribute("width") * m_baseUnitInSvg;
    m_height = xmlRoom->IntAttribute("height") * m_baseUnitInSvg;

    std::cout << "DDD xpos: " << m_startX << std::endl;

    return (nStatus);
}
