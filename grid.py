import numpy as np

# change consts to be part of Grid to depend of size 
OBS_CELL_QUANT = 100
OBS_NUM = 10
MYST_CELL_QUANT = 18
MYST_NUM = 2

class Grid:
    def __init__(self, row: int, col: int) -> None:
        self._r = row
        self._c = col
        self.grid = np.zeros((self._r, self._c), dtype=int)
        self._bggrid = np.zeros((self._r, self._c), dtype=int)
    
    def __getitem__(self, position):
        return self.grid[position], self._bggrid[position]

    def __len__(self):
        return len(self.grid), len(self._bggrid)
    
    def __repr__(self): # how to represent grid as unambigously 
        pass

    def populate(self) -> None:
        self._gen_fggrid(), self._gen_bggrid()

    def update(self) -> None:
        if self._is_empty(self._bggrid):
            self._bggrid = np.zeros((self._r, self._c), dtype=int)
            self._gen_bggrid()

        new_fg_row = self._bggrid[-1]
        self._bggrid = np.delete(self._bggrid, len(self._bggrid) - 1, axis=0)
        self.grid = np.insert(self.grid, 0, new_fg_row, axis=0)
        self.grid = np.delete(self.grid, len(self.grid) - 1, axis=0)

    def size(self) -> int:
        return len(self.grid)

    def _gen_fggrid(self):
        obs_coords, myst_coords = self._rand_coords(OBS_NUM, 5), self._rand_coords(MYST_NUM, 10)
        self._gen_obs(self.grid, obs_coords)
        self._gen_myst(self.grid, myst_coords)

    def _gen_bggrid(self):
        obs_coords, myst_coords = self._rand_coords(OBS_NUM, 5), self._rand_coords(MYST_NUM, 10)
        self._gen_obs(self._bggrid, obs_coords)
        self._gen_myst(self._bggrid, myst_coords)

    def _rand_coords(self, n_pairs: int, min_sep) -> list: 
        rng = np.random.default_rng()
        used_coordinates = []
        count_pairs = 0
        
        while count_pairs < n_pairs:
            rand_coordate = (rng.integers(0, self._r), rng.integers(0, self._c))
            if len(used_coordinates) > 1:
                i = 1
                ti = 10
                while i <= len(used_coordinates):
                        if (abs(used_coordinates[i-1][0] - rand_coordate[0]) < min_sep) or abs(used_coordinates[i-1][1] - rand_coordate[1] < min_sep): # min sep based off size of row and col
                            if ti < 1:
                                break
                            rand_coordate = (rng.integers(0, self._r), rng.integers(0, self._c))
                            ti -= 1
                            i = 0
                        i += 1                    
            used_coordinates = list(set(used_coordinates).union({rand_coordate}))
            count_pairs += 1
        
        return used_coordinates

    def _fill_adj_cells(self, grid, env: int, r: int, c: int, num_cells: int) -> None:
        rng = np.random.default_rng()
        generated_cells = 0
        moves = [(0, -1), (0, 1), (-1, 0), (1, 0)] 
        new_r = r
        new_c = c

        while generated_cells < num_cells:
            rng.shuffle(moves)
            new_r += moves[0][0]
            new_c += moves[0][1]

            if 0 <= new_r < self._r and 0 <= new_c < self._c and grid[new_r][new_c] == 0:
                grid[new_r][new_c] = env
                generated_cells += 1

    def _gen_obs(self, grid, coords) -> None: 
        cells_per_block = OBS_CELL_QUANT / OBS_NUM
        
        for coord in coords:
            self._fill_adj_cells(grid, 1, coord[0], coord[1], cells_per_block)

    def _gen_myst(self, grid, coords) -> None:
        cells_per_block = MYST_CELL_QUANT / MYST_NUM

        for coord in coords:
            self._fill_adj_cells(grid, 2, coord[0], coord[1], cells_per_block)
    
    def _valid_path(self) -> bool:
        # check if each row in obs has 4 slot 0's for ship to go through, if a row doesn't, regenerate obs matrix in self.obs_grid 
        return True

    def _is_empty(self, gird) -> bool:
        return len(gird) < 1
    
    
