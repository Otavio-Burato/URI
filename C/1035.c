// 1035 - Selection Test 1 7/17/20, 8:46:12 PM code:18900191

#include <stdio.h>
 
int main(void) {

    int A,B,C,D;
    scanf("%d %d %d %d",&A,&B,&C,&D);
		
    if(B>C&&D>A&&(C+D)>(A+B)&&C>0&&D>0&&(A%2)==0)
        puts("Valores aceitos");
    else
        puts("Valores nao aceitos");

    return 0;
}

