// JMMT PAK structures

#ifndef JMMT_PACKAGE_H
#define JMMT_PACKAGE_H

#include <cstdint>

// for LzssHeader
#include <jmmt/lzss.h>

#include "FourCCObject.h"

namespace jmmt {


	// This is the "file header" of sorts.
	struct PackageEofHeader {
		std::uint32_t headerSize;
		std::uint32_t debugInfoSize;

		std::uint32_t headerStartOffset;
	};

	struct PackageGroup : public FourCCMagic<0x50524750 /* 'PGRP' */> {
		MagicType magic;
		uint32_t groupNameCrc;

		uint32_t fileCount;
		uint32_t padding;	// 0xcdcdcdcd - padding to 0x10 bytes
	};

	struct PackageFile : public FourCCMagic<0x4C494650 /* 'PFIL' */> {
		MagicType magic;
		uint32_t unk[2]; // Don't know what these are?

		// Sequence number of the chunk.
		// This represents the order of each chunk,
		// presumably so order can just be whatever.
		//
		// However, the archives seem to order chunks for files
		// in order, and doesn't start/interleave other files
		// in between of files.
		//
		// In other words: this is a nice waste of 16 bits.
		uint16_t chunkSequenceNumber;

		// Amount of chunks which need to be read
		// from to read this file completely.
		//
		// 1 means this file starts and ends on this chunk.
		uint16_t chunkAmount;

		// A CRC32 hash of the path of this file.
		// Hashed with jmmt::HashString().
		uint32_t filenameCrc;

		uint32_t unk2[7]; // more unknown stuff I don't know about yet

		// Uncompressed size of this file chunk. Has a maximum of 65535 bytes.
		uint32_t chunkSize;

		// Offset where this file chunk should start,
		// inside of a larger buffer.
		uint32_t blockOffset;

		// Compressed (stored) size of this chunk.
		uint32_t compressedChunkSize;

		// Offset inside of the package file where
		// the compressed data blob starts.
		uint32_t dataOffset;

		uint32_t fileSize;

		// TECH LZSS header.
		// Used to (shocker) configure LZSS decompression.
		//
		// Duplicates a few things in the file.
		LzssHeader lzssHeader;
	};

	static_assert(sizeof(PackageEofHeader) == 0xc, "PackageEofHeader has invalid size. Extractor 100% won't work, good job");
	static_assert(sizeof(PackageGroup) == 0x10, "PackageGroup has invalid size, extractor won't work");
	static_assert(sizeof(PackageFile) == 0x64, "PackageFile has invalid size, extractor won't work");
} // namespace jmmt

#endif // JMMT_PACKAGE_H
