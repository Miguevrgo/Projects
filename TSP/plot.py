import sys
import matplotlib.pyplot as plt

def main():
    x_coords = []
    y_coords = []

    for line in sys.stdin:
        try:
            parts = line.strip().split()
            if len(parts) == 2:
                x_coords.append(float(parts[0]))
                y_coords.append(float(parts[1]))
        except ValueError:
            continue

    if not x_coords:
        print("No data received")
        return

    plt.style.use('dark_background')
    plt.figure(figsize=(10, 10))

    plt.plot(x_coords, y_coords, c='#00ff00', linewidth=0.8, alpha=0.7, label='Path')
    plt.scatter(x_coords, y_coords, c='white', s=2, alpha=0.5)

    plt.scatter(x_coords[0], y_coords[0], c='red', s=50, label='Start/End')

    plt.title("TSP Nearest Neighbor Visualization")
    plt.legend()
    plt.grid(True, linestyle='--', alpha=0.2)
    plt.axis('equal')
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    main()
