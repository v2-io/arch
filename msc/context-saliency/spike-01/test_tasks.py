"""CPU tests of the shipped Plan-NIAH generator. No GPU, no reimplementation."""
import unittest

from tasks import make_task, occlude, room_order_for


class TestPlacement(unittest.TestCase):
    def test_room_order_for(self):
        self.assertEqual(room_order_for(4, "chrono"), [1, 2, 3, 4])
        self.assertEqual(room_order_for(4, "reversed"), [4, 3, 2, 1])
        with self.assertRaises(ValueError):
            room_order_for(4, "shuffle")

    def test_two_placements_differ_and_keep_matched_codes(self):
        seed, n = 7, 4
        sc = make_task("screened", n_rooms=n, seed=seed, placement="chrono")
        sr = make_task("screened", n_rooms=n, seed=seed, placement="reversed")
        dc = make_task("delayed_reuse", n_rooms=n, seed=seed, placement="chrono")
        dr = make_task("delayed_reuse", n_rooms=n, seed=seed, placement="reversed")
        # same seed ⇒ same codes across variant and placement
        self.assertEqual(sc.codes, sr.codes)
        self.assertEqual(sc.codes, dc.codes)
        self.assertEqual(sc.codes, dr.codes)
        self.assertEqual(sc.reuse_room_draw, dc.reuse_room_draw)
        # presentation order actually moves rooms in the prompt
        self.assertEqual(sc.room_order, [1, 2, 3, 4])
        self.assertEqual(sr.room_order, [4, 3, 2, 1])
        self.assertNotEqual(sc.prompt, sr.prompt)
        self.assertLess(sc.prompt.index("Room 1 is"), sc.prompt.index("Room 4 is"))
        self.assertLess(sr.prompt.index("Room 4 is"), sr.prompt.index("Room 1 is"))
        # matched pair: only the question differs (transcript prefix identical)
        self.assertEqual(sc.prompt.split("All rooms")[0], dc.prompt.split("All rooms")[0])
        self.assertEqual(sr.prompt.split("All rooms")[0], dr.prompt.split("All rooms")[0])
        self.assertNotEqual(sc.prompt, dc.prompt)
        self.assertIn("room-number order", sc.prompt)
        self.assertIn("regardless of the order", sr.prompt)

    def test_formulaic_surface_equalizes_novelty_keeps_roles(self):
        n = make_task("delayed_reuse", n_rooms=4, seed=7, surface="narrative")
        f = make_task("delayed_reuse", n_rooms=4, seed=7, surface="formulaic")
        self.assertEqual(len(n.segments), len(f.segments))
        self.assertEqual({s.role for s in f.segments}, {s.role for s in n.segments})
        self.assertEqual(sum(s.reused_detail for s in f.segments), 1)
        body = f.prompt.split("All rooms")[0]
        self.assertIn("ROOM-1:", body)
        self.assertIn("COUNT-", body)
        self.assertIn("AUX-1-1: 0", body)
        self.assertNotIn("tapestries", body)
        self.assertNotIn("tapestries", f.prompt.split("All rooms")[0])
        # matched pair: only the question differs
        fs = make_task("screened", n_rooms=4, seed=7, surface="formulaic")
        self.assertEqual(fs.prompt.split("All rooms")[0], f.prompt.split("All rooms")[0])
        self.assertIn("COUNT-", f.prompt.split("All rooms")[1])

    def test_roles_and_occlusion_length(self):
        t = make_task("delayed_reuse", n_rooms=4, seed=7, placement="reversed")
        roles = {}
        for s in t.segments:
            roles[s.role] = roles.get(s.role, 0) + 1
        self.assertEqual(roles["header"], 4)
        self.assertEqual(roles["count"], 4)
        self.assertEqual(roles["terminal"], 4)
        self.assertEqual(sum(s.reused_detail for s in t.segments), 1)
        for kind in ("header", "interior", "count", "narrative", "terminal", "reuse_line"):
            p = occlude(t, (kind, t.reuse_room_draw if kind == "reuse_line" else 2))
            self.assertEqual(len(p), len(t.prompt), kind)


if __name__ == "__main__":
    unittest.main()
