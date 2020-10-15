def slices(series, length):
    if not series or length < 1 or len(series) < length:
        raise ValueError("Bad arguments...")
    return [ series[i:i+length] for i in range(len(series)-length+1) ]
