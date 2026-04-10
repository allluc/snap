import base64
import importlib
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path


class InlineImageDataTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.home = Path(self.tmp.name)
        inbox = self.home / ".snap" / "inbox"
        inbox.mkdir(parents=True, exist_ok=True)

        self.png_bytes = b"\x89PNG\r\n\x1a\nnot-a-real-png"
        self.base = inbox / "snap-20990101-000000-000"
        self.base.with_suffix(".png").write_bytes(self.png_bytes)
        self.base.with_suffix(".json").write_text(json.dumps({"annotations": []}))

        self.old_home = os.environ.get("HOME")
        os.environ["HOME"] = str(self.home)

        if "server" in sys.modules:
            del sys.modules["server"]
        self.server = importlib.import_module("server")

    def tearDown(self):
        if self.old_home is None:
            os.environ.pop("HOME", None)
        else:
            os.environ["HOME"] = self.old_home

    def test_get_latest_annotation_embeds_image_data_when_requested(self):
        result = self.server.get_latest_annotation(include_image_data=True)

        self.assertIn("image_base64", result)
        self.assertEqual(result["image_media_type"], "image/png")
        self.assertEqual(
            result["image_base64"], base64.b64encode(self.png_bytes).decode("ascii")
        )

    def test_get_latest_annotation_skips_inline_when_disabled(self):
        result = self.server.get_latest_annotation(include_image_data=False)
        self.assertNotIn("image_base64", result)

    def test_get_latest_annotation_warns_when_image_too_large(self):
        result = self.server.get_latest_annotation(
            include_image_data=True, max_image_bytes=4
        )
        self.assertIn("image_inline_warning", result)
        self.assertNotIn("image_base64", result)


if __name__ == "__main__":
    unittest.main()
