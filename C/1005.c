// 1005 - Average 1 4/17/20, 10:57:57 PM code:17869749

#include <stdio.h>

int main() {

	float A ,B;
	double MEDIA;

	scanf("%f", &A);
	scanf("%f", &B);

	MEDIA = ((A*3.5) + (B*7.5))/11;
	printf("MEDIA = %.5lf\n", MEDIA);

	return 0;
}
