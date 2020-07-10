// 1006 - Average 2 4/18/20, 4:34:15 PM code:17877592

#include <stdio.h>

int main() {

	float A, B, C;
	double MEDIA;
	scanf("%f", &A);
	scanf("%f", &B);
	scanf("%f", &C);

	MEDIA = ((A*2)+(B*3)+(C*5))/10;

	printf("MEDIA = %.1lf\n",MEDIA);

	return 0;
}
