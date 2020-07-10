// 1008 - Salary 4/18/20, 5:11:22 PM code:17877941

#include <stdio.h>

int main() {

	int HOURS, NUMBER;
	float WORKED;
	float SALARY;

	scanf("%d", &NUMBER);
	scanf("%d", &HOURS);
	scanf("%f", &WORKED);

	SALARY=(HOURS*WORKED);

	printf("NUMBER = %d\n", NUMBER);
	printf("SALARY = U$ %.2f\n", SALARY);

	return 0;
}
