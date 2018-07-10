"""
Demonstration of how to analyze the json files produced in this exampl
"""

import msgpack

if __name__ == "__main__":
    with open("./src/events.bin") as f:
        unpacker = msgpack.Unpacker(f, raw=False)
        for event in unpacker:
            print event.keys()

