// 1014 - Consumption 4/20/20, 12:54:39 AM code:17892905

#include <stdio.h>

int main() {

	int DISTANCE;
	float SPENT, TOTAL;	

	scanf("%d", &DISTANCE);
	scanf("%f", &SPENT);

	TOTAL=(DISTANCE/SPENT);

	printf("%.3f km/l\n", TOTAL);

	return 0;
}
