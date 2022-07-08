#ifndef JMMT_LZSS_H
#define JMMT_LZSS_H

#include <cstdint>

namespace jmmt {

	struct LzssHeader {
		std::uint32_t next; // done to keep data layout consistent with PS2
		std::uint8_t cByteId;
		std::uint8_t cHdrSize; // should be sizeof(LzssHeader)
		std::uint8_t nMaxMatch;
		std::uint8_t nFillByte;
		std::uint16_t nRingSize;
		std::uint16_t nErrorId;
		std::uint32_t nUnCompressedBytes;
		std::uint32_t nCompressedBytes;
		std::uint32_t nCRC;
		std::uint32_t nFileId;
		std::uint32_t nCompressedDataCRC;
	};

	static_assert(sizeof(LzssHeader) == 0x20, "LzssHeader doesn't match game expectations, you are CERTAINLY breaking structures");

	/**
	 * Decompress TECH LZSS data.
	 *
	 * \param[in,out] header LZSS header. Unused. Set to nullptr for now.
	 * \param[in] compressedInput LZSS compressed input data.
	 * \param[in] compressedLength Compressed length.
	 * \param[out] destBuffer Destination buffer.
	 *
	 * \return 0 on success. Non zero value means error.
	 */
	int DecompressLzss(LzssHeader* header, std::uint8_t* compressedInput, std::int32_t compressedLength, std::uint8_t* destBuffer);

} // namespace jmmt

#endif
