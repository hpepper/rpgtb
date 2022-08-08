#ifndef DOOR_H
#define DOOR_H

#include <list>
#include <tinyxml2.h>
#include "xml_if.h"

class Door
{
public:
    Door(int);
    ~Door();

    int getStartX() { return (m_startX); }
    int getStartY() { return (m_startY); }
    int getEndX() { return (m_endX); }
    int getEndY() { return (m_endY); }
    /// number of doors in the door opening.
    int getSections() { return (m_sections); }
    int loadXmlDoor(tinyxml2::XMLElement *);

private:
    XmlIf m_xmlIf;
    int m_baseUnitInSvg = 0;
    int m_startX = 0;
    int m_startY = 0;
    int m_endX = 0;
    int m_endY = 0;
    int m_sections = 0;
};

#endif
