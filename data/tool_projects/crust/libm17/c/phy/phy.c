//-------------------------------
// M17 C library - phy/interleave.c
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 28 December 2023
//-------------------------------

#include <m17.h>

//interleaver pattern
const uint16_t intrl_seq[SYM_PER_PLD*2]=
{
	0, 137, 90, 227, 180, 317, 270, 39, 360, 129, 82, 219, 172, 309, 262, 31,
	352, 121, 74, 211, 164, 301, 254, 23, 344, 113, 66, 203, 156, 293, 246, 15,
	336, 105, 58, 195, 148, 285, 238, 7, 328, 97, 50, 187, 140, 277, 230, 367,
	320, 89, 42, 179, 132, 269, 222, 359, 312, 81, 34, 171, 124, 261, 214, 351,
	304, 73, 26, 163, 116, 253, 206, 343, 296, 65, 18, 155, 108, 245, 198, 335,
	288, 57, 10, 147, 100, 237, 190, 327, 280, 49, 2, 139, 92, 229, 182, 319,
	272, 41, 362, 131, 84, 221, 174, 311, 264, 33, 354, 123, 76, 213, 166, 303,
	256, 25, 346, 115, 68, 205, 158, 295, 248, 17, 338, 107, 60, 197, 150, 287,
	240, 9, 330, 99, 52, 189, 142, 279, 232, 1, 322, 91, 44, 181, 134, 271,
	224, 361, 314, 83, 36, 173, 126, 263, 216, 353, 306, 75, 28, 165, 118, 255,
	208, 345, 298, 67, 20, 157, 110, 247, 200, 337, 290, 59, 12, 149, 102, 239,
	192, 329, 282, 51, 4, 141, 94, 231, 184, 321, 274, 43, 364, 133, 86, 223,
	176, 313, 266, 35, 356, 125, 78, 215, 168, 305, 258, 27, 348, 117, 70, 207,
	160, 297, 250, 19, 340, 109, 62, 199, 152, 289, 242, 11, 332, 101, 54, 191,
	144, 281, 234, 3, 324, 93, 46, 183, 136, 273, 226, 363, 316, 85, 38, 175,
	128, 265, 218, 355, 308, 77, 30, 167, 120, 257, 210, 347, 300, 69, 22, 159,
	112, 249, 202, 339, 292, 61, 14, 151, 104, 241, 194, 331, 284, 53, 6, 143,
	96, 233, 186, 323, 276, 45, 366, 135, 88, 225, 178, 315, 268, 37, 358, 127,
	80, 217, 170, 307, 260, 29, 350, 119, 72, 209, 162, 299, 252, 21, 342, 111,
	64, 201, 154, 291, 244, 13, 334, 103, 56, 193, 146, 283, 236, 5, 326, 95,
	48, 185, 138, 275, 228, 365, 318, 87, 40, 177, 130, 267, 220, 357, 310, 79,
	32, 169, 122, 259, 212, 349, 302, 71, 24, 161, 114, 251, 204, 341, 294, 63,
	16, 153, 106, 243, 196, 333, 286, 55, 8, 145, 98, 235, 188, 325, 278, 47
};

/**
 * @brief Reorder (interleave) 368 unpacked payload bits.
 * 
 * @param outp Reordered, unpacked type-4 bits.
 * @param inp Input unpacked type-2/3 bits.
 */
void reorder_bits(uint8_t outp[SYM_PER_PLD*2], const uint8_t inp[SYM_PER_PLD*2])
{
	for(uint16_t i=0; i<SYM_PER_PLD*2; i++)
        outp[i]=inp[intrl_seq[i]];
}

/**
 * @brief Reorder (interleave) 368 soft bits.
 * 
 * @param outp Reordered soft bits.
 * @param inp Input soft bits.
 */
void reorder_soft_bits(uint16_t outp[SYM_PER_PLD*2], const uint16_t inp[SYM_PER_PLD*2])
{
	for(uint16_t i=0; i<SYM_PER_PLD*2; i++)
        outp[i]=inp[intrl_seq[i]];
}

//--------------------------------------------------------------------
// M17 C library - phy/randomize.c
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 29 December 2023
//--------------------------------------------------------------------

//randomizing pattern
const uint8_t rand_seq[46]=
{
    0xD6, 0xB5, 0xE2, 0x30, 0x82, 0xFF, 0x84, 0x62, 0xBA, 0x4E, 0x96, 0x90, 0xD8, 0x98, 0xDD, 0x5D, 0x0C, 0xC8, 0x52, 0x43, 0x91, 0x1D, 0xF8,
    0x6E, 0x68, 0x2F, 0x35, 0xDA, 0x14, 0xEA, 0xCD, 0x76, 0x19, 0x8D, 0xD5, 0x80, 0xD1, 0x33, 0x87, 0x13, 0x57, 0x18, 0x2D, 0x29, 0x78, 0xC3
};

/**
 * @brief Randomize type-4 unpacked bits.
 * 
 * @param inp Input 368 unpacked type-4 bits.
 */
void randomize_bits(uint8_t inp[SYM_PER_PLD*2])
{
    for(uint16_t i=0; i<SYM_PER_PLD*2; i++)
    {
        if((rand_seq[i/8]>>(7-(i%8)))&1) //flip bit if '1'
        {
            if(inp[i])
                inp[i]=0;
            else
                inp[i]=1;
        }
    }
}

/**
 * @brief Randomize type-4 soft bits.
 * 
 * @param inp Input 368 soft type-4 bits.
 */
void randomize_soft_bits(uint16_t inp[SYM_PER_PLD*2])
{
    for(uint16_t i=0; i<SYM_PER_PLD*2; i++)
    {
        if((rand_seq[i/8]>>(7-(i%8)))&1) //flip bit if '1'
        {
            inp[i]=soft_bit_NOT(inp[i]);
        }
    }
}

//-------------------------------
// M17 C library - phy/slice.c
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 14 January 2024
//-------------------------------

/**
 * @brief Slice payload symbols into soft dibits.
 * Input (RRC filtered baseband sampled at symbol centers)
 * should be already normalized to {-3, -1, +1 +3}.
 * @param out Soft valued dibits (type-4).
 * @param inp Array of 184 floats (1 sample per symbol).
 */
void slice_symbols(uint16_t out[2*SYM_PER_PLD], const float inp[SYM_PER_PLD])
{
    for(uint8_t i=0; i<SYM_PER_PLD; i++)
    {
        //bit 0
        if(inp[i]>=symbol_list[3])
        {
            out[i*2+1]=0xFFFF;
        }
        else if(inp[i]>=symbol_list[2])
        {
            out[i*2+1]=-(float)0xFFFF/(symbol_list[3]-symbol_list[2])*symbol_list[2]+inp[i]*(float)0xFFFF/(symbol_list[3]-symbol_list[2]);
        }
        else if(inp[i]>=symbol_list[1])
        {
            out[i*2+1]=0x0000;
        }
        else if(inp[i]>=symbol_list[0])
        {
            out[i*2+1]=(float)0xFFFF/(symbol_list[1]-symbol_list[0])*symbol_list[1]-inp[i]*(float)0xFFFF/(symbol_list[1]-symbol_list[0]);
        }
        else
        {
            out[i*2+1]=0xFFFF;
        }

        //bit 1
        if(inp[i]>=symbol_list[2])
        {
            out[i*2]=0x0000;
        }
        else if(inp[i]>=symbol_list[1])
        {
            out[i*2]=0x7FFF-inp[i]*(float)0xFFFF/(symbol_list[2]-symbol_list[1]);
        }
        else
        {
            out[i*2]=0xFFFF;
        }
    }
}

//--------------------------------------------------------------------
// M17 C library - phy/sync.c
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 29 December 2023
//--------------------------------------------------------------------

//syncwords
const uint16_t SYNC_LSF = 0x55F7;
const uint16_t SYNC_STR = 0xFF5D;
const uint16_t SYNC_PKT = 0x75FF;
const uint16_t SYNC_BER = 0xDF55;
const uint16_t EOT_MRKR = 0x555D;
