#include <stdint.h>

/**
 * struct drm_xe_device_query - Input of &DRM_IOCTL_XE_DEVICE_QUERY - main
 * structure to query device information
 *
 * The user selects the type of data to query among DRM_XE_DEVICE_QUERY_*
 * and sets the value in the query member. This determines the type of
 * the structure provided by the driver in data, among struct drm_xe_query_*.
 *
 * The @query can be:
 *  - %DRM_XE_DEVICE_QUERY_ENGINES
 *  - %DRM_XE_DEVICE_QUERY_MEM_REGIONS
 *  - %DRM_XE_DEVICE_QUERY_CONFIG
 *  - %DRM_XE_DEVICE_QUERY_GT_LIST
 *  - %DRM_XE_DEVICE_QUERY_HWCONFIG - Query type to retrieve the hardware
 *    configuration of the device such as information on slices, memory,
 *    caches, and so on. It is provided as a table of key / value
 *    attributes.
 *  - %DRM_XE_DEVICE_QUERY_GT_TOPOLOGY
 *  - %DRM_XE_DEVICE_QUERY_ENGINE_CYCLES
 *
 * If size is set to 0, the driver fills it with the required size for
 * the requested type of data to query. If size is equal to the required
 * size, the queried information is copied into data. If size is set to
 * a value different from 0 and different from the required size, the
 * IOCTL call returns -EINVAL.
 *
 * For example the following code snippet allows retrieving and printing
 * information about the device engines with DRM_XE_DEVICE_QUERY_ENGINES:
 *
 * .. code-block:: C
 *
 *     struct drm_xe_query_engines *engines;
 *     struct drm_xe_device_query query = {
 *         .extensions = 0,
 *         .query = DRM_XE_DEVICE_QUERY_ENGINES,
 *         .size = 0,
 *         .data = 0,
 *     };
 *     ioctl(fd, DRM_IOCTL_XE_DEVICE_QUERY, &query);
 *     engines = malloc(query.size);
 *     query.data = (uintptr_t)engines;
 *     ioctl(fd, DRM_IOCTL_XE_DEVICE_QUERY, &query);
 *     for (int i = 0; i < engines->num_engines; i++) {
 *         printf("Engine %d: %s\n", i,
 *             engines->engines[i].instance.engine_class ==
 *                 DRM_XE_ENGINE_CLASS_RENDER ? "RENDER":
 *             engines->engines[i].instance.engine_class ==
 *                 DRM_XE_ENGINE_CLASS_COPY ? "COPY":
 *             engines->engines[i].instance.engine_class ==
 *                 DRM_XE_ENGINE_CLASS_VIDEO_DECODE ? "VIDEO_DECODE":
 *             engines->engines[i].instance.engine_class ==
 *                 DRM_XE_ENGINE_CLASS_VIDEO_ENHANCE ? "VIDEO_ENHANCE":
 *             engines->engines[i].instance.engine_class ==
 *                 DRM_XE_ENGINE_CLASS_COMPUTE ? "COMPUTE":
 *             "UNKNOWN");
 *     }
 *     free(engines);
 */
struct drm_xe_device_query {
	/** @extensions: Pointer to the first extension struct, if any */
	uint64_t extensions;

#define DRM_XE_DEVICE_QUERY_ENGINES		0
#define DRM_XE_DEVICE_QUERY_MEM_REGIONS		1
#define DRM_XE_DEVICE_QUERY_CONFIG		2
#define DRM_XE_DEVICE_QUERY_GT_LIST		3
#define DRM_XE_DEVICE_QUERY_HWCONFIG		4
#define DRM_XE_DEVICE_QUERY_GT_TOPOLOGY		5
#define DRM_XE_DEVICE_QUERY_ENGINE_CYCLES	6
#define DRM_XE_DEVICE_QUERY_UC_FW_VERSION	7
#define DRM_XE_DEVICE_QUERY_OA_UNITS		8
	/** @query: The type of data to query */
	uint32_t query;

	/** @size: Size of the queried data */
	uint32_t size;

	/** @data: Queried data is placed here */
	uint64_t data;

	/** @reserved: Reserved */
	uint64_t reserved[2];
};

enum drm_xe_memory_class {
	/** @DRM_XE_MEM_REGION_CLASS_SYSMEM: Represents system memory. */
	DRM_XE_MEM_REGION_CLASS_SYSMEM = 0,
	/**
	 * @DRM_XE_MEM_REGION_CLASS_VRAM: On discrete platforms, this
	 * represents the memory that is local to the device, which we
	 * call VRAM. Not valid on integrated platforms.
	 */
	DRM_XE_MEM_REGION_CLASS_VRAM
};

/**
 * struct drm_xe_mem_region - Describes some region as known to
 * the driver.
 */
struct drm_xe_mem_region {
	/**
	 * @mem_class: The memory class describing this region.
	 *
	 * See enum drm_xe_memory_class for supported values.
	 */
	uint16_t mem_class;
	/**
	 * @instance: The unique ID for this region, which serves as the
	 * index in the placement bitmask used as argument for
	 * &DRM_IOCTL_XE_GEM_CREATE
	 */
	uint16_t instance;
	/**
	 * @min_page_size: Min page-size in bytes for this region.
	 *
	 * When the kernel allocates memory for this region, the
	 * underlying pages will be at least @min_page_size in size.
	 * Buffer objects with an allowable placement in this region must be
	 * created with a size aligned to this value.
	 * GPU virtual address mappings of (parts of) buffer objects that
	 * may be placed in this region must also have their GPU virtual
	 * address and range aligned to this value.
	 * Affected IOCTLS will return %-EINVAL if alignment restrictions are
	 * not met.
	 */
	uint32_t min_page_size;
	/**
	 * @total_size: The usable size in bytes for this region.
	 */
	uint64_t total_size;
	/**
	 * @used: Estimate of the memory used in bytes for this region.
	 *
	 * Requires CAP_PERFMON or CAP_SYS_ADMIN to get reliable
	 * accounting.  Without this the value here will always equal
	 * zero.
	 */
	uint64_t used;
	/**
	 * @cpu_visible_size: How much of this region can be CPU
	 * accessed, in bytes.
	 *
	 * This will always be <= @total_size, and the remainder (if
	 * any) will not be CPU accessible. If the CPU accessible part
	 * is smaller than @total_size then this is referred to as a
	 * small BAR system.
	 *
	 * On systems without small BAR (full BAR), the probed_size will
	 * always equal the @total_size, since all of it will be CPU
	 * accessible.
	 *
	 * Note this is only tracked for DRM_XE_MEM_REGION_CLASS_VRAM
	 * regions (for other types the value here will always equal
	 * zero).
	 */
	uint64_t cpu_visible_size;
	/**
	 * @cpu_visible_used: Estimate of CPU visible memory used, in
	 * bytes.
	 *
	 * Requires CAP_PERFMON or CAP_SYS_ADMIN to get reliable
	 * accounting. Without this the value here will always equal
	 * zero.  Note this is only currently tracked for
	 * DRM_XE_MEM_REGION_CLASS_VRAM regions (for other types the value
	 * here will always be zero).
	 */
	uint64_t cpu_visible_used;
	/** @reserved: Reserved */
	uint64_t reserved[6];
};

/**
 * struct drm_xe_query_mem_regions - describe memory regions
 *
 * If a query is made with a struct drm_xe_device_query where .query
 * is equal to DRM_XE_DEVICE_QUERY_MEM_REGIONS, then the reply uses
 * struct drm_xe_query_mem_regions in .data.
 */
struct drm_xe_query_mem_regions {
	/** @num_mem_regions: number of memory regions returned in @mem_regions */
	uint32_t num_mem_regions;
	/** @pad: MBZ */
	uint32_t pad;
	/** @mem_regions: The returned memory regions for this device */
	struct drm_xe_mem_region mem_regions[];
};

/**
 * struct drm_xe_query_config - describe the device configuration
 *
 * If a query is made with a struct drm_xe_device_query where .query
 * is equal to DRM_XE_DEVICE_QUERY_CONFIG, then the reply uses
 * struct drm_xe_query_config in .data.
 *
 * The index in @info can be:
 *  - %DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID - Device ID (lower 16 bits)
 *    and the device revision (next 8 bits)
 *  - %DRM_XE_QUERY_CONFIG_FLAGS - Flags describing the device
 *    configuration, see list below
 *
 *    - %DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM - Flag is set if the device
 *      has usable VRAM
 *  - %DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT - Minimal memory alignment
 *    required by this device, typically SZ_4K or SZ_64K
 *  - %DRM_XE_QUERY_CONFIG_VA_BITS - Maximum bits of a virtual address
 *  - %DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY - Value of the highest
 *    available exec queue priority
 */
struct drm_xe_query_config {
	/** @num_params: number of parameters returned in info */
	uint32_t num_params;

	/** @pad: MBZ */
	uint32_t pad;

#define DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID	0
#define DRM_XE_QUERY_CONFIG_FLAGS			1
	#define DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM	(1 << 0)
#define DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT		2
#define DRM_XE_QUERY_CONFIG_VA_BITS			3
#define DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY	4
	/** @info: array of elements containing the config info */
	uint64_t info[];
};

#define DRM_IOCTL_XE_DEVICE_QUERY               3223872576
