#include <std/io.pat>


// TECH types

// LZSS compression header thingy.
struct Lzss_Header {
	u32 next; // this is formally Lzss_Header*
			  // but hexpat does handle pointers
			  // and I kinda don't want it to.

	u8 cByteId;
	u8 cHdrSize;
	u8 nMaxMatch;
	u8 nFillByte;
	u16	nRingSize;
	u16	nErrorId;
	u32	nUnCompressedBytes;
	u32	nCompressedBytes;
	u32	nCRC;
	u32	nFileId;
	u32	nCompressedDataCRC;

	// if valid, print out (useful) information
	//if(cByteId == 0x91 && cHdrSize == 0x20) {
	//std::print("<lzss>");
	//std::print("  header_size: {}, max_match: {}, fill_byte: {} ring_size: {}", cHdrSize, nMaxMatch, nFillByte, nRingSize);
	//std::print("  uncompressed_size: {}, compressed_size: {}, crc: {}, file_id: {}, compressed_crc: {}", nUnCompressedBytes, nCompressedBytes, nCRC, nFileId, nCompressedDataCRC);
	//std::print("</lzss>");
	//}
};

// Data strutures actually part of the file:

// "PGRP" entry.
//
// This marks the start of a "package group".
// Whatever that is.
struct JMMT_PGRP {
	u32 magic;
	u32 groupNameCrc; // This is in the string table
	u32 nrfiles;
	u32 pad; // seemingly always 0xCDCDCDCD
};

// "PFIL" entry.
//
// This represents a file block,
// which can itself represent either a whole file (> 65535 bytes),
// or part of a file (which will need to be stiched together).
struct JMMT_PFIL {
	u32 magic;

	u32 unk[2]; // Don't know what these are?

	// Sequence number of the chunk.
	// This repressents the order of each chunk,
	// presumably so order can just be whatever.
	//
	// However the game seems to order chunks for files
	// in order, and doesn't start/interleave other files
	// in between. So this is a nice waste of 16 bits.
	u16 chunkSequenceNumber;

	// Amount of chunks which need to be read
	// from to read this file completely.
	//
	// 1 means this file starts and ends on this chunk.
	u16 chunkAmount;

	// This is a CRC32 hash of the path of this file.
	//
	// Hashed with jmmt::HashString() (in the jmmt_tools repo).
	u32 filenameCrc;

	u32 unk2[7]; // more unknown stuff I don't care/know about

	// Uncompressed size of this file chunk. Has a maximum of 65535 bytes.
	u32 chunkSize;

	// Offset where this file chunk should start,
	// inside of a larger buffer.
	u32 blockOffset;

	// ?
	u32 unk3;

	// Offset inside of the package file where
	// the compressed data blob starts.
	u32 dataOffset;

	u32 fileSize;

	// TECH LZSS header.
	// Used to (shocker) configure LZSS decompression.
	Lzss_Header lzssHeader;

	// Debug information. This doesn't print literally everything,
	// just the useful stuff to look at it.
	if(1) {
		std::print("    Chunk seqNum: {}, Filename CRC: {:0x}, File Size: {}, Chunk Size: {}, Block Offset: {}, Data Offset: {}", chunkSequenceNumber, filenameCrc, fileSize, chunkSize, blockOffset, dataOffset);
	}

	//if(lzHeader.cByteId == 0x91)
	//	std::print("file has a valid lzss header");

};

// This is a wrapper so we can easily do the chunk viewing in imhex.
// While there's probably a way I could do this easier this works.
// So it's what I'm gonna use.
struct PFIL_WRAPPER {
	JMMT_PFIL pfilChunkZero;

	if(pfilChunkZero.chunkAmount != 1) {
		std::print("This file has {} chunks", pfilChunkZero.chunkAmount);
		JMMT_PFIL pfilChunkExtra[pfilChunkZero.chunkAmount - 1];
	} else {
		std::print("File ended with 1 chunk");
	}
};


// Group header is hardcoded for config.pak cause that's what I'm using to test

// Group header.
JMMT_PGRP grp @ 0xd1c30;

// This isn't right (as one PFIL chunk doesn't actually have to mean one file),
// but it works for testing and trying to understand the format.
PFIL_WRAPPER files[grp.nrfiles] @ $;