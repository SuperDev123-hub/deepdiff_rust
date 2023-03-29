from deepdiff import DeepDiff

def diff(v1, v2, ignore_order):    
    ddiff = DeepDiff(v1, v2, ignore_order=ignore_order)    
    return ddiff.__str__()
