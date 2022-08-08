
#include "door.h"

#include <cxxtest/TestSuite.h>
#include <assert.h>

class DoorTestSuite: public CxxTest::TestSuite {

	Door m_cDoor;
public:

	void setUp() {
	}

	void tearDown() {
	}

	void testConstruct() {
		int nRetrunCode = m_cDoor.populateFromFile("map_1.xml", ".");
		TS_ASSERT_EQUALS(nRetrunCode, 0);
	}
};
