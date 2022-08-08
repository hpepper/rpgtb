
/**
 * Description
 */
#include <cmath>
#include "svg_renderer.h"
#include "trace.h"

/**
 * fileName - relative (in the asset directory)
 */
SvgRenderer::SvgRenderer(MapManager *mapMananger)
{
    ABORT_IF_TRUE(mapMananger == nullptr, "!!! mapMananger is a nullptr")
    m_mapMananger = mapMananger;
}

/**
 * @brief Open the file and write the initial SVG file.
 *
 * @return int 0 = ok
 */
int SvgRenderer::leadIn()
{
    int nStatus = 0;
    m_svgFile << "<svg version=\"1.1\"\n";
    m_svgFile << " width=\"" << m_mapMananger->getMapHeight() << "\" height=\"" << m_mapMananger->getMapWidth() << "\"\n";
    m_svgFile << " xmlns=\"http://www.w3.org/2000/svg\">\n";
    m_svgFile << "\n";
    return (nStatus);
}

/**
 * @brief Write the end of the SVG and close the file.
 *
 * @return int 0 = ok
 */
int SvgRenderer::leadOut()
{
    int nStatus = 0;
    m_svgFile << "</svg>\n";
    m_svgFile.close();
    return (nStatus);
}

void SvgRenderer::renderDoor(Door *door, int roomBaseX, int roomBaseY)
{

    // Calculate the door width
    int deltaX = door->getEndX() - door->getStartX();
    int deltaY = door->getEndY() - door->getStartY();
    double doorLength = sqrt(pow(deltaX, 2) + pow(deltaY, 2));
    std::cout << "DDD renderDoor() deltaX: " << deltaX << "\n";
    std::cout << "DDD renderDoor() deltaY: " << deltaY << "\n";
    std::cout << "DDD renderDoor() doorLength: " << doorLength << std::endl;
    // TODO find the start angle
    double pi = 3.14;
    int degrees = -25;
    double radians = (degrees * pi) / 180;
    int newX = deltaX * cos(radians) - deltaY * sin(radians);
    int newY = deltaX * sin(radians) + deltaY * cos(radians);

    int startX = roomBaseX + door->getStartX();
    int startY = roomBaseY + door->getStartY();
    std::cout << "DDD renderDoor() startX: " << startX << "\n";
    std::cout << "DDD renderDoor() startY: " << startY << "\n";
    m_svgFile << "<line x1=\"" << startX << "\"";
    m_svgFile << " y1=\"" << startY << "\"";
    m_svgFile << " x2=\"" << startX + newX << "\"";
    m_svgFile << " y2=\"" << startY + newY << "\"";
    m_svgFile << " stroke=\"black\" fill=\"transparent\"";
    m_svgFile << " stroke-width=\"" << m_doorStrokeWidth << "\"/>\n";

    int xAxisRotation = 0; // TODO figure out how to do this
    int largeArcFlag = 0;
    int sweepFlag = 0;

    // do the arc
    m_svgFile << "<path d=\"M " << roomBaseX + door->getEndX() << " " << roomBaseX + door->getEndY() << "";
    // A rx ry x-axis-rotation large-arc-flag sweep-flag x y
    int doorArcRadius = 10; // TODO how to calculate this?
    m_svgFile << " A " << doorArcRadius << " " << doorArcRadius << ",";
    m_svgFile << " " << xAxisRotation << ", ";
    m_svgFile << " " << largeArcFlag << ", ";
    m_svgFile << " " << sweepFlag << ", ";
    m_svgFile << " " << startX + newX << " " << startY + newY << "\"";
    // m_svgFile << " fill=\"transparent\"";
    m_svgFile << " stroke-width=\""<< m_doorStrokeWidth <<"\"/>";
}

/**
 * @brief Render the given room.
 *
 * @param room
 */
void SvgRenderer::renderRoom(Room *room)
{

    m_svgFile << "<rect x=\"" << room->getStartX() << "\" y=\"" << room->getStartY() << "\" width=\"" << room->getWidth() << "\" height=\"" << room->getHeight() << "\" stroke=\"black\" fill=\"transparent\" stroke-width=\"" << m_wallStrokeWidth << "\"/>\n";
    // TODO Render all doors
    Door *door = room->getFirstDoor();
    renderDoor(door, room->getStartX(), room->getStartY());

    // TODO Render all windows
}

/**
 * @brief render the user map, as SVG file
 *
 * @param filename - full name of the file to be generated.
 * @return int 0 is ok.
 */
int SvgRenderer::renderUserMap(std::string filename)
{
    int nStatus = 0;
    m_svgFile.open(filename);
    nStatus = leadIn();
    Room *room = m_mapMananger->getFirstRoom();
    renderRoom(room);
    nStatus = leadOut();
    return (nStatus);
}

// generate DM overlay (text)
int SvgRenderer::renderGameMasterMap(std::string filename)
{
    int nStatus = 0;
    return (nStatus);
}
