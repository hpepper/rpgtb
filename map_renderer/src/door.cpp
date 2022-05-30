
/**
 * Description
 */

#include "door.h"
#include "trace.h"

/**
 * fileName - relative (in the asset directory)
 */
Door::Door(int baseUnitInSvg)
{
    m_baseUnitInSvg = baseUnitInSvg;
}

int Door::loadXmlDoor(tinyxml2::XMLElement *xmlRoom)
{
    int nStatus = 0;
    ABORT_IF_TRUE(xmlRoom == nullptr, "!!! Room XML pointer is a nullptr");
    m_startX = xmlRoom->IntAttribute("startx") * m_baseUnitInSvg;
    m_startY = xmlRoom->IntAttribute("starty") * m_baseUnitInSvg;
    m_endX = xmlRoom->IntAttribute("endx") * m_baseUnitInSvg;
    m_endY = xmlRoom->IntAttribute("endy") * m_baseUnitInSvg;
    m_sections = xmlRoom->IntAttribute("sections");

    std::cout << "DDD loadXmlDoor() m_startX: " << m_startX << "\n";
    std::cout << "DDD loadXmlDoor() m_startY: " << m_startY << "\n";
    std::cout << "DDD loadXmlDoor() m_endX: " << m_endX << "\n";
    std::cout << "DDD loadXmlDoor() m_endY: " << m_endY << "\n";

    return (nStatus);
}