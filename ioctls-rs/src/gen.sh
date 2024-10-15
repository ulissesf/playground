bindgen --verbose --no-layout-tests --no-prepend-enum-name --no-doc-comments xe_drm.h > xe_drm-body.rs
cat allow-hdr xe_drm-body.rs > xe_drm-new.rs
rm -f xe_drm-body.rs
