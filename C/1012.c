// 1012 - Area 4/18/20, 9:13:22 PM code:17880311

#include <stdio.h>

int main() {

	const float PI=3.14159;
	double A, B, C;
	scanf("%lf %lf %lf", &A, &B, &C);

	printf("TRIANGULO: %.3lf\n", (A*C)/2);
	printf("CIRCULO: %.3lf\n", PI*(C*C));
	printf("TRAPEZIO: %.3lf\n", ((A+B)*C)/2);
	printf("QUADRADO: %.3lf\n", B*B);
	printf("RETANGULO: %.3lf\n", A*B);

	return 0;
}
