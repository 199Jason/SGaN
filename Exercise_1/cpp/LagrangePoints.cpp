#include <iostream>
#include <Eigen/Dense>
#include "matplotlibcpp.h"
#include <cmath>

namespace plt = matplotlibcpp;

// Define the function to find L1, L2, and L3 points
double y1(double x, double mu) {
    return x - (1 - mu) / std::pow(x + mu, 2) + mu / std::pow(x - 1 + mu, 2);
}

double y2(double x, double mu) {
    return x - (1 - mu) / std::pow(x + mu, 2) - mu / std::pow(x - 1 + mu, 2);
}

double y3(double x, double mu) {
    return x + (1 - mu) / std::pow(x + mu, 2) + mu / std::pow(x - 1 + mu, 2);
}

int main() {
    // Parameters
    double mu = 3.0359e-6;

    // Find L1, L2, and L3 points
    double L1 = fsolve(y1, 0.5, mu);
    double L2 = fsolve(y2, 1.5, mu);
    double L3 = fsolve(y3, -1.5, mu);

    // Calculate L4 and L5 components
    double L4x = 0.5 - mu;
    double L4y = 0.5 * std::sqrt(3.0);
    double L5x = 0.5 - mu;
    double L5y = -0.5 * std::sqrt(3.0);

    // Print Lagrange points
    std::cout << "L1: [" << L1 << ";0;0]\n";
    std::cout << "L2: [" << L2 << ";0;0]\n";
    std::cout << "L3: [" << L3 << ";0;0]\n";
    std::cout << "L4: [" << L4x << ";" << L4y << ";0]\n";
    std::cout << "L5: [" << L5x << ";" << L5y << ";0]\n";

    // Plot Lagrange points
    plt::plot({L1, L2, L3, L4x, L5x}, {0.0, 0.0, 0.0, L4y, L5y}, "ko");
    plt::annotate("L1", {L1, 0.0});
    plt::annotate("L2", {L2, 0.0});
    plt::annotate("L3", {L3, 0.0});
    plt::annotate("L4", {L4x, L4y});
    plt::annotate("L5", {L5x, L5y});

    // Plot the Sun and Earth
    plt::plot({-mu, 1 - mu}, {0.0, 0.0}, {{"color", "yellow"}, {"marker", "*"}, {"linewidth", 2}});
    plt::annotate("Sun", {-mu, 0.0});
    plt::plot({1 - mu}, {0.0}, {{"color", "blue"}, {"marker", "."}, {"linewidth", 1}});
    plt::annotate("Earth", {1 - mu, 0.1});

    plt::xlabel("$x$ [nondimensional]");
    plt::ylabel("$y$ [nondimensional]");
    plt::title("Lagrange Points");
    plt::grid(true);
    plt::axis("equal");
    plt::show();

    return 0;
}
