from array import array
import sys


def array_growth():
    return sys.getsizeof(array('Q', [2**40]) * 10240) - sys.getsizeof(array('Q'))


def check_array_storage(get_size):
    cases = {
        'I': [0, 1000, 2**31, 2**32 - 1],
        'Q': [0, 1000, 2**40, 2**63],
        'd': [0.0, 1.5, 1e100],
    }
    for typecode, values in cases.items():
        assert get_size(array(typecode)) == sys.getsizeof(array(typecode))
        data = array(typecode, [values[0]]) * 10240
        size = sys.getsizeof(data)
        address = data.buffer_info()[0]
        for value in values:
            for i in range(len(data)):
                data[i] = value
            assert data.buffer_info()[0] == address
            assert sys.getsizeof(data) == size
            assert get_size(data) == size, (typecode, value, get_size(data), size)


def check_array_references(get_size):
    first = array('I', [1000, 2000])
    second = array('I', first)
    values = [first, first, second]
    values.append(values)
    expected = sys.getsizeof(values) + sys.getsizeof(first) + sys.getsizeof(second)
    assert get_size(values) == expected

    # Ordinary containers still need their referenced objects counted once.
    number = int('10000000000')
    values = [number, number]
    assert get_size(values) == sys.getsizeof(values) + sys.getsizeof(number)


class DictArray(array):
    def __iter__(self):
        raise AssertionError('measuring an array must not iterate over its values')


class SlotsArray(array):
    __slots__ = ('payload',)


def check_array_subclasses(get_size):
    data = DictArray('I', [1000, 2000])
    data.payload = bytearray(1024)
    data.alias = data.payload
    expected = (sys.getsizeof(data) + sys.getsizeof(vars(data))
                + sum(sys.getsizeof(key) for key in vars(data))
                + sys.getsizeof(data.payload))
    assert get_size(data) == expected

    data = SlotsArray('Q', [2**63])
    data.payload = bytearray(1024)
    assert get_size(data) == sys.getsizeof(data) + sys.getsizeof(data.payload)
