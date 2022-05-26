
/**
 * Description
 */

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
    nStatus = leadOut();
    return (nStatus);
}

// generate DM overlay (text)
int SvgRenderer::renderGameMasterMap(std::string filename)
{
    int nStatus = 0;
    return (nStatus);
}
