// JMMT .DAT file renamer utility thingy
//
// Renames the .DAT files in /DATA on the disc to
// the original filenames, for easier identification,
// less pain, and.. well just because a bunch of DAT
// files is really stupid to go through every time.
//
// (C) 2022 modeco80.
//
// Usage:
//
// - Compile the tool (or scream at me for a binary)
// - Run the tool in the DATA directory of the files
// - ...
// - Profit?

#include <array>
#include <cstdint>
#include <cstdio>
#include <filesystem>
#include <string>
#include <string_view>
namespace fs = std::filesystem;

#include <jmmt/crc.h>

// These are the original filenames that the game tries to load,
// extracted from the game binary.
//
// We could brute-force these, but since the game has them in executable,
// it's a whole lot faster to just try every game filename and see
// what sticks (& rename it if it does).
constinit static std::array<std::string_view, 20> OriginalFilenames = {
	// First loaded by the game
	"package.toc",

	// General packs
	"config.pak",

	// This file is referenced in the game files,
	// but doesn't seem to exist anymore in the final build.
	//"shell.pak",

	"shell_character_select.pak",
	"shell_main.pak",
	"shell_title.pak",
	"shell_venue.pak",
	"shell_event.pak",
	"shell_option.pak",
	"win_screens.pak",

	// Game levels
	"SF_san_fran.pak",
	"DC_washington.pak",
	"MK_MT_KILI.pak",
	"MP_MACHU_PIHU.pak",
	"LV_Las_Vegas.pak",
	"AN_ANTARTICA.pak",
	"NP_Nepal.pak",
	"TH_TAHOE.pak",
	"VA_Valdez_alaska.pak",
	"RV_Rome.pak",
	"TR_training.pak"
};

std::string MakeDatFilename(const char* filename) {
	char datFile[13] {};

	// .DAT and .MET filenames are formatted like "[hex char * 8].DAT"
	// The name component is the CRC32 of the original filename.
	//
	// The DAT/MET filename can be a max of 13 characters long.
	int res = std::snprintf(&datFile[0], 13, "%X.DAT", jmmt::HashString(filename));

	// FIXME: probably throw exception
	if(res == -1)
		return "";

	return { &datFile[0], static_cast<std::size_t>(res) };
}

int main() {
	int renamedFiles = 0;

	for(auto filename : OriginalFilenames) {
		auto datFile = MakeDatFilename(filename.data());

		if(fs::exists(datFile)) {
			// Try to rename the .DAT file to the game filename.
			try {
				fs::rename(datFile, filename);
			} catch(std::exception& ex) {
				// If there's an error renaming, we already catch
				// if the source .DAT file (that's supposed to exist)
				// doesn't exist, so print the exception and exit.
				std::printf("Got exception: %s\n", ex.what());
				return 1;
			}

			std::printf("\"%s\" -> \"%s\"\n", datFile.c_str(), filename.data());
			renamedFiles++;
		} else {
			// FIXME: should probably stop here?
			std::printf("???? Generated hash filename \"%s\" (for \"%s\") which does not exist on disk\n", datFile.c_str(), filename.data());
		}
	}

	std::printf("Renamed %d files successfully.\n", renamedFiles);
	return 0;
}
