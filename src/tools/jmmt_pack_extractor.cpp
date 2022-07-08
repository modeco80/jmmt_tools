// Package file extractor.
// Yes, this code is messy, but I just wanted it to work after days of it not doing so.

#include <jmmt/crc.h>
#include <jmmt/lzss.h>
#include <jmmt/package.h>

#include <cstring>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <vector>

namespace fs = std::filesystem;

// This is lame. But it works :)
template <class T>
T LameRead(std::istream& is) {
	if(!is)
		throw std::runtime_error("stream is bad");

	T t {};
	is.read(reinterpret_cast<char*>(&t), sizeof(T));
	return t;
}


/**
 * Reads package files.
 */
struct PackageReader {

	/**
 	 * Decompressed and un-split package file.
	 */
	struct DecompressedFile {
		std::string filename;
		std::vector<std::uint8_t> data;
	};

	explicit PackageReader(std::istream& is)
		: is(is) {
	}

	void Init() {
		is.seekg(-0xc, std::istream::end);

		// Read the eof header
		eofHeader = LameRead<jmmt::PackageEofHeader>(is);

		// We ideally should be at the end of file after reading the eof header.
		auto fileSize = is.tellg();

		// Seek to the header start and read the pgrp.
		is.seekg(static_cast<std::streamsize>(eofHeader.headerStartOffset), std::istream::beg);
		group = LameRead<jmmt::PackageGroup>(is);

		if(group.magic != jmmt::PackageGroup::TypeMagic) {
			fileInvalid = true;
			return;
		}

		// Read the string table, and hash every string out into a map.
		// This is used to build our crc->filename mapping for this archive.
		{
			is.seekg(static_cast<std::streamsize>(eofHeader.headerStartOffset) + static_cast<std::streamsize>(eofHeader.headerSize), std::istream::beg);

			auto ReadString = [&]() {
				std::string s;
				char c {};

				while(true) {
					c = is.get();

					// std::printf("%c\n", c);

					if(c == '\0')
						return s;

					s.push_back(c);
				}
			};

			auto l = is.tellg();
			// seek ahead of the "header" of the debug info/string table,
			// since we don't care about it (we read strings until we "stop". though
			// it might be smart to trust it? idk.)
			is.seekg(sizeof(uint32_t), std::istream::cur);

			while(l != fileSize - static_cast<std::streamsize>(sizeof(eofHeader))) {
				auto string = ReadString();
				crcToFilename[jmmt::HashString(string.c_str())] = string;
				l = is.tellg();

				// print out the creation of the crc/filename map for debugging
				// std::printf("%s -> 0x%x\n", string.c_str(), jmmt::HashString(string.c_str()));
			}
		}

		// Go to the start of the first pfil (skipping the pgrp we just read)
		// after we setup our map.

		is.seekg(static_cast<std::streamsize>(eofHeader.headerStartOffset) + sizeof(jmmt::PackageGroup), std::istream::beg);
	}

	/**
	 *
	 * \return false if file isn't invalid, true otherwise.
	 */
	[[nodiscard]] bool Invalid() const {
		return fileInvalid;
	}

	// Read a file chunk.
	void ReadFileChunk() {
		if(fileInvalid)
			return;

		currChunk = LameRead<jmmt::PackageFile>(is);
		if(currChunk.magic != jmmt::PackageFile::TypeMagic) {
			fileInvalid = true;
			return;
		}

		// Setup some variables

		// If we finished a file, the work buffer is empty.
		if(fileWorkBuffer.empty()) {

			// TODO: Implement CRC-based fallback, if required.
			// 	It PROBABLY isn't.

			currFileName = crcToFilename[currChunk.filenameCrc];

			std::cout << "Reading \"" << currFileName << "\".\n";

			chunksLeft = currChunk.chunkAmount - 1;
			fileWorkBuffer.resize(currChunk.fileSize);
		}

		std::vector<std::uint8_t> compressedBuffer(currChunk.compressedChunkSize);

		auto old = is.tellg();

		is.seekg(currChunk.dataOffset, std::istream::beg);
		is.read(reinterpret_cast<char*>(compressedBuffer.data()), currChunk.compressedChunkSize);

		// If the chunk isn't actually compressed, just copy it into the work buffer.
		// If it is, decompress it.
		if(currChunk.compressedChunkSize == currChunk.chunkSize) {
			memcpy(fileWorkBuffer.data() + currChunk.blockOffset, compressedBuffer.data(), currChunk.chunkSize);
		} else {
			jmmt::DecompressLzss(nullptr, compressedBuffer.data(), currChunk.compressedChunkSize, fileWorkBuffer.data() + currChunk.blockOffset);
		}

		// Seek back to the old place the stream was before reading and decompress
		is.seekg(old, std::istream::beg);
	}

	/**
	 * Read a file from this package.
	 * \param[in] cb Called when file is read
	 */
	template <class DoneCallback>
	void ReadFile(DoneCallback&& cb) {
		if(fileInvalid)
			return;

		// Read first file chunk.
		// It's perfectly legal for this to be all we need to do.
		ReadFileChunk();

		// Read additional chunks, if required.
		for(auto i = 0; i < chunksLeft; ++i) {
			// std::printf("reading additional chunk %d/%d\n", i, chunksLeft);
			ReadFileChunk();
		}

		std::cout << "Read file \"" << currFileName << "\"\n";

		cb(DecompressedFile { .filename = currFileName,
			 .data = fileWorkBuffer });

		// write file data to stdout (debugging!)
		// std::cout.write(reinterpret_cast<const char*>(fileWorkBuffer.data()), fileWorkBuffer.size());

		fileWorkBuffer.clear();
	}

	/**
	 * Read all possible files from this package.
	 * \param[in] cb Called when file is read
	 */
	template <class DoneCallback>
	void ReadFiles(DoneCallback&& cb) {
		if(fileInvalid)
			return;

		for(auto i = 0; i < group.fileCount; ++i)
			ReadFile(cb);
	}

	jmmt::PackageGroup& GetGroup() {
		return group;
	}

   private:
	std::istream& is;

	// Set to true on any invalid file data.
	bool fileInvalid = false;

	jmmt::PackageEofHeader eofHeader {};

	jmmt::PackageGroup group {};

	/**
	 * CRC->sensible filename map.
	 */
	std::map<jmmt::crc32_t, std::string> crcToFilename;

	// file stuff
	uint32_t chunksLeft {};

	// The name of the file we are processing.
	std::string currFileName;

	// The current chunk the reader is reading.
	jmmt::PackageFile currChunk {};

	// File-sized work buffer used to store the file
	// we're currently working on. Freed when a file is
	// finished being extracted.
	std::vector<std::uint8_t> fileWorkBuffer;
};

int main(int argc, char** argv) {
	if(argc != 2) {
		std::cout << "Usage: " << argv[0] << " [path 2 JMMT PAK file]";
		return 1;
	}

	std::ifstream ifs(argv[1], std::ifstream::binary);

	if(!ifs) {
		std::cout << "Invalid file \"" << argv[1] << "\"\n";
		return 1;
	}

	PackageReader reader(ifs);

	reader.Init();

	if(reader.Invalid()) {
		std::cout << "File \"" << argv[1] << "\" doesn't seem to be a PAK file.\n";
		return 1;
	}

	auto path = fs::path(argv[1]).stem();

	reader.ReadFiles([&](const auto& file) {
		auto outpath = path / file.filename;
		fs::create_directories(outpath.parent_path());

		std::ofstream ofs(outpath.string(), std::ofstream::binary);
		if(!ofs) {
			std::cout << "Could not open \"" << outpath.string() << "\".\n";
			std::exit(1);
		}

		ofs.write(reinterpret_cast<const char*>(file.data.data()), static_cast<std::streampos>(file.data.size()));
		ofs.close();

		std::cout << "Wrote \"" << outpath.string() << "\".\n";
	});

	return 0;
}
