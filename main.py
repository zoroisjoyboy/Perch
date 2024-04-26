import pygame
import grid
import ship

# ship + bullets, implemented, behavior with obs and myst, beginning and implement ending, shooting towers - M2
# include more myst features, beatify, refactor ship, enemy ship - M3
# further beatify, add more visual speds, add audio, refactor - M4 
# menu screen, exit screen, refactor, beatify further, add more features - M5  

WINDOW_WIDTH = 1920
WINDOW_HEIGHT = 1080
CELL_SIZE = 20
PADDING = 2

if __name__ == "__main__":
    pygame.init()
    screen = pygame.display.set_mode((WINDOW_WIDTH, WINDOW_HEIGHT))
    clock = pygame.time.Clock()
    running = True

    total_cell_size = CELL_SIZE + PADDING
    pixel_rows = WINDOW_HEIGHT // total_cell_size
    pixel_cols = WINDOW_WIDTH // total_cell_size 
    g = grid.Grid(pixel_rows, pixel_cols)
    s = ship.Ship(WINDOW_WIDTH // 2, WINDOW_HEIGHT - total_cell_size)
    g.populate()

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
        
        screen.fill("white")
        for r in range(g.size()):
            for c in range(len(g.grid[0])):
                x = c * (CELL_SIZE + PADDING) + PADDING
                y = r * (CELL_SIZE + PADDING) + PADDING     
                match g.grid[r][c]:
                    case 1:
                        pygame.draw.rect(screen, "red", (x, y, CELL_SIZE, CELL_SIZE))
                    case 2:
                        pygame.draw.rect(screen, "green", (x, y, CELL_SIZE, CELL_SIZE))
        
        keys = pygame.key.get_pressed()
        if keys[pygame.K_LEFT]:
            s.left(CELL_SIZE, PADDING)
            if s.x <= CELL_SIZE + PADDING:
                s.x = PADDING
        if keys[pygame.K_RIGHT]:
            s.right(CELL_SIZE, PADDING)
            if s.x >= WINDOW_WIDTH:
                s.x = WINDOW_WIDTH - total_cell_size

        pygame.draw.rect(screen, "blue", (s.x, s.y, CELL_SIZE, CELL_SIZE))

        g.update()
        pygame.display.flip()
        clock.tick(60)

    pygame.quit()