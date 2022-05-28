/**
 * Loads the map
 *   and handles e.g. walls, so that you have the render and the collision boxes
 *   and defines the spawn points
 */

#include "map_manager.h"
#include "trace.h"

/**
 * fileName - relative (in the asset directory)
 */
MapManager::MapManager()
{
}

MapManager::~MapManager()
{
}

/**
 * @brief get first room of of m_roomList
 * 
 * This will not work in a threat setting, if it is the samme MapManager class
 *  that is accessed from two different threats. Due to the m_roomIterator
 *  could be initialized at the same time the other process calls getNextRoom.
 * 
 * @return Room* 
 */
Room *MapManager::getFirstRoom()
{
    m_roomIterator = m_roomList.begin();
    return (*m_roomIterator);
}

Room *MapManager::getNextRoom()
{
    Room *roomPointer = nullptr;
    if (m_roomIterator != m_roomList.end())
    {
        ++m_roomIterator;
        roomPointer = *m_roomIterator;
    }
    return(roomPointer);
}

/**
 * @brief return the number of wall elements
 *
 * TODO used for?
 *
 * @return int
 */
int MapManager::getNumberOfEnvironmentWalls()
{
    return (-1);
}

/**
 * TODO delete all sub classes
 * @param fileName - Name of xml to load.
 */

int MapManager::populateFromFile(std::string fileName)
{
    return (populateFromFile(fileName, ASSET_DIR));
}

/**
 * @brief create all walls defined in the XML file referenced.
 *
 * @param fileName - name of the xml file
 * @param directoryPath - the path to the xml file.
 * @return int 0: ok
 */
int MapManager::populateFromFile(std::string fileName, std::string directoryPath)
{
    int nStatus = 0;
    m_assetsDirectory = directoryPath;
    m_fileName = fileName;

    std::cout << "DDD populateFromFile()\n";

    // TODO load xml
    std::string fileNameXml = m_assetsDirectory + "/" + m_fileName;

    m_xmlRoot = m_xmlif.loadXmlFile(fileNameXml);
    std::cout << "DDD XML file loaded: " << fileNameXml << "\n";
    nStatus = 0;
    // TODO read xml structure version
    m_baseUnitInSvg = m_xmlif.getChildIntOfFirtSubElementName(m_xmlRoot, "BaseUnitInSvg");
    readMapDefinitions(m_xmlRoot, "MapDefinitions");
    ABORT_IF_TRUE(m_baseUnitInSvg == 0, "!!! <BaseUnitInSvg> is not defined or filled out in the xml.");
    std::cout << "DDD BaseUnitInSvg: " << m_baseUnitInSvg << "\n";
    tinyxml2::XMLElement *xmlEnvironmentElement = m_xmlRoot->FirstChildElement("Room");
    // From https://stackoverflow.com/questions/7942191/how-to-handle-tinyxml-null-pointer-returned-on-gettext
    ABORT_IF_FALSE(xmlEnvironmentElement != nullptr, "No child element named 'Room'");
    while (xmlEnvironmentElement != nullptr)
    {
        // TODO Create a Room instance and give the roomXml element to that instance
        Room *room = new Room(m_baseUnitInSvg);
        // Then load the room
        room->loadXmlRoom(xmlEnvironmentElement);
        m_roomList.push_back(room);
        // TODO the room will handle windows etc.
        xmlEnvironmentElement = xmlEnvironmentElement->NextSiblingElement("Room");
    }
    // TODO read all Environment (TODO later be able to support other env type than wall )
    return (nStatus);
}

/**
 * @brief Read the map definitions from the ownerTagName
 *
 * @param xmlParent - the XML element that holds the <ownerTagName>
 * @param ownerTagName - name of the map definitions tag.
 * @return int
 */
int MapManager::readMapDefinitions(tinyxml2::XMLElement *xmlParent, std::string ownerTagName)
{
    int nStatus = 0;

    tinyxml2::XMLElement *xmlMapDefinitionElement = xmlParent->FirstChildElement(ownerTagName.c_str());

    m_mapHeight = m_xmlif.getChildIntOfFirtSubElementName(xmlMapDefinitionElement, "MapHeight") * m_baseUnitInSvg;
    m_mapWidth = m_xmlif.getChildIntOfFirtSubElementName(xmlMapDefinitionElement, "MapWidth") * m_baseUnitInSvg;
    std::cout << "DDD m_mapWidth: " << m_mapWidth << "\n";

    if (m_mapWidth * m_mapHeight == 0)
    {
        nStatus = -1;
    }

    return (nStatus);
}