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

// "PGRP" chunk.
//
// This marks the start of a "package group".
struct JMMT_PGRP {
	u32 magic;
	u32 groupNameCrc; // This is in the string table
	u32 nrfiles;
	u32 pad; // seemingly always 0xCDCDCDCD
};

// "PFIL" chunk.
//
// This represents a file chunk,
// which can itself represent either a whole file (when it is > 65536 bytes),
// or part of a file (which will need to be stiched together from multiple chunks).
struct JMMT_PFIL {
	u32 magic;

	u32 unk; // These two seem to stay the same for every PFIL
	u32 unk2;

	// Sequence number of the chunk.
	// This repressents the order of each chunk,
	// presumably so order can just be whatever.
	//
	// However the packaging tool seems to leave files
	// in order, and doesn't start/interleave other files
	// in between. This is definitely still useful, but
	// not quite as much.
	u16 chunkNumber;

	// Amount of chunks which need to be read
	// from to complete this file.
	//
	// 1 means this file starts and ends on this chunk.
	u16 chunkAmount;

	// This is a CRC32 hash of the path of this file.
	//
	// Hashed with jmmt::HashString().
	u32 filenameCrc;

	u32 unk3[7]; // These stay the same per file chunk. Could be hashes

	// Uncompressed size of this file chunk. Has a maximum of 65535 bytes.
	u32 chunkSize;

	// Offset where this file chunk should start,
	// inside of a larger buffer.
	u32 bufferOffset;

	// Compressed size of the chunk.
	u32 compressedSize;

	// Offset inside of the package file where
	// the compressed data blob starts.
	u32 dataOffset;

    // Total file size.
	u32 fileSize;

	// TECH LZSS header.
	// Used to (shocker) configure LZSS decompression.
	Lzss_Header lzssHeader;

	// Debug information. This doesn't print literally everything,
	// just the useful stuff to look at it.
	if(1) {
		std::print("Hash: {:0x}, Seqnum: {}, ZOff: {}, FileSize: {}, ChunkSize: {}(z {}), BufferOff: {},", 
		filenameCrc, chunkNumber, dataOffset, fileSize, 
		chunkSize, compressedSize, bufferOffset );
	}
};

// This is a wrapper so we can easily do the chunk viewing in imhex.
// While there's probably a way I could do this easier this works.
// So it's what I'm gonna use.
struct PFIL_WRAPPER {
	JMMT_PFIL pfilChunkZero;

	if(pfilChunkZero.chunkAmount != 1) {
		//std::print("This file has {} chunks", pfilChunkZero.chunkAmount);
		JMMT_PFIL pfilChunkExtra[pfilChunkZero.chunkAmount - 1];
	} else {
		//std::print("File ended with 1 chunk");
	}
};


// Group header is hardcoded for config.pak cause that's what I'm using to test

// Group header.
JMMT_PGRP grp @ 0xd1c30;

// All pfil objects. The wrapper expands out other pfil chunks automatically. Pretty cool.
PFIL_WRAPPER files[grp.nrfiles] @ $;
