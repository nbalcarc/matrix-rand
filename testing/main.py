import numpy as np
import math
from functools import reduce

a = np.array([
    [0,  1,  2,  3,  4,  5,  6 ],
    [7,  8,  9,  10, 11, 12, 13],
    [14, 15, 16, 17, 18, 19, 20],
])

b = np.array([
    [0, 1, 2, 3],
    [4, 5, 6, 7],
])


def get_coord(i: int, n: int) -> float:
    return i / n

def get_index_floor(c: float, n: int) -> int:
    return math.floor(n * c)

def get_index_ceil(c: float, n: int) -> int:
    return math.ceil(n * c) - 1

def get_index_round(c: float, n: int) -> int:
    return round(n * c)

#b_cell = (2, 1) #cell we've chosen
b_cell = (1, 2) #cell we've chosen
#bx_start = b_cell[0] / b.shape[0] #x
#bx_end = (b_cell[0] + 1) / b.shape[0]
#by_start = b_cell[1] / b.shape[1] #y
#by_end = (b_cell[1] + 1) / b.shape[1]
# coords
bx_start = get_coord(b_cell[0], b.shape[0]) #x
bx_end = get_coord(b_cell[0] + 1, b.shape[0])
by_start = get_coord(b_cell[1], b.shape[1]) #y
by_end = get_coord(b_cell[1] + 1, b.shape[1])

print(bx_start)
print(bx_end)
print(by_start)
print(by_end)
print("-----")

# indices
ax_start = get_index_floor(bx_start, a.shape[0]) - 1
ax_end = get_index_floor(bx_end, a.shape[0])
ay_start = get_index_floor(by_start, a.shape[1]) - 1
ay_end = get_index_floor(by_end, a.shape[1])

print(f"{ax_start}->{ax_end}, {ay_start}->{ay_end}")

print(f"{bx_start}")
print(f"{bx_end}")
print()

print(f"{get_coord(ax_start, a.shape[0])}")
print(f"{get_coord(ax_end, a.shape[0])}")
print()


print(f"{by_start}")
print()

print(f"{get_coord(ay_start, a.shape[1])} --")
print(f"{get_coord(ay_start+1, a.shape[1])} --")
print(f"{get_coord(ay_start+1, a.shape[1])}")
print(f"{get_coord(ay_start+2, a.shape[1])}")
print()

print(f"{by_end}")
print()

print(f"{get_coord(ay_end-1, a.shape[1])}")
print(f"{get_coord(ay_end, a.shape[1])}")
print(f"{get_coord(ay_end, a.shape[1])} --")
print(f"{get_coord(ay_end+1, a.shape[1])} --")
print()


def get_matched(chosen: tuple[int, int], B_sizes: tuple[int, int], A_sizes: tuple[int, int]) -> tuple[int, int, int, int]:
    """Get the range of cells when overlaying a matrix on another"""
    # get edge coords of cell in B
    bx_start = get_coord(chosen[0], B_sizes[0]) #x
    bx_end = get_coord(chosen[0] + 1, B_sizes[0])
    by_start = get_coord(chosen[1], B_sizes[1]) #y
    by_end = get_coord(chosen[1] + 1, B_sizes[1])

    # get the closest cell in A
    ax_start = get_index_floor(bx_start, A_sizes[0])
    ax_end = get_index_floor(bx_end, A_sizes[0])
    ay_start = get_index_floor(by_start, A_sizes[1])
    ay_end = get_index_floor(by_end, A_sizes[1])

    return (ax_start, ax_end, ay_start, ay_end)

print(b_cell)
print(f"{b[b_cell]}")
results = get_matched(b_cell, b.shape, a.shape)
print(f"{results} <-")
print(f"{get_coord(results[0], a.shape[0])}, {get_coord(results[0]+1, a.shape[0])}")
print(f"{get_coord(results[1], a.shape[0])}, {get_coord(results[1]+1, a.shape[0])}")
#print(f"{a[results[0]]}, {a[results[1]]}, {a[results[2]]}, {a[results[3]]}")
#print(f"{a[results[0], results[2]]}, {a[results[1], results[3]]}")

print()
thing = 0.9
print(f"{get_index_round(thing, 2)}")
print(f"{get_index_floor(thing, 2)}")

x_fractions = list(map(lambda x: (x, 1.0), range(results[0], results[1]+1)))
y_fractions = list(map(lambda y: (y, 1.0), range(results[2], results[3]+1)))
print(x_fractions)
print(y_fractions)
print()

x_fractions[0] = (x_fractions[0][0], (1.0 - get_coord(results[0], a.shape[0])) / get_coord(results[0] + 1, a.shape[0]))

print(x_fractions)
print(y_fractions)

print("\n=================================\n")

def calc_progress_end(x: float, y: float, z: float) -> float:
    """Calculates progress from the left side"""
    #print(f"end {x} {y} {z}: {(z - x) / (y - x)}")
    return (z - x) / (y - x)

def calc_progress_start(x: float, y: float, z: float) -> float:
    """Calculates progress from the right side"""
    #print(f"start {x} {y} {z}: {1 - (z - x) / (y - x)}")
    return 1 - (z - x) / (y - x)

#results = get_matched(b_cell, b.shape, a.shape)
chosen = b_cell
b_sizes = b.shape
a_sizes = a.shape

edges = (
    get_coord(chosen[0], b_sizes[0]), #x
    get_coord(chosen[0] + 1, b_sizes[0]),
    get_coord(chosen[1], b_sizes[1]), #y
    get_coord(chosen[1] + 1, b_sizes[1]),
)

a_cells = (
    get_index_floor(bx_start, a_sizes[0]),
    get_index_ceil(bx_end, a_sizes[0]),
    get_index_floor(by_start, a_sizes[1]),
    get_index_ceil(by_end, a_sizes[1]),
)

print(a_cells)

x_fractions = list(map(lambda x: [x, 1.0], range(a_cells[0], a_cells[1]+1)))
y_fractions = list(map(lambda y: [y, 1.0], range(a_cells[2], a_cells[3]+1)))

#x_fractions[0][1] = (edges[0] - get_coord(a_cells[0], a_sizes[0])) / get_coord(a_cells[0] + 1, a_sizes[0])
#print(f"{get_coord(a_cells[0], a_sizes[0])}, {get_coord(a_cells[0]+1, a_sizes[0])}, {edges[0]}")
x_fractions[0][1] = calc_progress_start(get_coord(a_cells[0], a_sizes[0]), get_coord(a_cells[0]+1, a_sizes[0]), edges[0])
x_fractions[-1][1] = calc_progress_end(get_coord(a_cells[1], a_sizes[0]), get_coord(a_cells[1]+1, a_sizes[0]), edges[1])
y_fractions[0][1] = calc_progress_start(get_coord(a_cells[2], a_sizes[1]), get_coord(a_cells[2]+1, a_sizes[1]), edges[2])
y_fractions[-1][1] = calc_progress_end(get_coord(a_cells[3], a_sizes[1]), get_coord(a_cells[3]+1, a_sizes[1]), edges[3])

print(x_fractions)
print(y_fractions)

x_total = reduce(lambda a, x: a + x[1], x_fractions, 0.0)
y_total = reduce(lambda a, y: a + y[1], y_fractions, 0.0)
print(x_total)
print(y_total)

x_probs = list(map(lambda x: (x[0], x[1] / x_total), x_fractions))
y_probs = list(map(lambda y: (y[0], y[1] / y_total), y_fractions))
print(x_probs)
print(y_probs)





