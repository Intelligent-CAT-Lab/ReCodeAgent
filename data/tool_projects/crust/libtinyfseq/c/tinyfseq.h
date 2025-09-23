/**
 * libtinyfseq v3.2.0 (2024-05-19)
 * https://github.com/Cryptkeeper/libtinyfseq
 * MIT License
 *
 * `TINYFSEQ_IMPLEMENTATION` should be defined in a SINGLE C/C++ source code file to include the function implementations.
 * More information on using single-file libraries: https://github.com/nothings/stb#how-do-i-use-these-libraries
 */
#ifndef TINYFSEQ_H
#define TINYFSEQ_H

#include <stdint.h>

#define TINYFSEQ_VERSION "3.2.0"

typedef enum tf_err_t {
    TF_OK = 0,
    TF_EINVALID_MAGIC,
    TF_EINVALID_COMPRESSION_TYPE,
    TF_EINVALID_BUFFER_SIZE,
    TF_EINVALID_VAR_SIZE,
} TFError;

/**
 * `TFError_string` returns a string version of the enum name, including a comment with additional error context.
 * `TFError_string` does not allocate, its return values should not be freed.
 *
 * @param err Unknown/invalid values will default to returning "unknown TFError value"
 * @return A string version of the given value, never NULL
 */
const char *TFError_string(TFError err);

typedef enum tf_compression_type_t {
    TF_COMPRESSION_NONE,
    TF_COMPRESSION_ZSTD,
    TF_COMPRESSION_ZLIB,
} TFCompressionType;

typedef struct tf_header_t {
    uint16_t channelDataOffset;
    uint8_t minorVersion;
    uint8_t majorVersion;
    uint16_t variableDataOffset;
    uint32_t channelCount;
    uint32_t frameCount;
    uint8_t frameStepTimeMillis;
    enum tf_compression_type_t compressionType;
    uint8_t compressionBlockCount;
    uint8_t channelRangeCount;
    uint64_t sequenceUid;
} TFHeader;

/**
 * `TFHeader_read` selectively decodes the fields available within `TFHeader` from a given data buffer.
 * The buffer should be at least 32 bytes in length (the minimum size of a FSEQ v2.0+ header).
 *
 * @param bd Data buffer to decode into the given `TFHeader` pointer
 * @param bs Size of the data buffer (>=32)
 * @param header `TFHeader` instance to decode data buffer fields into
 * @param ep End pointer which upon a successful return will point to the end of the structure within `bd` (may be NULL)
 * @return A `TFError` value indicating an error, if any, otherwise `TF_OK`
 */
TFError
TFHeader_read(const uint8_t *bd, int bs, TFHeader *header, uint8_t **ep);

typedef struct tf_compression_block_t {
    uint32_t firstFrameId;
    uint32_t size;
} TFCompressionBlock;

/**
 * `TFCompressionBlock_read` decodes the fields available within `TFCompressionBlock` from a given data buffer.
 * The buffer should be at least 8 bytes in length (the fixed size of an encoded compression block).
 *
 * @param bd Data buffer to decode into the given `TFCompressionBlock` pointer
 * @param bs Size of the data buffer (>=8)
 * @param block `TFCompressionBlock` instance to decode data buffer fields into
 * @param ep End pointer which upon a successful return will point to the end of the structure within `bd` (may be NULL)
 * @return A `TFError` value indicating an error, if any, otherwise `TF_OK`
 */
TFError TFCompressionBlock_read(const uint8_t *bd,
                                int bs,
                                TFCompressionBlock *block,
                                uint8_t **ep);

typedef struct tf_var_header_t {
    uint16_t size;
    uint8_t id[2];
} TFVarHeader;

/**
 * `TFVarHeader_read` decodes the fields available within `TFVarHeader` from a given data buffer.
 * The buffer should be at least 5 bytes in length (the minimum size of an encoded variable header).
 *
 * An additional data buffer (`vd`) is used to store the decoded variable value (compared to `TFVarHeader` which
 * only contains the header fields). A NULL `vd` value will skip this decoding step.
 *
 * @param bd Data buffer to decode into the given `TFVarHeader` pointer
 * @param bs Size of the data buffer (>=6)
 * @param varHeader `TFVarHeader` instance to decode data buffer fields into
 * @param vd Optional data buffer for storing the corresponding value of the variable header (may be NULL)
 * @param vs Size of the optional data buffer
 * @param ep End pointer which upon a successful return will point to the end of the structure within `bd` (may be NULL)
 * @return A `TFError` value indicating an error, if any, otherwise `TF_OK`
 */
TFError TFVarHeader_read(const uint8_t *bd,
                         int bs,
                         TFVarHeader *varHeader,
                         uint8_t *vd,
                         int vs,
                         uint8_t **ep);

typedef struct tf_channel_range_t {
    uint32_t firstChannelNumber;
    uint32_t channelCount;
} TFChannelRange;

/**
 * `TFChannelRange_Read` decodes the fields available within `TFChannelRange` from a given data buffer.
 * The buffer should be at least 6 bytes in length (the fixed size of an encoded channel range).
 *
 * @param bd Data buffer to decode into the given `TFChannelRange` pointer
 * @param bs Size of the data buffer (>=6)
 * @param channelRange `TFChannelRange` instance to decode data buffer fields into
 * @param ep End pointer which upon a successful return will point to the end of the structure within `bd` (may be NULL)
 * @return A `TFError` value indicating an error, if any, otherwise `TF_OK`
 */
TFError TFChannelRange_read(const uint8_t *bd,
                            int bs,
                            TFChannelRange *channelRange,
                            uint8_t **ep);

#endif//TINYFSEQ_H

#ifdef TINYFSEQ_IMPLEMENTATION

const char *TFError_string(const TFError err) {
    switch (err) {
        case TF_OK:
            return "TF_OK (ok)";
        case TF_EINVALID_MAGIC:
            return "TF_EINVALID_MAGIC (invalid magic file signature)";
        case TF_EINVALID_COMPRESSION_TYPE:
            return "TF_EINVALID_COMPRESSION_TYPE (unknown compression "
                   "identifier)";
        case TF_EINVALID_BUFFER_SIZE:
            return "TF_EINVALID_BUFFER_SIZE (undersized data decoding buffer "
                   "argument)";
        case TF_EINVALID_VAR_SIZE:
            return "TF_EINVALID_VAR_SIZE (invalid variable size in header)";
        default:
            return "unknown TFError value";
    }
}

static int TFCompressionType_valid(const uint8_t b) {
    switch (b) {
        case TF_COMPRESSION_NONE:
        case TF_COMPRESSION_ZSTD:
        case TF_COMPRESSION_ZLIB:
            return 1;
        default:
            return 0;
    }
}

TFError TFHeader_read(const uint8_t *const bd,
                      const int bs,
                      TFHeader *const header,
                      uint8_t **const ep) {
    // header structure is a fixed 32 byte size according to schema
    // https://github.com/Cryptkeeper/fseq-file-format#header
    const int HEADER_SIZE = 32;

    if (bs < HEADER_SIZE) return TF_EINVALID_BUFFER_SIZE;

    if (bd[0] != 'P' || bd[1] != 'S' || bd[2] != 'E' || bd[3] != 'Q')
        return TF_EINVALID_MAGIC;

    __builtin_memcpy(&header->channelDataOffset, &bd[4],
                     sizeof(header->channelDataOffset));

    header->minorVersion = bd[6];
    header->majorVersion = bd[7];

    __builtin_memcpy(&header->variableDataOffset, &bd[8],
                     sizeof(header->variableDataOffset));
    __builtin_memcpy(&header->channelCount, &bd[10],
                     sizeof(header->channelCount));
    __builtin_memcpy(&header->frameCount, &bd[14], sizeof(header->frameCount));

    header->frameStepTimeMillis = bd[18];

    // upper 4 bits contain additional compression block count data that is ignored by tinyfseq
    // mask to lower 4 bits to filter only the compression type field
    const uint8_t compressionType = bd[20] & 0xF;

    if (!TFCompressionType_valid(compressionType))
        return TF_EINVALID_COMPRESSION_TYPE;

    header->compressionType       = (TFCompressionType) compressionType;
    header->compressionBlockCount = bd[21];

    header->channelRangeCount = bd[22];

    __builtin_memcpy(&header->sequenceUid, &bd[24], sizeof(header->sequenceUid));

    if (ep) *ep = (uint8_t *) bd + HEADER_SIZE;

    return TF_OK;
}

TFError TFCompressionBlock_read(const uint8_t *const bd,
                                const int bs,
                                TFCompressionBlock *const block,
                                uint8_t **const ep) {
    const int COMPRESSION_BLOCK_SIZE = 8;

    if (bs < COMPRESSION_BLOCK_SIZE) return TF_EINVALID_BUFFER_SIZE;

    __builtin_memcpy(&block->firstFrameId, &bd[0], sizeof(block->firstFrameId));
    __builtin_memcpy(&block->size, &bd[4], sizeof(block->size));

    if (ep) *ep = (uint8_t *) bd + COMPRESSION_BLOCK_SIZE;

    return TF_OK;
}

TFError TFVarHeader_read(const uint8_t *const bd,
                         const int bs,
                         TFVarHeader *const varHeader,
                         uint8_t *const vd,
                         const int vs,
                         uint8_t **const ep) {
    const int VAR_HEADER_SIZE = 4;

    // variable header requires 4 bytes and is NULL terminated
    // an empty variable should be at least 5 bytes
    if (bs <= VAR_HEADER_SIZE) return TF_EINVALID_BUFFER_SIZE;

    __builtin_memcpy(&varHeader->size, &bd[0], sizeof(varHeader->size));

    if (varHeader->size <= VAR_HEADER_SIZE) return TF_EINVALID_VAR_SIZE;

    __builtin_memcpy(varHeader->id, &bd[2], sizeof(varHeader->id));

    // only attempt to read variable value if a decoding buffer (`vd`) is provided
    // `.size` already includes the 4 bytes the header consumes
    if (vd) {
        // ensure the source buffer has enough data to read the variable value
        if (bs < varHeader->size) return TF_EINVALID_VAR_SIZE;

        // ensure the destination buffer is large enough to store the variable value
        const int valueSize = varHeader->size - VAR_HEADER_SIZE;
        if (vs < valueSize) return TF_EINVALID_BUFFER_SIZE;

        __builtin_memcpy(vd, &bd[VAR_HEADER_SIZE], valueSize);
    }

    if (ep) *ep = (uint8_t *) bd + varHeader->size;

    return TF_OK;
}

static uint32_t TFUint24_read(const uint8_t *const bd) {
    return bd[0] | bd[1] << 8 | bd[2] << 16;
}

TFError TFChannelRange_read(const uint8_t *const bd,
                            const int bs,
                            TFChannelRange *const channelRange,
                            uint8_t **const ep) {
    const int CHANNEL_RANGE_SIZE = 6;

    if (bs < CHANNEL_RANGE_SIZE) return TF_EINVALID_BUFFER_SIZE;

    channelRange->firstChannelNumber = TFUint24_read(&bd[0]);
    channelRange->channelCount       = TFUint24_read(&bd[3]);

    if (ep) *ep = (uint8_t *) bd + CHANNEL_RANGE_SIZE;

    return TF_OK;
}

#endif//TINYFSEQ_IMPLEMENTATION
