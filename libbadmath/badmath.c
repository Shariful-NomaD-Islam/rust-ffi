#include <stdio.h>

float bad_add(float v1, float v2, int *v3)
{
    printf("Add %f and %f\n", v1, v2);
    // return v1 + v2;
    return *v3;
}

float change_val(int *v1)
{
    printf("Current value %d\n", *v1);
    *v1 = *v1 + 10;
    return *v1;
}