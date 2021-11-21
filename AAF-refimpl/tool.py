import re

comments = re.compile("^\W*//.*$")
cmacros = re.compile("^#.*$")
with open("AAFMetaDictionary.h") as f:
    
    comments_removed = ""

    for line in f.readlines():
        if comments.match(line):
            pass
        elif cmacros.match(line):
            pass
        elif len(line) == 0:
            pass
        else:
            comments_removed = comments_removed + line.strip()

class_def = re.compile(
        r"AAF_CLASS\(\s*([A-Za-z0-9]+)\s*," + 
        r"\s*(.*?)\s*," +
        r"\s*([A-z0-9]+)\s*,\s*(true|false)\s*\)" +
        r"(.*?)" +
        r"AAF_CLASS_END\(")

class_property_re = re.compile(
        r"\s*AAF_PROPERTY\((.*?),(AAF_LITERAL_AUID\(.*?\)),(.*?),(.*?),\s*(true|false)\s*,\s*(true|false)\s*"
        )


klasses = []

for m in class_def.findall(comments_removed):
    this_klass = {}
    #print("Class: ", m[0])
    #print("id: ", m[1])
    #print("superclass: ", m[2])
    #print("is concrete?:", m[3])
    #print("def:", m[4])
    
    this_klass['name'] = m[0]
    this_klass['id'] = m[1]
    this_klass['superclass'] = m[2]
    this_klass['is_concrete'] = m[3]

    this_props = []
    for p in class_property_re.findall(m[4]):
        this_prop = {}
        
        this_prop['name'] = p[0]
        this_prop['auid'] = p[1]
        this_prop['pid'] = p[2]
        this_prop['type'] = p[3]
        this_prop['mandatory'] = p[4]
        this_prop['is_unique'] = p[5]

        this_props.append(this_prop)
    
    this_klass['properties'] = this_props
    klasses.append(this_klass)

import pprint as pp
pp.pprint(klasses)
