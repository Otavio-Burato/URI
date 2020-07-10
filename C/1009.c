// 1009 - Salary with Bonus 4/18/20, 6:16:25 PM code:17878661

#include <stdio.h>

int main() {

	char NAME[] = "";
	float SALARY, SOLD;
	float TOTAL;

	scanf("%s", NAME);
	scanf("%f", &SALARY);
	scanf("%f", &SOLD);

	TOTAL=SALARY+(SOLD*0.15f);

	printf("TOTAL = R$ %.2f\n", TOTAL);

	return 0;
}
