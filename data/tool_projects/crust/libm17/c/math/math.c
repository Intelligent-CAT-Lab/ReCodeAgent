//--------------------------------------------------------------------
// M17 C library - math/rrc.c
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 29 December 2023
//--------------------------------------------------------------------
#include <m17.h>
#include <string.h>


//sample RRC filter for 48kHz sample rate
//alpha=0.5, span=8, sps=10, gain=sqrt(sps)
const float rrc_taps_10[8*10+1]=
{
	-0.003195702904062073f, -0.002930279157647190f, -0.001940667871554463f,
	-0.000356087678023658f,  0.001547011339077758f,  0.003389554791179751f,
	 0.004761898604225673f,  0.005310860846138910f,  0.004824746306020221f,
	 0.003297923526848786f,  0.000958710871218619f, -0.001749908029791816f,
	-0.004238694106631223f, -0.005881783042101693f, -0.006150256456781309f,
	-0.004745376707651645f, -0.001704189656473565f,  0.002547854551539951f,
	 0.007215575568844704f,  0.011231038205363532f,  0.013421952197060707f,
	 0.012730475385624438f,  0.008449554307303753f,  0.000436744366018287f,
	-0.010735380379191660f, -0.023726883538258272f, -0.036498030780605324f,
	-0.046500883189991064f, -0.050979050575999614f, -0.047340680079891187f,
	-0.033554880492651755f, -0.008513823955725943f,  0.027696543159614194f,
	 0.073664520037517042f,  0.126689053778116234f,  0.182990955139333916f,
	 0.238080025892859704f,  0.287235637987091563f,  0.326040247765297220f,
	 0.350895727088112619f,  0.359452932027607974f,  0.350895727088112619f,
	 0.326040247765297220f,  0.287235637987091563f,  0.238080025892859704f,
	 0.182990955139333916f,  0.126689053778116234f,  0.073664520037517042f,
	 0.027696543159614194f, -0.008513823955725943f, -0.033554880492651755f,
	-0.047340680079891187f, -0.050979050575999614f, -0.046500883189991064f,
	-0.036498030780605324f, -0.023726883538258272f, -0.010735380379191660f,
	 0.000436744366018287f,  0.008449554307303753f,  0.012730475385624438f,
	 0.013421952197060707f,  0.011231038205363532f,  0.007215575568844704f,
	 0.002547854551539951f, -0.001704189656473565f, -0.004745376707651645f,
	-0.006150256456781309f, -0.005881783042101693f, -0.004238694106631223f,
	-0.001749908029791816f,  0.000958710871218619f,  0.003297923526848786f,
	 0.004824746306020221f,  0.005310860846138910f,  0.004761898604225673f,
	 0.003389554791179751f,  0.001547011339077758f, -0.000356087678023658f,
	-0.001940667871554463f, -0.002930279157647190f, -0.003195702904062073f
};

//sample RRC filter for 24kHz sample rate
//alpha=0.5, span=8, sps=5, gain=sqrt(sps)
const float rrc_taps_5[8*5+1]=
{
	-0.004519384154389f, -0.002744505321971f,
	 0.002187793653660f,  0.006734308458208f,
	 0.006823188093192f,  0.001355815246317f,
	-0.005994389201970f, -0.008697733303330f,
	-0.002410076268276f,  0.010204314627992f,
	 0.018981413448435f,  0.011949415510291f,
	-0.015182045838927f, -0.051615756197679f,
	-0.072094910038768f, -0.047453533621088f,
	 0.039168634270669f,  0.179164496628150f,
	 0.336694345124862f,  0.461088271869920f,
	 0.508340710642860f,  0.461088271869920f,
	 0.336694345124862f,  0.179164496628150f,
	 0.039168634270669f, -0.047453533621088f,
	-0.072094910038768f, -0.051615756197679f,
	-0.015182045838927f,  0.011949415510291f,
	 0.018981413448435f,  0.010204314627992f,
	-0.002410076268276f, -0.008697733303330f,
	-0.005994389201970f,  0.001355815246317f,
	 0.006823188093192f,  0.006734308458208f,
	 0.002187793653660f, -0.002744505321971f,
	-0.004519384154389f
};

//--------------------------------------------------------------------
// M17 C library - math/golay.c
//
// This file contains:
// - Golay(24, 12) encoder
// - Golay(24, 12) soft decoder with accompanying utility functions
// - Link Information Channel (LICH) decoder
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 29 December 2023
//--------------------------------------------------------------------


/**
 * @brief Precomputed encoding matrix for Golay(24, 12).
 *
 */
const uint16_t encode_matrix[12]=
{
    0x8eb, 0x93e, 0xa97, 0xdc6, 0x367, 0x6cd,
    0xd99, 0x3da, 0x7b4, 0xf68, 0x63b, 0xc75
};

/**
 * @brief Precomputed decoding matrix for Golay(24, 12).
 *
 */
const uint16_t decode_matrix[12]=
{
    0xc75, 0x49f, 0x93e, 0x6e3, 0xdc6, 0xf13,
    0xab9, 0x1ed, 0x3da, 0x7b4, 0xf68, 0xa4f
};

/**
 * @brief Encode a 12-bit value with Golay(24, 12).
 *
 * @param data 12-bit input value (right justified).
 * @return uint32_t 24-bit Golay codeword.
 */
uint32_t golay24_encode(const uint16_t data)
{
    uint16_t checksum=0;

    for(uint8_t i=0; i<12; i++)
    {
        if(data&(1<<i))
        {
            checksum ^= encode_matrix[i];
        }
    }

    return ((uint32_t)data<<12) | checksum;
}

/**
 * @brief Soft-valued equivalent of `popcount()`
 *
 * @param in Pointer to an array holding soft logic vector.
 * @param siz Vector's size.
 * @return uint32_t Sum of all values.
 */
uint32_t s_popcount(const uint16_t* in, uint8_t siz)
{
	uint32_t tmp=0;

	for(uint8_t i=0; i<siz; i++)
		tmp+=in[i];

	return tmp;
}

/**
 * @brief
 *
 * @param out
 * @param value
 */
void s_calc_checksum(uint16_t* out, const uint16_t* value)
{
    uint16_t checksum[12];
    uint16_t soft_em[12]; //soft valued encoded matrix entry

    for(uint8_t i=0; i<12; i++)
    	checksum[i]=0;

    for(uint8_t i=0; i<12; i++)
    {
    	int_to_soft(soft_em, encode_matrix[i], 12);

        if(value[i]>0x7FFF)
        {
            soft_XOR(checksum, checksum, soft_em, 12);
        }
    }

    memcpy((uint8_t*)out, (uint8_t*)checksum, 12*2);
}

/**
 * @brief Detect errors in a soft-valued Golay(24, 12) codeword.
 *
 * @param codeword Input 24-bit soft codeword.
 * @return uint32_t Detected errors vector.
 */
uint32_t s_detect_errors(const uint16_t* codeword)
{
    uint16_t data[12];
    uint16_t parity[12];
    uint16_t cksum[12];
    uint16_t syndrome[12];
    uint32_t weight; //for soft popcount

	memcpy((uint8_t*)data, (uint8_t*)&codeword[12], 2*12);
	memcpy((uint8_t*)parity, (uint8_t*)&codeword[0], 2*12);

	s_calc_checksum(cksum, data);
	soft_XOR(syndrome, parity, cksum, 12);

	weight=s_popcount(syndrome, 12);

	//all (less than 4) errors in the parity part
    if(weight < 4*0xFFFE)
    {
    	//printf("1: %1.2f\n", (float)weight/0xFFFF);
        return soft_to_int(syndrome, 12);
    }

    //one of the errors in data part, up to 3 in parity
    for(uint8_t i = 0; i<12; i++)
    {
        uint16_t e = 1<<i;
        uint16_t coded_error = encode_matrix[i];
        uint16_t scoded_error[12]; //soft coded_error
        uint16_t sc[12]; //syndrome^coded_error

        int_to_soft(scoded_error, coded_error, 12);
        soft_XOR(sc, syndrome, scoded_error, 12);
        weight=s_popcount(sc, 12);

        if(weight < 3*0xFFFE)
        {
        	//printf("2: %1.2f\n", (float)weight/0xFFFF+1);
        	uint16_t s=soft_to_int(syndrome, 12);
            return (e << 12) | (s ^ coded_error);
        }
    }

    //two of the errors in data part and up to 2 in parity
    for(uint8_t i = 0; i<11; i++)
    {
    	for(uint8_t j = i+1; j<12; j++)
    	{
    		uint16_t e = (1<<i) | (1<<j);
        	uint16_t coded_error = encode_matrix[i]^encode_matrix[j];
        	uint16_t scoded_error[12]; //soft coded_error
	        uint16_t sc[12]; //syndrome^coded_error

	        int_to_soft(scoded_error, coded_error, 12);
	        soft_XOR(sc, syndrome, scoded_error, 12);
	        weight=s_popcount(sc, 12);

	        if(weight < 2*0xFFFF)
	        {
	        	//printf("3: %1.2f\n", (float)weight/0xFFFF+2);
	        	uint16_t s=soft_to_int(syndrome, 12);
	            return (e << 12) | (s ^ coded_error);
	        }
		}
    }

	//algebraic decoding magic
    uint16_t inv_syndrome[12]={0,0,0,0,0,0,0,0,0,0,0,0};
    uint16_t dm[12]; //soft decode matrix

    for(uint8_t i=0; i<12; i++)
    {
        if(syndrome[i] > 0x7FFF)
        {
        	int_to_soft(dm, decode_matrix[i], 12);
        	soft_XOR(inv_syndrome, inv_syndrome, dm, 12);
        }
    }

	//all (less than 4) errors in the data part
	weight=s_popcount(inv_syndrome, 12);
    if(weight < 4*0xFFFF)
    {
    	//printf("4: %1.2f\n", (float)weight/0xFFFF);
        return soft_to_int(inv_syndrome, 12) << 12;
    }

	//one error in parity bits, up to 3 in data - this part has some quirks, the reason remains unknown
    for(uint8_t i=0; i<12; i++)
    {
        uint16_t e = 1<<i;
        uint16_t coding_error = decode_matrix[i];

        uint16_t ce[12]; //soft coding error
        uint16_t tmp[12];

        int_to_soft(ce, coding_error, 12);
        soft_XOR(tmp, inv_syndrome, ce, 12);
        weight=s_popcount(tmp, 12);

        if(weight < 3*(0xFFFF+2))
        {
        	//printf("5: %1.2f\n", (float)weight/0xFFFF+1);
            return ((soft_to_int(inv_syndrome, 12) ^ coding_error) << 12) | e;
        }
    }

    return 0xFFFFFFFFUL;
}

/**
 * @brief Soft decode Golay(24, 12) codeword.
 *
 * @param codeword Pointer to a 24-element soft-valued (fixed-point) bit codeword.
 * @return uint16_t Decoded data.
 */
uint16_t golay24_sdecode(const uint16_t codeword[24])
{
    //match the bit order in M17
    uint16_t cw[24]; //local copy
    for(uint8_t i=0; i<24; i++)
        cw[i]=codeword[23-i];

    uint32_t errors = s_detect_errors(cw);

    if(errors == 0xFFFFFFFF)
		return 0xFFFF;

    return (((soft_to_int(&cw[0], 16) | (soft_to_int(&cw[16], 8) << 16)) ^ errors) >> 12) & 0x0FFF;
}

/**
 * @brief Soft decode LICH into a 6-byte array.
 *
 * @param outp An array of packed, decoded bits.
 * @param inp Pointer to an array of 96 soft bits.
 */
void decode_LICH(uint8_t outp[6], const uint16_t inp[96])
{
    uint16_t tmp;

    memset(outp, 0, 6);

    tmp=golay24_sdecode(&inp[0]);
    outp[0]=(tmp>>4)&0xFF;
    outp[1]|=(tmp&0xF)<<4;
    tmp=golay24_sdecode(&inp[1*24]);
    outp[1]|=(tmp>>8)&0xF;
    outp[2]=tmp&0xFF;
    tmp=golay24_sdecode(&inp[2*24]);
    outp[3]=(tmp>>4)&0xFF;
    outp[4]|=(tmp&0xF)<<4;
    tmp=golay24_sdecode(&inp[3*24]);
    outp[4]|=(tmp>>8)&0xF;
    outp[5]=tmp&0xFF;
}

void encode_LICH(uint8_t outp[12], const uint8_t inp[6])
{
    uint32_t val;

    val=golay24_encode((inp[0]<<4)|(inp[1]>>4));
    outp[0]=(val>>16)&0xFF;
    outp[1]=(val>>8)&0xFF;
    outp[2]=(val>>0)&0xFF;
    val=golay24_encode(((inp[1]&0x0F)<<8)|inp[2]);
    outp[3]=(val>>16)&0xFF;
    outp[4]=(val>>8)&0xFF;
    outp[5]=(val>>0)&0xFF;
    val=golay24_encode((inp[3]<<4)|(inp[4]>>4));
    outp[6]=(val>>16)&0xFF;
    outp[7]=(val>>8)&0xFF;
    outp[8]=(val>>0)&0xFF;
    val=golay24_encode(((inp[4]&0x0F)<<8)|inp[5]);
    outp[9]=(val>>16)&0xFF;
    outp[10]=(val>>8)&0xFF;
    outp[11]=(val>>0)&0xFF;
}

//--------------------------------------------------------------------
// M17 C library - math/rrc.c
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 29 December 2023
//--------------------------------------------------------------------

//sample RRC filter for 48kHz sample rate
//alpha=0.5, span=8, sps=10, gain=sqrt(sps)
// const float rrc_taps_10[8*10+1]=
// {
// 	-0.003195702904062073f, -0.002930279157647190f, -0.001940667871554463f,
// 	-0.000356087678023658f,  0.001547011339077758f,  0.003389554791179751f,
// 	 0.004761898604225673f,  0.005310860846138910f,  0.004824746306020221f,
// 	 0.003297923526848786f,  0.000958710871218619f, -0.001749908029791816f,
// 	-0.004238694106631223f, -0.005881783042101693f, -0.006150256456781309f,
// 	-0.004745376707651645f, -0.001704189656473565f,  0.002547854551539951f,
// 	 0.007215575568844704f,  0.011231038205363532f,  0.013421952197060707f,
// 	 0.012730475385624438f,  0.008449554307303753f,  0.000436744366018287f,
// 	-0.010735380379191660f, -0.023726883538258272f, -0.036498030780605324f,
// 	-0.046500883189991064f, -0.050979050575999614f, -0.047340680079891187f,
// 	-0.033554880492651755f, -0.008513823955725943f,  0.027696543159614194f,
// 	 0.073664520037517042f,  0.126689053778116234f,  0.182990955139333916f,
// 	 0.238080025892859704f,  0.287235637987091563f,  0.326040247765297220f,
// 	 0.350895727088112619f,  0.359452932027607974f,  0.350895727088112619f,
// 	 0.326040247765297220f,  0.287235637987091563f,  0.238080025892859704f,
// 	 0.182990955139333916f,  0.126689053778116234f,  0.073664520037517042f,
// 	 0.027696543159614194f, -0.008513823955725943f, -0.033554880492651755f,
// 	-0.047340680079891187f, -0.050979050575999614f, -0.046500883189991064f,
// 	-0.036498030780605324f, -0.023726883538258272f, -0.010735380379191660f,
// 	 0.000436744366018287f,  0.008449554307303753f,  0.012730475385624438f,
// 	 0.013421952197060707f,  0.011231038205363532f,  0.007215575568844704f,
// 	 0.002547854551539951f, -0.001704189656473565f, -0.004745376707651645f,
// 	-0.006150256456781309f, -0.005881783042101693f, -0.004238694106631223f,
// 	-0.001749908029791816f,  0.000958710871218619f,  0.003297923526848786f,
// 	 0.004824746306020221f,  0.005310860846138910f,  0.004761898604225673f,
// 	 0.003389554791179751f,  0.001547011339077758f, -0.000356087678023658f,
// 	-0.001940667871554463f, -0.002930279157647190f, -0.003195702904062073f
// };

// //sample RRC filter for 24kHz sample rate
// //alpha=0.5, span=8, sps=5, gain=sqrt(sps)
// onst float rrc_taps_5[8*5+1]=
// {
// 	-0.004519384154389f, -0.002744505321971f,
// 	 0.002187793653660f,  0.006734308458208f,
// 	 0.006823188093192f,  0.001355815246317f,
// 	-0.005994389201970f, -0.008697733303330f,
// 	-0.002410076268276f,  0.010204314627992f,
// 	 0.018981413448435f,  0.011949415510291f,
// 	-0.015182045838927f, -0.051615756197679f,
// 	-0.072094910038768f, -0.047453533621088f,
// 	 0.039168634270669f,  0.179164496628150f,
// 	 0.336694345124862f,  0.461088271869920f,
// 	 0.508340710642860f,  0.461088271869920f,
// 	 0.336694345124862f,  0.179164496628150f,
// 	 0.039168634270669f, -0.047453533621088f,
// 	-0.072094910038768f, -0.051615756197679f,
// 	-0.015182045838927f,  0.011949415510291f,
// 	 0.018981413448435f,  0.010204314627992f,
// 	-0.002410076268276f, -0.008697733303330f,
// 	-0.005994389201970f,  0.001355815246317f,
// 	 0.006823188093192f,  0.006734308458208f,
// 	 0.002187793653660f, -0.002744505321971f,
// 	-0.004519384154389f
// };


//--------------------------------------------------------------------
// M17 C library - math/math.c
//
// This file contains:
// - absolute difference value
// - Euclidean norm (L2) calculation for n-dimensional vectors (float)
// - soft-valued arrays to integer conversion (and vice-versa)
// - fixed-valued multiplication and division
//
// Wojciech Kaczmarski, SP5WWP
// M17 Project, 29 December 2023
//--------------------------------------------------------------------
// #include <math.h>
// #include <m17.h>

/**
 * @brief Utility function returning the absolute value of a difference between
 * two fixed-point values.
 *
 * @param v1 First value.
 * @param v2 Second value.
 * @return abs(v1-v2) value.
 */
uint16_t q_abs_diff(const uint16_t v1, const uint16_t v2)
{
    if(v2 > v1) return v2 - v1;
    return v1 - v2;
}

/**
 * @brief Calculate L2 norm between two n-dimensional vectors of floats.
 *
 * @param in1 Vector 1.
 * @param in2 Vector 2.
 * @param n Vectors' size.
 * @return float L2 norm.
 */
float eucl_norm(const float* in1, const int8_t* in2, const uint8_t n)
{
    float tmp = 0.0f;

    for(uint8_t i=0; i<n; i++)
    {
        tmp += (in1[i]-(float)in2[i])*(in1[i]-(float)in2[i]);
    }

    return sqrtf(tmp);
}

/**
 * @brief Convert an unsigned int into an array of soft, fixed-point values.
 * Maximum length is 16. LSB is at index 0.
 * @param out Pointer to an array of uint16_t.
 * @param in Input value.
 * @param len Input's bit length.
 */
void int_to_soft(uint16_t* out, const uint16_t in, const uint8_t len)
{
	for(uint8_t i=0; i<len; i++)
	{
		(in>>i)&1 ? (out[i]=0xFFFF) : (out[i]=0);
	}
}

/**
 * @brief Convert an array of soft, fixed-point
 * Maximum length is 16. LSB is at index 0.
 * @param in Pointer to an array of uint16_t.
 * @param len Input's length.
 * @return uint16_t Return value.
 */
uint16_t soft_to_int(const uint16_t* in, const uint8_t len)
{
	uint16_t tmp=0;

	for(uint8_t i=0; i<len; i++)
	{
		if(in[i]>0x7FFFU)
			tmp|=(1<<i);
	}

	return tmp;
}

/**
 * @brief 1st quadrant fixed point addition with saturation.
 *
 * @param a Addend 1.
 * @param b Addend 2.
 * @return uint16_t Sum = a+b.
 */
uint16_t add16(const uint16_t a, const uint16_t b)
{
	uint32_t r=(uint32_t)a+b;
	
	return r<=0xFFFFU ? r : 0xFFFFU;
}

/**
 * @brief 1st quadrant fixed point subtraction with saturation.
 *
 * @param a Minuend.
 * @param b Subtrahent.
 * @return uint16_t Difference = a-b.
 */
uint16_t sub16(const uint16_t a, const uint16_t b)
{	
	if(a>=b)
		return a-b;
	else
		return 0x0000U;
}

/**
 * @brief 1st quadrant fixed point division with saturation.
 *
 * @param a Dividend.
 * @param b Divisor.
 * @return uint16_t Quotient = a/b.
 */
uint16_t div16(const uint16_t a, const uint16_t b)
{
	uint32_t aa=(uint32_t)a<<16;
	uint32_t r=aa/b;

	return r<=0xFFFFU ? r : 0xFFFFU;
}

/**
 * @brief 1st quadrant fixed point multiplication.
 *
 * @param a Multiplicand.
 * @param b Multiplier.
 * @return uint16_t Product = a*b.
 */
uint16_t mul16(const uint16_t a, const uint16_t b)
{
	return (uint16_t)(((uint32_t)a*b)>>16);
}

/**
 * @brief Bilinear interpolation (soft-valued expansion) for XOR.
 * This approach retains XOR(0.5, 0.5)=0.5
 * https://math.stackexchange.com/questions/3505934/evaluation-of-not-and-xor-in-fuzzy-logic-rules
 * @param a Input A.
 * @param b Input B.
 * @return uint16_t Output = A xor B.
 */
uint16_t soft_bit_XOR(const uint16_t a, const uint16_t b)
{
	//a(1-b)+b(1-a)
	//return mul16(div16(0xFFFF-b, 0xFFFF), div16(a, 0xFFFF)) + mul16(div16(b, 0xFFFF), div16(0xFFFF-a, 0xFFFF));
	return add16(mul16(a, sub16(0xFFFF, b)), mul16(b, sub16(0xFFFF, a)));
}

/**
 * @brief Soft logic NOT.
 * 
 * @param a Input A.
 * @return uint16_t Output = not A.
 */
uint16_t soft_bit_NOT(const uint16_t a)
{
	return 0xFFFFU-a;
}

/**
 * @brief XOR for vectors of soft-valued logic.
 * Max length is 255.
 * @param out Output vector = A xor B.
 * @param a Input vector A.
 * @param b Input vector B.
 * @param len Vectors' size.
 */
void soft_XOR(uint16_t* out, const uint16_t* a, const uint16_t* b, const uint8_t len)
{
	for(uint8_t i=0; i<len; i++)
		out[i]=soft_bit_XOR(a[i], b[i]);
}
