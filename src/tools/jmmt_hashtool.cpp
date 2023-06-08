// JMMT HashTool

#include <jmmt/crc.h>

#include <array>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <string_view>

struct Arguments {
	enum class OutputMode {
		Hex,	///< Hexadecimal output.
		Decimal ///< Decimal output.
	};

	char* hashName {};
	bool useCase { false };
	OutputMode outMode { OutputMode::Hex };

	/** Parse arguments from the main() argv. **/
	static Arguments FromArgv(int argc, char** argv) {
		Arguments args;
		args.progname = argv[0];

		// no options provided
		if(argc == 1) {
			args.DispHelp();
			std::exit(1);
		}

		// non-pepper getopt(). I'm too lazy to make it any better though
		for(int i = 1; i < argc; ++i) {
			if(argv[i][0] == '-' && argv[i][1] != '\0') {
				char sw = argv[i][1];

				switch(sw) {
						// flag options

					case 'c':
						args.useCase = true;
						break;

					case 'd':
						if(args.outMode == OutputMode::Hex)
							args.outMode = OutputMode::Decimal;
						break;

					case 'h':
						if(args.outMode == OutputMode::Decimal)
							args.outMode = OutputMode::Hex;
						break;

					// terminals
					case '?':
						args.DispHelp();
						std::exit(0);

					default:
						std::printf("Unknown command-line switch '-%c'\n", sw);
						args.DispHelp();
						std::exit(1);
				}
			} else {
				// Assume any non-positional argument is what we're supposed to hash
				args.hashName = argv[i];
			}
		}
		return args;
	}

	bool Validate() const {
		if(!hashName) {
			std::printf("No hash name provided\n");
			DispHelp();
			return false;
		}
		return true;
	}

   private:
	char* progname {};

	void DispHelp() const {
		// no I'm not sorry
		std::printf(
		// clang-format off
			"JMMT HashTool - a thing for generating TECH HashID's\n"
			"Usage: %s [-c] [-?] <hash object>\n"
			"    -c Use case-senstive HashID variant (default is case-insensitive)\n"
			"    -d Output in decimal (default hex)\n"
			"    -h Output as hexadecimal (if previously overridden; kinda pointless)\n"
			"    -? Show this help message (and exit)\n",
			progname
		// clang-format on
		);
	}
};

int main(int argc, char** argv) {
	auto args = Arguments::FromArgv(argc, argv);

	if(!args.Validate()) {
		return 1;
	}

	jmmt::crc32_t result {};
	if(args.useCase)
		result = jmmt::HashStringCase(args.hashName);
	else
		result = jmmt::HashString(args.hashName);

	// clang-format off
	using enum Arguments::OutputMode;
	switch(args.outMode) {
		case Decimal: std::printf("%d\n", result);  break;	
		case Hex: std::printf("0x%08x\n", result);  break;
#ifdef __GNUC__
		// Mark this path explicitly as UB
		default: __builtin_unreachable();
#endif
	}
	// clang-format on
	return 0;
}
