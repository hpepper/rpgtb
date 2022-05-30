
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

/**
 * @brief get first room of of m_roomList
 * 
 * This will not work in a threat setting, if it is the samme MapManager class
 *  that is accessed from two different threats. Due to the m_roomIterator
 *  could be initialized at the same time the other process calls getNextRoom.
 * 
 * @return Door* 
 */
Door *Room::getFirstDoor()
{
    m_doorIterator = m_doorList.begin();
    return (*m_doorIterator);
}

Door *Room::getNextDoor()
{
    Door *doorPointer = nullptr;
    if (m_doorIterator != m_doorList.end())
    {
        ++m_doorIterator;
        doorPointer = *m_doorIterator;
    }
    return(doorPointer);
}


int Room::loadXmlRoom(tinyxml2::XMLElement *xmlRoom)
{
    int nStatus = 0;
    ABORT_IF_TRUE(xmlRoom == nullptr, "!!! Room XML pointer is a nullptr");
    m_startX = xmlRoom->IntAttribute("xpos") * m_baseUnitInSvg;
    m_startY = xmlRoom->IntAttribute("ypos") * m_baseUnitInSvg;
    m_width = xmlRoom->IntAttribute("width") * m_baseUnitInSvg;
    m_height = xmlRoom->IntAttribute("height") * m_baseUnitInSvg;

    // TODO Add list of doors
        tinyxml2::XMLElement *xmlEnvironmentElement = xmlRoom->FirstChildElement("Door");
    ABORT_IF_FALSE(xmlEnvironmentElement != nullptr, "No child element named 'Door'");
    while (xmlEnvironmentElement != nullptr)
    {
        // TODO Create a Door instance and give the doorXml element to that instance
        Door *door = new Door(m_baseUnitInSvg);
        // Then load the door
        door->loadXmlDoor(xmlEnvironmentElement);
        m_doorList.push_back(door);
        // TODO the door will handle windows etc.
        xmlEnvironmentElement = xmlEnvironmentElement->NextSiblingElement("Door");
    }

    // TODO add list of windows
    return (nStatus);
}
