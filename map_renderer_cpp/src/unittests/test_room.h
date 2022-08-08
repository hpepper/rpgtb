
#include "room.h"

#include <cxxtest/TestSuite.h>
#include <assert.h>

class RoomTestSuite: public CxxTest::TestSuite {

	Room m_cRoom;
public:

	void setUp() {
	}

	void tearDown() {
	}

	void testConstruct() {
		int nRetrunCode = m_cRoom.populateFromFile("map_1.xml", ".");
		TS_ASSERT_EQUALS(nRetrunCode, 0);
	}
};
