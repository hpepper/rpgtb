
#include "svg_renderer.h"

#include <cxxtest/TestSuite.h>
#include <assert.h>

class SvgRendererTestSuite: public CxxTest::TestSuite {

	SvgRenderer m_cSvgRenderer;
public:

	void setUp() {
	}

	void tearDown() {
	}

	void testConstruct() {
		int nRetrunCode = m_cSvgRenderer.populateFromFile("map_1.xml", ".");
		TS_ASSERT_EQUALS(nRetrunCode, 0);
	}
};
