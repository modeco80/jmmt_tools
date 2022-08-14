// ImHex pattern for .ps2_texture or texture_*.b files

struct Tex1Header {
	u8 magic[4];
	
	// I don't know what these are?
	u32 unk;
	u16 unk2;
	
	u16 width;
	u16 height;
	
	// bpp of the texture.
	// > 8 = no palette file
	u16 bpp;
	
	u8 unknown[0x40];
};

Tex1Header tex1 @ $;
