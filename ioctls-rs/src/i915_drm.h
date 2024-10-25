#include <stdint.h>
/**
 * struct drm_i915_query_item - An individual query for the kernel to process.
 *
 * The behaviour is determined by the @query_id. Note that exactly what
 * @data_ptr is also depends on the specific @query_id.
 */
struct drm_i915_query_item {
	/**
	 * @query_id:
	 *
	 * The id for this query.  Currently accepted query IDs are:
	 *  - %DRM_I915_QUERY_TOPOLOGY_INFO (see struct drm_i915_query_topology_info)
	 *  - %DRM_I915_QUERY_ENGINE_INFO (see struct drm_i915_engine_info)
	 *  - %DRM_I915_QUERY_PERF_CONFIG (see struct drm_i915_query_perf_config)
	 *  - %DRM_I915_QUERY_MEMORY_REGIONS (see struct drm_i915_query_memory_regions)
	 *  - %DRM_I915_QUERY_HWCONFIG_BLOB (see `GuC HWCONFIG blob uAPI`)
	 *  - %DRM_I915_QUERY_GEOMETRY_SUBSLICES (see struct drm_i915_query_topology_info)
	 *  - %DRM_I915_QUERY_GUC_SUBMISSION_VERSION (see struct drm_i915_query_guc_submission_version)
	 */
	uint64_t query_id;
#define DRM_I915_QUERY_TOPOLOGY_INFO		1
#define DRM_I915_QUERY_ENGINE_INFO		2
#define DRM_I915_QUERY_PERF_CONFIG		3
#define DRM_I915_QUERY_MEMORY_REGIONS		4
#define DRM_I915_QUERY_HWCONFIG_BLOB		5
#define DRM_I915_QUERY_GEOMETRY_SUBSLICES	6
#define DRM_I915_QUERY_GUC_SUBMISSION_VERSION	7
/* Must be kept compact -- no holes and well documented */

	/**
	 * @length:
	 *
	 * When set to zero by userspace, this is filled with the size of the
	 * data to be written at the @data_ptr pointer. The kernel sets this
	 * value to a negative value to signal an error on a particular query
	 * item.
	 */
	int32_t length;

	/**
	 * @flags:
	 *
	 * When &query_id == %DRM_I915_QUERY_TOPOLOGY_INFO, must be 0.
	 *
	 * When &query_id == %DRM_I915_QUERY_PERF_CONFIG, must be one of the
	 * following:
	 *
	 *	- %DRM_I915_QUERY_PERF_CONFIG_LIST
	 *      - %DRM_I915_QUERY_PERF_CONFIG_DATA_FOR_UUID
	 *      - %DRM_I915_QUERY_PERF_CONFIG_FOR_UUID
	 *
	 * When &query_id == %DRM_I915_QUERY_GEOMETRY_SUBSLICES must contain
	 * a struct i915_engine_class_instance that references a render engine.
	 */
	uint32_t flags;
#define DRM_I915_QUERY_PERF_CONFIG_LIST          1
#define DRM_I915_QUERY_PERF_CONFIG_DATA_FOR_UUID 2
#define DRM_I915_QUERY_PERF_CONFIG_DATA_FOR_ID   3

	/**
	 * @data_ptr:
	 *
	 * Data will be written at the location pointed by @data_ptr when the
	 * value of @length matches the length of the data to be written by the
	 * kernel.
	 */
	uint64_t data_ptr;
};

/**
 * struct drm_i915_query - Supply an array of struct drm_i915_query_item for the
 * kernel to fill out.
 *
 * Note that this is generally a two step process for each struct
 * drm_i915_query_item in the array:
 *
 * 1. Call the DRM_IOCTL_I915_QUERY, giving it our array of struct
 *    drm_i915_query_item, with &drm_i915_query_item.length set to zero. The
 *    kernel will then fill in the size, in bytes, which tells userspace how
 *    memory it needs to allocate for the blob(say for an array of properties). 
 *
 * 2. Next we call DRM_IOCTL_I915_QUERY again, this time with the
 *    &drm_i915_query_item.data_ptr equal to our newly allocated blob. Note that *    the &drm_i915_query_item.length should still be the same as what the
 *    kernel previously set. At this point the kernel can fill in the blob.
 *
 * Note that for some query items it can make sense for userspace to just pass
 * in a buffer/blob equal to or larger than the required size. In this case only * a single ioctl call is needed. For some smaller query items this can work
 * quite well.
 *
 */
struct drm_i915_query {
       /** @num_items: The number of elements in the @items_ptr array */
       uint32_t num_items;

       /**
        * @flags: Unused for now. Must be cleared to zero.
        */
       uint32_t flags;

       /**
        * @items_ptr:
        *
        * Pointer to an array of struct drm_i915_query_item. The number of
        * array elements is @num_items.
        */
       uint64_t items_ptr;
};

/**
 * enum drm_i915_gem_memory_class - Supported memory classes
 */
enum drm_i915_gem_memory_class {
	/** @I915_MEMORY_CLASS_SYSTEM: System memory */
	I915_MEMORY_CLASS_SYSTEM = 0,
	/** @I915_MEMORY_CLASS_DEVICE: Device local-memory */
	I915_MEMORY_CLASS_DEVICE,
};

/**
 * struct drm_i915_gem_memory_class_instance - Identify particular memory region
 */
struct drm_i915_gem_memory_class_instance {
	/** @memory_class: See enum drm_i915_gem_memory_class */
	uint16_t memory_class;

	/** @memory_instance: Which instance */
	uint16_t memory_instance;
};

/**
 * struct drm_i915_memory_region_info - Describes one region as known to the
 * driver.
 *
 * Note this is using both struct drm_i915_query_item and struct drm_i915_query.
 * For this new query we are adding the new query id DRM_I915_QUERY_MEMORY_REGIONS
 * at &drm_i915_query_item.query_id.
 */
struct drm_i915_memory_region_info {
	/** @region: The class:instance pair encoding */
	struct drm_i915_gem_memory_class_instance region;

	/** @rsvd0: MBZ */
	uint32_t rsvd0;

	/**
	 * @probed_size: Memory probed by the driver
	 *
	 * Note that it should not be possible to ever encounter a zero value
	 * here, also note that no current region type will ever return -1 here.
	 * Although for future region types, this might be a possibility. The
	 * same applies to the other size fields.
	 */
	uint64_t probed_size;

	/**
	 * @unallocated_size: Estimate of memory remaining
	 *
	 * Requires CAP_PERFMON or CAP_SYS_ADMIN to get reliable accounting.
	 * Without this (or if this is an older kernel) the value here will
	 * always equal the @probed_size. Note this is only currently tracked
	 * for I915_MEMORY_CLASS_DEVICE regions (for other types the value here
	 * will always equal the @probed_size).
	 */
	uint64_t unallocated_size;

	union {
		/** @rsvd1: MBZ */
		uint64_t rsvd1[8];
		struct {
			/**
			 * @probed_cpu_visible_size: Memory probed by the driver
			 * that is CPU accessible.
			 *
			 * This will be always be <= @probed_size, and the
			 * remainder (if there is any) will not be CPU
			 * accessible.
			 *
			 * On systems without small BAR, the @probed_size will
			 * always equal the @probed_cpu_visible_size, since all
			 * of it will be CPU accessible.
			 *
			 * Note this is only tracked for
			 * I915_MEMORY_CLASS_DEVICE regions (for other types the
			 * value here will always equal the @probed_size).
			 *
			 * Note that if the value returned here is zero, then
			 * this must be an old kernel which lacks the relevant
			 * small-bar uAPI support (including
			 * I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS), but on
			 * such systems we should never actually end up with a
			 * small BAR configuration, assuming we are able to load
			 * the kernel module. Hence it should be safe to treat
			 * this the same as when @probed_cpu_visible_size ==
			 * @probed_size.
			 */
			uint64_t probed_cpu_visible_size;

			/**
			 * @unallocated_cpu_visible_size: Estimate of CPU
			 * visible memory remaining.
			 *
			 * Note this is only tracked for
			 * I915_MEMORY_CLASS_DEVICE regions (for other types the
			 * value here will always equal the
			 * @probed_cpu_visible_size).
			 *
			 * Requires CAP_PERFMON or CAP_SYS_ADMIN to get reliable
			 * accounting.  Without this the value here will always
			 * equal the @probed_cpu_visible_size. Note this is only
			 * currently tracked for I915_MEMORY_CLASS_DEVICE
			 * regions (for other types the value here will also
			 * always equal the @probed_cpu_visible_size).
			 *
			 * If this is an older kernel the value here will be
			 * zero, see also @probed_cpu_visible_size.
			 */
			uint64_t unallocated_cpu_visible_size;
		};
	};
};

/**
 * struct drm_i915_query_memory_regions
 *
 * The region info query enumerates all regions known to the driver by filling
 * in an array of struct drm_i915_memory_region_info structures.
 *
 * Example for getting the list of supported regions:
 *
 * .. code-block:: C
 *
 *	struct drm_i915_query_memory_regions *info;
 *	struct drm_i915_query_item item = {
 *		.query_id = DRM_I915_QUERY_MEMORY_REGIONS;
 *	};
 *	struct drm_i915_query query = {
 *		.num_items = 1,
 *		.items_ptr = (uintptr_t)&item,
 *	};
 *	int err, i;
 *
 *	// First query the size of the blob we need, this needs to be large
 *	// enough to hold our array of regions. The kernel will fill out the
 *	// item.length for us, which is the number of bytes we need.
 *	err = ioctl(fd, DRM_IOCTL_I915_QUERY, &query);
 *	if (err) ...
 *
 *	info = calloc(1, item.length);
 *	// Now that we allocated the required number of bytes, we call the ioctl
 *	// again, this time with the data_ptr pointing to our newly allocated
 *	// blob, which the kernel can then populate with the all the region info.
 *	item.data_ptr = (uintptr_t)&info,
 *
 *	err = ioctl(fd, DRM_IOCTL_I915_QUERY, &query);
 *	if (err) ...
 *
 *	// We can now access each region in the array
 *	for (i = 0; i < info->num_regions; i++) {
 *		struct drm_i915_memory_region_info mr = info->regions[i];
 *		uint16_t class = mr.region.class;
 *		uint16_t instance = mr.region.instance;
 *
 *		....
 *	}
 *
 *	free(info);
 */
struct drm_i915_query_memory_regions {
	/** @num_regions: Number of supported regions */
	uint32_t num_regions;

	/** @rsvd: MBZ */
	uint32_t rsvd[3];

	/** @regions: Info about each supported region */
	struct drm_i915_memory_region_info regions[];
};

#define DRM_IOCTL_I915_QUERY    3222299769
