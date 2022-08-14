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

std::string ReadString(std::istream& is) {
	std::string s;
	char c;

	if(!is)
		return "";

	while(true) {
		c = static_cast<char>(is.get());

		if(c == '\0')
			return s;

		s.push_back(c);
	}
}

/**
 * Reader for a package file.
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
		is.seekg(-static_cast<ssize_t>(sizeof(jmmt::PackageEofHeader)), std::istream::end);
		eofHeader = LameRead<jmmt::PackageEofHeader>(is);

		// We ideally should be at the end of file after reading the eof header.
		auto fileSize = is.tellg();

		is.seekg(static_cast<std::streamsize>(eofHeader.headerStartOffset), std::istream::beg);

		group = LameRead<jmmt::PackageGroup>(is);
		if(group.magic != jmmt::PackageGroup::TypeMagic) {
			fileInvalid = true;
			return;
		}

		// Read the string table, and hash every string in it into a map of CRC to string.
		{
			is.seekg(static_cast<std::streamsize>(eofHeader.headerStartOffset) + static_cast<std::streamsize>(eofHeader.headerSize), std::istream::beg);
			auto l = is.tellg();

			// seek ahead of the "header" of the debug string table,
			// since we don't care about it (we read strings until we hit true EOF.
			// though it might be smart to trust it? IDK.)
			is.seekg(sizeof(uint32_t), std::istream::cur);

			while(l != fileSize - static_cast<ssize_t>(sizeof(eofHeader))) {
				auto string = ReadString(is);
				crcToFilename[jmmt::HashString(string.c_str())] = string;
				l = is.tellg();
			}
		}

		//std::cout << "Group name: \"" << crcToFilename[group.groupNameCrc] << "\"\n";

		// Go to the start of the first file chunk, skipping the group that we just read,
		// after we have finished creating our CRC->filename map.
		is.seekg(static_cast<ssize_t>(eofHeader.headerStartOffset) + static_cast<ssize_t>(sizeof(jmmt::PackageGroup)), std::istream::beg);
	}

	/**
	 * Get if the package file is invalid.
	 *
	 * \return True if file is invalid; false otherwise
	 */
	[[nodiscard]] bool Invalid() const {
		return fileInvalid;
	}

	/**
	 * Read a single file chunk.
	 */
	void ReadFileChunk() {
		currChunk = LameRead<jmmt::PackageFile>(is);

		if(currChunk.magic != jmmt::PackageFile::TypeMagic) {
			std::cout << "Invalid file chunk\n";
			std::exit(1);
		}

		// If we finished a file, the work buffer is empty.
		if(fileWorkBuffer.empty()) {
			currFileName = crcToFilename[currChunk.filenameCrc];

			//std::cout << "Reading \"" << currFileName << "\".\n";

			chunksLeft = currChunk.chunkAmount - 1;
			fileWorkBuffer.resize(currChunk.fileSize);
		}

		std::vector<std::uint8_t> compressedBuffer(currChunk.compressedChunkSize);

		// Read into temporary buffer.
		auto old = is.tellg();
		is.seekg(currChunk.dataOffset, std::istream::beg);
		is.read(reinterpret_cast<char*>(compressedBuffer.data()), currChunk.compressedChunkSize);
		is.seekg(old, std::istream::beg);

		// If the chunk isn't actually compressed, just copy it into the work buffer.
		// If it is, decompress it into the work buffer.
		if(currChunk.compressedChunkSize == currChunk.chunkSize) {
			memcpy(fileWorkBuffer.data() + currChunk.blockOffset, compressedBuffer.data(), currChunk.chunkSize);
		} else {
			jmmt::DecompressLzss(nullptr, compressedBuffer.data(), static_cast<std::int32_t>(currChunk.compressedChunkSize), fileWorkBuffer.data() + currChunk.blockOffset);
		}
	}

	/**
	 * Read a file from this package.
	 * \param[in] cb Called when file is finished being read.
	 */
	template <class DoneCallback>
	void ReadFile(DoneCallback&& cb) {
		ReadFileChunk();

		// Read additional chunks required to complete the file,
		// if we (well) have to.
		for(auto i = 0; i < chunksLeft; ++i) {
			//std::cout << "Reading additional chunk " << i + 1 << '/' << chunksLeft << ".\n";
			ReadFileChunk();
		}

		std::cout << "Read file \"" << currFileName << "\" from archive.\n";

		// Call user-provided callback
		cb(DecompressedFile { .filename = currFileName,
			 .data = fileWorkBuffer });


		fileWorkBuffer.clear();
	}

	/**
	 * Read all possible files from this package.
	 * \param[in] cb Called when file is finished being read.
	 */
	template <class DoneCallback>
	void ReadFiles(DoneCallback&& cb) {
		for(auto i = 0; i < group.fileCount; ++i)
			ReadFile(cb);
	}

	[[maybe_unused]] jmmt::PackageGroup& GetGroup() {
		return group;
	}

   private:
	std::istream& is;

	// Set to true on any invalid file data.
	bool fileInvalid = false;

	/**
	 * EOF header.
	 */
	jmmt::PackageEofHeader eofHeader {};

	/**
	 * Group header.
	 */
	jmmt::PackageGroup group {};

	/**
	 * CRC->sensible string map.
	 * Might be worth renaming.
	 */
	std::map<jmmt::crc32_t, std::string> crcToFilename;

	/**
	 * The amount of chunks left that we need to read to complete a file.
	 */
	uint32_t chunksLeft {};

	/**
	 * Filename from crcToFilename of the file we're reading.
	 */
	std::string currFileName;

	/**
	 * The current chunk the reader is reading.
	 */
	jmmt::PackageFile currChunk {};

	/**
	 * Work buffer used to store the file we are currently trying to read.
	 *
	 * Freed when a file is extracted.
	 */
	std::vector<std::uint8_t> fileWorkBuffer;
};

int main(int argc, char** argv) {
	if(argc != 2) {
		std::cout << "Usage: " << argv[0] << " [path to JMMT PAK file]";
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

		if(!fs::exists(outpath.parent_path()))
			fs::create_directories(outpath.parent_path());

		std::ofstream ofs(outpath.string(), std::ofstream::binary);
		if(!ofs) {
			std::cerr << "Could not open \"" << outpath.string() << "\" for writing.\n";
			return;
		}

		ofs.write(reinterpret_cast<const char*>(file.data.data()), static_cast<std::streampos>(file.data.size()));
		ofs.close();

		std::cout << "Wrote \"" << outpath.string() << "\" to disk.\n";
	});

	std::cout << "Finished extracting successfully.\n";

	return 0;
}