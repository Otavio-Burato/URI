// 1015 - Distance Between Two Points 4/20/20, 1:20:43 AM code:17893012

#include <stdio.h>
#include <math.h>

int main() {

	double x1,x2,y1,y2;
	double DISTANCE;

	scanf("%lf %lf", &x1,&y1);
	scanf("%lf %lf", &x2,&y2);

	DISTANCE=sqrt(pow(x2-x1,2)+pow(y2-y1,2));

	printf("%.4lf\n",DISTANCE);

	return 0;
}
