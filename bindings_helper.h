#include <linux/types.h>
#include <linux/stdarg.h>

/*
 * We define our own versions of these files so ensure these files don't get
 * included.
 */
#define __NVRM_NVTYPES_H__
#define _NVPORT_UTIL_H_
#define NVTYPES_INCLUDED

#define NV_ALIGN_BYTES(a) __attribute__ ((__aligned__(a)))
#define NV_DECLARE_ALIGNED(f,a) f __attribute__ ((__aligned__(a)))

/* We don't care about functions so this shouldn't make a difference */
#define NV_INLINE
#define NV_FORCEINLINE
#define NV_NOINLINE
#define NV_FORCERESULTCHECK
#define NV_ATTRIBUTE_UNUSED

#define NvU8 u8
#define NvS8 s8
#define NvV8 u8
#define NvU16 u16
#define NvS16 s16
#define NvV16 u16
#define NvU32 u32
#define NvS32 s32
#define NvV32 u32
#define NvU64 u64
#define NvS64 s64
#define NvV64 u64
#define NvP64 void*
#define NvBool u8
#define NvUPtr u64
#define NvSPtr s64
#define NvHandle u32
#define NvLength u64
#define RmPhysAddr u64
#define NV_STATUS u32

/* Boolean type */
#define NV_TRUE           (0 == 0)
#define NV_FALSE          (0 != 0)

#define NV_TRISTATE_FALSE           (0)
#define NV_TRISTATE_TRUE            (1)
#define NV_TRISTATE_INDETERMINATE   (2)

/* As defined by nvtypes.h */
#define NV_S8_MIN       (-128)
#define NV_S8_MAX       (+127)
#define NV_U8_MIN       (0U)
#define NV_U8_MAX       (+255U)
#define NV_S16_MIN      (-32768)
#define NV_S16_MAX      (+32767)
#define NV_U16_MIN      (0U)
#define NV_U16_MAX      (+65535U)
#define NV_S32_MIN      (-2147483647 - 1)
#define NV_S32_MAX      (+2147483647)
#define NV_U32_MIN      (0U)
#define NV_U32_MAX      (+4294967295U)
#define NV_S64_MIN      (-9223372036854775807LL - 1LL)
#define NV_S64_MAX      (+9223372036854775807LL)
#define NV_U64_MIN      (0ULL)
#define NV_U64_MAX      (+18446744073709551615ULL)

#define NVCPU_X86_64
