// 1020 - Age in Days 7/16/20, 8:46:10 AM code:18883974

#include <stdio.h>
 
int main(void) {
 
    int years, months, days;
    scanf("%d", &years);
    months = years % 365;
    years = years / 365;
    days = months % 30;
    months = months / 30;
    printf("%d ano(s)\n%d mes(es)\n%d dia(s)\n", years,months,days);
 
    return 0;
}

