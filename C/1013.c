// 1013 - The Greatest 4/20/20, 12:42:38 AM code:17892863

#include <stdio.h>
#include <stdlib.h>

int main() {

	int A, B, C, MaiorAB;
	scanf("%d %d %d", &A, &B, &C);

	MaiorAB = (A+B+abs(A-B))/2;
	MaiorAB = (C+MaiorAB+abs(MaiorAB-C))/2;

	printf("%d eh o maior\n", MaiorAB);
	return 0;
}
