//--------------------------------------------------------------------
// M17 C library - payload/call.c
//
// This file contains:
// - callsign encoders and decoders
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 24 September 2024
//--------------------------------------------------------------------
#include <stdio.h>
#include <string.h>
#include <m17.h>

/**
 * @brief Decode a 48-bit value (stored as uint64_t) into callsign string.
 * 
 * @param outp Decoded callsign string.
 * @param inp Encoded value.
 */
void decode_callsign_value(uint8_t *outp, const uint64_t inp)
{
    uint64_t encoded=inp;
    uint8_t start=0; //where to put the first decoded, non-# character

	//address range check
	if(encoded>=U40_9)
	{
        if(encoded==0xFFFFFFFFFFFF) //broadcast special address
        {
            sprintf((char*)outp, "@ALL");
            return;
        }
        else if(encoded<=U40_9_8) //#-address range
        {
            start=1;
            encoded-=U40_9;
            outp[0]='#';
        }
        else //reserved address range
        {
            return;
        }
	}

	//decode the callsign
	uint8_t i=start;
	while(encoded>0)
	{
		outp[i]=CHAR_MAP[encoded%40];
		encoded/=40;
		i++;
	}
	outp[i]=0;
}

/**
 * @brief Decode a 6-byte long array (big-endian) into callsign string.
 * 
 * @param outp Decoded callsign string.
 * @param inp Pointer to a byte array holding the encoded value (big-endian).
 */
void decode_callsign_bytes(uint8_t *outp, const uint8_t inp[6])
{
	uint64_t encoded=0;

	//repack the data to a uint64_t
	for(uint8_t i=0; i<6; i++)
		encoded|=(uint64_t)inp[5-i]<<(8*i);

	decode_callsign_value(outp, encoded);
}

/**
 * @brief Encode callsign string into a 48-bit value, stored as uint64_t.
 * 
 * @param out Pointer to a uint64_t variable for the encoded value.
 * @param inp Callsign string.
 * @return int8_t Return value, 0 -> OK.
 */
int8_t encode_callsign_value(uint64_t *out, const uint8_t *inp)
{
    //assert inp length
    if(strlen((const char*)inp)>9)
        return -1;

    const uint8_t charMap[40]=CHAR_MAP;

    uint64_t tmp=0;
    uint8_t start=0; //where's the first char of the address? this excludes the leading #, if present

    //a special address that's encoded differently
    if(strcmp((const char*)inp, "@ALL")==0)
    {
        *out=0xFFFFFFFFFFFF;
        return 0;
    }

    //check if the address is in the hash-space
    if(inp[0]=='#')
        start=1;

    for(int8_t i=strlen((const char*)inp)-1; i>=start; i--)
    {
        for(uint8_t j=0; j<40; j++)
        {
            if(inp[i]==charMap[j])
            {
                tmp=tmp*40+j;
                break;
            }
        }
    }

    if(start) //starts with a hash?
        tmp+=U40_9; //40^9

    *out=tmp;
    return 0;
}

/**
 * @brief Encode callsign string and store in a 6-byte array (big-endian)
 * 
 * @param out Pointer to a byte array for the encoded value (big-endian).
 * @param inp Callsign string.
 * @return int8_t Return value, 0 -> OK.
 */
int8_t encode_callsign_bytes(uint8_t out[6], const uint8_t *inp)
{
    uint64_t tmp=0;

    if(encode_callsign_value(&tmp, inp))
    {
        return -1;
    }

	for(uint8_t i=0; i<6; i++)
    	out[5-i]=(tmp>>(8*i))&0xFF;
    	
    return 0;
}


//--------------------------------------------------------------------
// M17 C library - payload/crc.c
//
// This file contains:
// - CRC calculating functions (arbitrary length)
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 29 December 2023
//--------------------------------------------------------------------


//M17 CRC polynomial
const uint16_t M17_CRC_POLY = 0x5935;

/**
 * @brief Calculate CRC value.
 *
 * @param in Pointer to the input byte array.
 * @param len Input's length.
 * @return uint16_t CRC value.
 */
uint16_t CRC_M17(const uint8_t* in, const uint16_t len)
{
	uint32_t crc=0xFFFF; //init val

	for(uint16_t i=0; i<len; i++)
	{
		crc^=in[i]<<8;
		for(uint8_t j=0; j<8; j++)
		{
			crc<<=1;
			if(crc&0x10000)
				crc=(crc^M17_CRC_POLY)&0xFFFF;
		}
	}

	return crc&(0xFFFF);
}

/**
 * @brief Calculate CRC value for the Link Setup Frame.
 *
 * @param in Pointer to an LSF struct.
 * @return uint16_t CRC value.
 */
uint16_t LSF_CRC(const lsf_t* in)
{
    uint8_t d[28];

    memcpy(&d[0], in->dst, 6);
    memcpy(&d[6], in->src, 6);
    memcpy(&d[12], in->type, 2);
    memcpy(&d[14], in->meta, 14);

    return CRC_M17(d, 28);
}

//--------------------------------------------------------------------
// M17 C library - payload/lich.c
//
// This file contains:
// - Link Information CHannel (LICH) repacking functions
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 8 January 2024
//--------------------------------------------------------------------

/**
 * @brief Extract LICH from the whole LSF.
 * 
 * @param outp 6-byte array for the LICH.
 * @param cnt LICH counter (0 to 5)
 * @param inp Pointer to an LSF struct.
 */
void extract_LICH(uint8_t outp[6], const uint8_t cnt, const lsf_t* inp)
{
    switch(cnt)
    {
        case 0:
            outp[0]=inp->dst[0];
            outp[1]=inp->dst[1];
            outp[2]=inp->dst[2];
            outp[3]=inp->dst[3];
            outp[4]=inp->dst[4];
        break;

        case 1:
            outp[0]=inp->dst[5];
            outp[1]=inp->src[0];
            outp[2]=inp->src[1];
            outp[3]=inp->src[2];
            outp[4]=inp->src[3];
        break;

        case 2:
            outp[0]=inp->src[4];
            outp[1]=inp->src[5];
            outp[2]=inp->type[0];
            outp[3]=inp->type[1];
            outp[4]=inp->meta[0];
        break;

        case 3:
            outp[0]=inp->meta[1];
            outp[1]=inp->meta[2];
            outp[2]=inp->meta[3];
            outp[3]=inp->meta[4];
            outp[4]=inp->meta[5];
        break;

        case 4:
            outp[0]=inp->meta[6];
            outp[1]=inp->meta[7];
            outp[2]=inp->meta[8];
            outp[3]=inp->meta[9];
            outp[4]=inp->meta[10];
        break;

        case 5:
            outp[0]=inp->meta[11];
            outp[1]=inp->meta[12];
            outp[2]=inp->meta[13];
            outp[3]=inp->crc[0];
            outp[4]=inp->crc[1];
        break;

        default:
            ;
        break;
    }

    outp[5]=cnt<<5;
}

/**
 * @brief Unpack LICH bytes.
 * 
 * @param out Unpacked, encoded LICH bits (array of at least 96 bytes).
 * @param in 12-byte (96 bits) encoded LICH, packed.
 */
void unpack_LICH(uint8_t *out, const uint8_t in[12])
{
    for(uint8_t i=0; i<12; i++)
    {
        for(uint8_t j=0; j<8; j++)
            out[i*8+j]=(in[i]>>(7-j))&1;
    }
}
