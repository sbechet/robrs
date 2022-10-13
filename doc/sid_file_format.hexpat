
// ImHex Pattern helper...

#pragma MIME audio/prs.sid

bitfield Speed {
	tune32: 1;
	tune31: 1;
	tune30: 1;
	tune29: 1;
	tune28: 1;
	tune27: 1;
	tune26: 1;
	tune25: 1;
	tune24: 1;
	tune23: 1;
	tune22: 1;
	tune21: 1;
	tune20: 1;
	tune19: 1;
	tune18: 1;
	tune17: 1;
	tune16: 1;
	tune15: 1;
	tune14: 1;
	tune13: 1;
	tune12: 1;
	tune11: 1;
	tune10: 1;
	tune9: 1;
	tune8: 1;
	tune7: 1;
	tune6: 1;
	tune5: 1;
	tune4: 1;
	tune3: 1;
	tune2: 1;
	tune1: 1;
};

struct SoundInterfaceDesign {
	char psid[4] [[comment("PSID or RSID Signature")]];
	be u16 version [[comment("SID file version")]];
	be u16 dataOffset [[comment("SID file data offset")]];
	le u16 loadAddress [[comment("C64 memory load address")]];
	be u16 initAddress [[comment("C64 memory init address")]];
	be u16 playAddress [[comment("C64 memory play address")]];
	be u16 songs [[comment("SID file number of songs")]];
	be u16 startSong [[comment("SID file start song")]];
	be Speed speed [[comment("1 bit per song. 0:vbank 1:CIA 1 timer")]];
	char title[32] [[comment("title")]];
	char author[32] [[comment("author")]];
	char release[32] [[comment("copyright")]];
};

enum SidModel: u8 {
	Unknown = 0,
	MOS6581 = 1,
	MOS8580 = 2,
	MOS6581_and_MOS8580 = 3
};

bitfield Flags {
	reserved: 9;
	sidModel: 2;
	clock: 2;
	psidSpecific: 1;
	format: 1 [[comment("bit0=0:buit-in music player")]];
};


struct SoudInterfaceDesign2: SoundInterfaceDesign {
	be Flags flags;
	char startPage [[comment("relocStartPage")]];
	char pageLength [[comment("relocPages")]];
	be u16 reserved;
};

struct DataInfo {
	le u16 dataLoadAddress [[comment("LE load address")]];
};

SoudInterfaceDesign2 sid @0x00;
DataInfo data @sid.dataOffset;

