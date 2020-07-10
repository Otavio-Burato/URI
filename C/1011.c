// 1011 - Sphere 4/18/20, 8:28:40 PM code:17879896

#include <stdio.h>

int main() {

	const double PI = 3.14159;
	double R, VOLUME;

	scanf("%lf", &R);

	VOLUME=(4.0/3.0)*PI*(R*R*R);

	printf("VOLUME = %.3lf\n", VOLUME);

	return 0;
}
