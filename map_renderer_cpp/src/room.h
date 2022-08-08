#ifndef ROOM_H
#define ROOM_H

#include "door.h"
#include <list>
#include <tinyxml2.h>
#include "xml_if.h"

class Room
{
public:
    Room(int);
    ~Room();

    int getStartX(){return(m_startX);}
    int getStartY(){return(m_startY);}
    int getWidth(){return(m_width);}
    int getHeight(){return(m_height);}
    int loadXmlRoom(tinyxml2::XMLElement *);

    Door *getFirstDoor();
    Door *getNextDoor();


private:
    int m_baseUnitInSvg = 0;
    int m_startX = 0;
    int m_startY = 0;

    int m_width = 0;
    int m_height = 0;

    XmlIf m_xmlIf;

    std::list<Door*> m_doorList;
    std::list<Door*>::iterator m_doorIterator;

    // TODO list of windows
};

#endif
