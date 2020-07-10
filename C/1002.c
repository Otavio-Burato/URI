// 1002 - Area of a Circle 4/17/20, 2:04:35 AM code:17857957

#include <stdio.h>

int main() {

	double A, R;
	const double n = 3.14159;
    scanf("%lf", &R);
	A = n * (R*R);
    printf("A=%.4lf\n", A);

    return 0;
}
