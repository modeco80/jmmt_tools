 

add_library(jmmt
		crc.cpp
		lzss.cpp
		)

target_include_directories(jmmt PUBLIC ${PROJECT_SOURCE_DIR}/include)

set_target_properties(jmmt PROPERTIES
		CXX_STANDARD 20
		CXX_STANDARD_REQUIRED ON
		)