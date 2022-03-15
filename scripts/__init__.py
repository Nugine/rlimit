from pprint import pprint
import sys


def epprint(*args, **kwargs):
    pprint(*args, **kwargs, stream=sys.stderr)
