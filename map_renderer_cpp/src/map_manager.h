#ifndef MAP_MANAGER_H
#define MAP_MANAGER_H

#include <list>
#include <string>
#include <tinyxml2.h>
#include "xml_if.h"
#include "room.h"



// TODO make this a parm
#define ASSET_DIR "../assets"

/**
 * @brief This handles the sprites/elements on the map
 * 
 * This DOES NOT handle movable objects, like avatars and projectiles.
 * 
 * TODO for movable objects @see todo add that here
 */
class MapManager
{
public:
    MapManager();
    ~MapManager();
    // int collisionDetection();

    // this is for use with unit testing, to validate number of entries read.
    int getNumberOfEnvironmentWalls();


    int populateFromFile(std::string fileName);
    int populateFromFile(std::string fileName, std::string directoryPath);

    int renderEntities();

    int getMapHeight() {return(m_mapHeight);};
    int getMapWidth() {return(m_mapWidth);};

    Room *getFirstRoom();
    Room *getNextRoom();


private:
    std::string m_fileName = "";
    std::string m_assetsDirectory = ASSET_DIR;
    tinyxml2::XMLElement *m_xmlRoot = nullptr;
    XmlIf m_xmlif;
    int m_baseUnitInSvg = 0;
    int m_mapHeight = 25;
    int m_mapWidth = 25;

    std::list<Room*> m_roomList;
    std::list<Room*>::iterator m_roomIterator;

    int populateMapListFromXml(tinyxml2::XMLElement *);
    int readMapDefinitions(tinyxml2::XMLElement *, std::string);

};

#endif