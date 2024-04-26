import pygame 

class Ship:
    def __init__(self, x, y) -> None:
        self.x = x
        self.y = y
        self.health = 100
        self.ammo = 50
        self.name = "Ship"
    
    def name(self, name: str) -> None:
        self._name = name
    
    def left(self, cell_size: int, padding: int) -> None:
        self.x -= (cell_size + padding)        
    
    def right(self, cell_size: int, padding: int) -> None:
        self.x += (cell_size + padding)         
    
    def shoot(self) -> None:
        self.ammo = max(self.ammo - 1, 0)

    def health(self) -> None:
        return self.heath

    
    