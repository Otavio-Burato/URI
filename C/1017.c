// 1017 - Fuel Spent 4/20/20, 2:26:03 AM code:17893335

#include <stdio.h>

int main() {

	const float KMpL = 12.0;
	int HOURS, KM;
	float FUEL;

	scanf("%d", &HOURS);
	scanf("%d", &KM);

	FUEL=(HOURS*KM)/KMpL;

	printf("%.3f\n",FUEL);

	return 0;
}
