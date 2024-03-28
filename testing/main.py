import numpy as np

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

def get_index(c: float, n: int) -> int:
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
ax_start = get_index(bx_start, a.shape[0]) - 1
ax_end = get_index(bx_end, a.shape[0])
ay_start = get_index(by_start, a.shape[1]) - 1
ay_end = get_index(by_end, a.shape[1])

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
    bx_start = get_coord(chosen[0], B_sizes[0]) #x
    bx_end = get_coord(chosen[0] + 1, B_sizes[0])
    by_start = get_coord(chosen[1], B_sizes[1]) #y
    by_end = get_coord(chosen[1] + 1, B_sizes[1])

    ax_start = get_index(bx_start, A_sizes[0]) - 1
    ax_end = get_index(bx_end, A_sizes[0])
    ay_start = get_index(by_start, A_sizes[1]) - 1
    ay_end = get_index(by_end, A_sizes[1])

    return (ax_start, ax_end, ay_start, ay_end)

print(b_cell)
print(f"{b[b_cell]}")
results = get_matched(b_cell, b.shape, a.shape)
print(results)
#print(f"{a[results[0]]}, {a[results[1]]}, {a[results[2]]}, {a[results[3]]}")
#print(f"{a[results[0], results[2]]}, {a[results[1], results[3]]}")

print()
print(f"{get_index(1.0, 2)}")





