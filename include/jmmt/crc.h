#ifndef JMMT_CRC_H
#define JMMT_CRC_H

#include <cstdint>

namespace jmmt {
	/**
	 * Result type of HashString()/HashStringCase().
	 */
	using crc32_t = std::uint32_t;

	crc32_t HashString(const char* s);

	/**
	 * Hash a case-sensitive string.
	 */
	crc32_t HashStringCase(const char* s);

} // namespace jmmt

#endif // JMMT_CRC_H
