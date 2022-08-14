// ImHex pattern for .ps2_palette or palette_*.b files

struct Pal1Header {
	u8 magic[4];
	
	u32 unk;
	u16 unk2;
	
	// This is probably implied
	// by bpp of the image, I guess
	u16 NrColors;
	
	u16 PaletteBpp;
	
	u16 unk3;
	
	u32 HeaderSize;
	
	// Don't know what this data is
	u8 unk4[0x1c];
};

struct Pal1File {
	Pal1Header Header;

	if(Header.PaletteBpp == 32) {
		u32 Colors[Header.NrColors];
	} else if(Header.PaletteBpp == 16) {
		u16 Colors[Header.Nrcolors];
	}
};


Pal1File Pal1 @ 0x0;

