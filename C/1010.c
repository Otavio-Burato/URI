// 1010 - Simple Calculate 4/18/20, 7:00:07 PM code:17879113

#include <stdio.h>

int main() {

	float C1, N1, V1;
	float C2, N2, V2;
	double TOTAL;

	scanf("%f %f %f", &C1, &N1, &V1);
	scanf("%f %f %f", &C2, &N2, &V2);

	TOTAL=((V1*N1)+(V2*N2));

	printf("VALOR A PAGAR: R$ %.2lf\n", TOTAL);

	return 0;
}
