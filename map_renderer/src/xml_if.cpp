
/**
 * Description
 */

#include "xml_if.h"
#include "trace.h"

/**
 * fileName - relative (in the asset directory)
 */
XmlIf::XmlIf()
{
}

XmlIf::~XmlIf()
{
}

/**
 * @param fileName - Name of xml to load.
 */

tinyxml2::XMLElement *XmlIf::loadXmlFile(std::string fileName)
{
    return (loadXmlFile(fileName, "."));
}

/**
 * @brief create all walls defined in the XML file referenced.
 *
 * @param fileName - name of the xml file
 * @param directoryPath - the path to the xml file.
 * @return int 0: ok
 */
tinyxml2::XMLElement *XmlIf::loadXmlFile(std::string fileName, std::string directoryPath)
{
    std::string assetsDirectory = directoryPath;
    // std::string fileName = fileName;

    tinyxml2::XMLDocument *xmlDoc = new tinyxml2::XMLDocument();
    ABORT_IF_TRUE(xmlDoc == nullptr, "XML document not created")

    std::string fileNameXml = assetsDirectory + "/" + fileName;
    tinyxml2::XMLError xmlStatus = xmlDoc->LoadFile(fileNameXml.c_str());

    if (xmlStatus != tinyxml2::XML_SUCCESS)
    {
        printf("!!! file: %s has an error: %s\n", fileNameXml.c_str(), xmlDoc->ErrorStr());
        ABORT_IF_FALSE(xmlStatus == tinyxml2::XML_SUCCESS, "XML load error");
    }
    m_xmlRoot = xmlDoc->RootElement();
    if (m_xmlRoot == nullptr)
    {
        printf("!!! the file '%s' has no root element.\n", fileName.c_str());
        ABORT_IF_TRUE(m_xmlRoot == nullptr, "No root element in the XML file");
    }
    return (m_xmlRoot);
}

std::string XmlIf::getChildStringOfFirtSubElementName(tinyxml2::XMLElement *xmlElement, std::string tagName)
{
    ABORT_IF_TRUE(xmlElement == nullptr, "XML element pointer is null")
    std::string returnString = "";
    tinyxml2::XMLElement *xmlFirstChildElement = xmlElement->FirstChildElement(tagName.c_str());
    if ( xmlFirstChildElement != nullptr ) {
        returnString = xmlFirstChildElement->GetText();
    }

    return (returnString);
}

int XmlIf::getChildIntOfFirtSubElementName(tinyxml2::XMLElement *xmlElement, std::string tagName)
{
    
    return (std::atoi(getChildStringOfFirtSubElementName(xmlElement,tagName).c_str()));
}
