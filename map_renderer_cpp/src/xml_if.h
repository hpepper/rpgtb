#ifndef XML_IF_H
#define XML_IF_H

#include<string>
#include<tinyxml2.h>


class XmlIf
{
public:
    XmlIf();
    ~XmlIf();

    tinyxml2::XMLElement *loadXmlFile(std::string);
    tinyxml2::XMLElement *loadXmlFile(std::string, std::string);
    tinyxml2::XMLElement *getRoot() {return(m_xmlRoot);};


    std::string getChildStringOfFirtSubElementName(tinyxml2::XMLElement*, std::string);
    int getChildIntOfFirtSubElementName(tinyxml2::XMLElement*, std::string);

private:
    tinyxml2::XMLElement *m_xmlRoot = nullptr;
};

#endif
