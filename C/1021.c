// 1021 - Banknotes and Coins 7/17/20, 6:42:22 PM code:18899212

#include <stdio.h>
#include <math.h>
 
int main(void) {
    double number,integer;
	int remainderInt, cents, remainderFloat;
    int hundred,fifty,twenty,ten,five,two;
    int dollar,fiftyCents,twentyFiveCents,tenCents,fiveCents,oneCents; 

    scanf("%lf.2", &number);
	cents = (number * 100)-((int)number * 100);

    hundred = (int)number/100;
    remainderInt = (int)number%100;
    fifty = remainderInt/50;
    remainderInt = remainderInt%50;
    twenty = remainderInt/20;
    remainderInt = remainderInt%20;
    ten = remainderInt/10;
    remainderInt = remainderInt%10;
    five = remainderInt/5;
    remainderInt = remainderInt%5;
    two = remainderInt/2;
    remainderInt = remainderInt%2;
    dollar = remainderInt;
    
    fiftyCents = cents/50;
    remainderFloat = cents%50;
    twentyFiveCents = remainderFloat/25;
    remainderFloat = remainderFloat%25;
    tenCents = remainderFloat/10;
    remainderFloat = remainderFloat%10;
    fiveCents = remainderFloat/5;
	remainderFloat = remainderFloat%5;
    oneCents = remainderFloat;

    printf("NOTAS:\n%d nota(s) de R$ 100.00\n",hundred);
    printf("%d nota(s) de R$ 50.00\n",fifty);
    printf("%d nota(s) de R$ 20.00\n",twenty);
    printf("%d nota(s) de R$ 10.00\n",ten);
    printf("%d nota(s) de R$ 5.00\n",five);
    printf("%d nota(s) de R$ 2.00\n",two);

	printf("MOEDAS:\n%d moeda(s) de R$ 1.00\n",dollar);
	printf("%d moeda(s) de R$ 0.50\n",fiftyCents);
	printf("%d moeda(s) de R$ 0.25\n",twentyFiveCents);
	printf("%d moeda(s) de R$ 0.10\n",tenCents);
	printf("%d moeda(s) de R$ 0.05\n",fiveCents);
	printf("%d moeda(s) de R$ 0.01\n",oneCents);
 
    return 0;
}

