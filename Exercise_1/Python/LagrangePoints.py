import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import fsolve

def lagrange_points(mu):
    # Find L1
    def y1(x):
        return x - (1 - mu) / (x + mu)**2 + mu / (x - 1 + mu)**2
    L1 = fsolve(y1, 0.5)[0]
    
    # Find L2
    def y2(x):
        return x - (1 - mu) / (x + mu)**2 - mu / (x - 1 + mu)**2
    L2 = fsolve(y2, 1.5)[0]
    
    # Find L3
    def y3(x):
        return x + (1 - mu) / (x + mu)**2 + mu / (x - 1 + mu)**2
    L3 = fsolve(y3, -1.5)[0]
    
    # Find components of L4 and L5
    L4x, L4y = 0.5 - mu, 0.5 * np.sqrt(3)
    L5x, L5y = 0.5 - mu, -0.5 * np.sqrt(3)
    
    return {'L1': (L1, 0, 0), 'L2': (L2, 0, 0), 'L3': (L3, 0, 0), 'L4': (L4x, L4y, 0), 'L5': (L5x, L5y, 0)}

# Parameters
mu = 3.0359e-6

# Calculate Lagrange points
LP = lagrange_points(mu)

for point, (x, y, z) in LP.items():
    print(f"{point}: [{x};{y};{z}]")

# Plot Lagrange points
plt.figure()
for point, (x, y, z) in LP.items():
    plt.plot(x, y, 'ko')
    plt.text(x, y-0.05, point, verticalalignment='bottom' if point in ['L4', 'L5'] else 'top', horizontalalignment='center')

# Plot the Sun and Earth
plt.plot(-mu, 0, 'y*', linewidth=2)
plt.text(-mu, 0, 'Sun', verticalalignment='top', horizontalalignment='center')
plt.plot(1 - mu, 0, 'b.', linewidth=1)
plt.text(1 - mu, 0.1, 'Earth', verticalalignment='top', horizontalalignment='center')

plt.xlabel(r'$x$ [nondimensional]')
plt.ylabel(r'$y$ [nondimensional]')
plt.title('Lagrange Points')
plt.grid(True)
plt.axis('equal')
plt.show()
