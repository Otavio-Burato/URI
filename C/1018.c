// 1018 - Banknotes 4/23/20, 1:34:25 AM code:17932356

#include<stdio.h>

int main(){

	int NUMBER, HUNDRED, FIFTY, TWENTY, TEN, FIVE, TWO, ONE;

	scanf("%d", &NUMBER);
	printf("%d\n", NUMBER);
	HUNDRED = NUMBER/100;
	FIFTY = NUMBER%100;
	TWENTY = FIFTY%50;
	TEN = TWENTY%20;
	FIVE = TEN%10;
	TWO = FIVE%5;
	ONE = TWO%2;

	printf("%d nota(s) de R$ 100,00\n",HUNDRED);
	printf("%d nota(s) de R$ 50,00\n",FIFTY/50);
	printf("%d nota(s) de R$ 20,00\n",TWENTY/20);
	printf("%d nota(s) de R$ 10,00\n",TEN/10);
	printf("%d nota(s) de R$ 5,00\n",FIVE/5);
	printf("%d nota(s) de R$ 2,00\n",TWO/2);
	printf("%d nota(s) de R$ 1,00\n",ONE);

	return 0;
}
