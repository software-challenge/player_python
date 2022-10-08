# Test _xflux.py

import unittest
from python.socha.api.networking._xflux import _XFluxClient


class XFluxTest(unittest.TestCase):
    def testXFluxInit(self):
        xflux = _XFluxClient("localhost", 1234)
        self.assertEqual(xflux.host, "localhost")
        self.assertEqual(xflux.port, 1234)
