#include <stdio.h>
#include <stdlib.h>

#include "../../target/headers/space_physics_engine.h"

int main (int argc, char const * const argv[])
{
    Point_t a = { .x = 84, .y = 45 };
    Point_t b = { .x = 0, .y = 39 };
    Point_t m = mid_point(&a, &b);
    print_point(&a);
    print_point(&b);
    print_point(&m);

    Simulation_t* sim = new_Simulation(1.0f);
    printf("%p\n", (void *)sim);
    sim_simulate(sim, 1.2f);
    sim_free(sim);
    printf("%p\n", (void *)sim);

    return EXIT_SUCCESS;
}