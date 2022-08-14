#ifndef JMMT_TOOLS_FOURCCOBJECT_H
#define JMMT_TOOLS_FOURCCOBJECT_H

#include <cstdint>

namespace jmmt {


	template <class TMagic, TMagic ValidMagic>
	struct BasicStructureWithMagic {
		constexpr static TMagic TypeMagic = ValidMagic;
	};

	template <uint32_t ValidMagic>
	using FourCCMagic = BasicStructureWithMagic<uint32_t, ValidMagic>;

}

#endif // JMMT_TOOLS_FOURCCOBJECT_H
