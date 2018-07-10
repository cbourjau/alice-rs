"""
Demonstration of how to analyze the json files produced in this exampl
"""

import json


if __name__ == "main":
    with open("events.json") as f:
        data_set = json.loads(f.read())
        for event in data_set:
            print event.keys()
            break
