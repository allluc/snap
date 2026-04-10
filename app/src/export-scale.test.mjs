import test from "node:test";
import assert from "node:assert/strict";
import { mapCropToNative } from "./export-scale.mjs";

test("mapCropToNative scales crop from preview to native capture pixels", () => {
  const crop = { x: 100, y: 50, w: 400, h: 200 };
  const mapped = mapCropToNative(crop, {
    previewWidth: 1920,
    previewHeight: 1080,
    nativeWidth: 3840,
    nativeHeight: 2160,
  });

  assert.deepEqual(mapped, { x: 200, y: 100, w: 800, h: 400 });
});

test("mapCropToNative clamps crop to native bounds", () => {
  const crop = { x: -10, y: 1000, w: 2000, h: 300 };
  const mapped = mapCropToNative(crop, {
    previewWidth: 1000,
    previewHeight: 1000,
    nativeWidth: 2000,
    nativeHeight: 2000,
  });

  assert.deepEqual(mapped, { x: 0, y: 2000, w: 2000, h: 0 });
});
