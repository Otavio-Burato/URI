// 1019 - Time Conversion 7/11/20, 8:34:53 PM code:18842051

#include <stdio.h>
 
int main(void) {
    int hours,minutes,seconds;
    double TimeConversion = 0;
 
    scanf("%d",&seconds);
    
    minutes = seconds/60;
		seconds = seconds % 60;
		hours = minutes/60;
		minutes = minutes % 60;

    printf("%d:%d:%d\n",hours,minutes,seconds);
 
    return (0);
}
