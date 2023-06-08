
add_executable(jmmt_renamer jmmt_renamer.cpp)
target_link_libraries(jmmt_renamer PUBLIC jmmt)

set_target_properties(jmmt_renamer PROPERTIES
		CXX_STANDARD 20
		CXX_STANDARD_REQUIRED ON
		)


add_executable(jmmt_pack_extractor jmmt_pack_extractor.cpp)
target_link_libraries(jmmt_pack_extractor PUBLIC jmmt)

set_target_properties(jmmt_pack_extractor PROPERTIES
		CXX_STANDARD 20
		CXX_STANDARD_REQUIRED ON
		)

add_executable(jmmt_hashtool jmmt_hashtool.cpp)
target_link_libraries(jmmt_hashtool PUBLIC jmmt)

set_target_properties(jmmt_hashtool PROPERTIES
		CXX_STANDARD 20
		CXX_STANDARD_REQUIRED ON
		)
