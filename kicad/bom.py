import sys
import xml.etree.ElementTree as ET
import csv
import re
from collections import defaultdict

mult = 1 if len(sys.argv) < 4 else int(sys.argv[3])

def seperate_ref(ref):
    match = re.match("^([A-Z]+)([0-9]+)$", ref)
    return (match[1], int(match[2]))

parts = defaultdict(lambda: [])  # (value, manfno) -> [ref]

xml = ET.parse(sys.argv[1])
for part in xml.findall("components/comp"):
    ref = part.get("ref")
    value = part.findtext("value")
    manfno = part.findtext("fields/field[@name='Manf#']")
    parts[(value, manfno)].append(seperate_ref(ref))

rows = []
for (value, manfno), refs in parts.items():
    refs.sort()
    rows.append([refs, value, len(refs) * mult, manfno])
rows.sort(key=lambda row: row[0])
for row in rows:
    row[0] = ",".join(a + str(b) for a, b in row[0])

with open(sys.argv[2], "w", newline="") as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(("References", "Value", "Count*" + str(mult), "Manf#"))
    for row in rows:
        writer.writerow(row)
