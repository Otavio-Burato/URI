// 1036 - Bhaskara's Formula 7/17/20, 10:39:07 PM code:18900933

#include <stdio.h>
#include <math.h>
 
int main(void) {
 
    double A,B,C,R1,R2,squareRoot;

    scanf("%lf %lf %lf",&A,&B,&C);
    squareRoot = pow(B,2)-(4*A*C);

    if(squareRoot < 0 || A == 0){
        puts("Impossivel calcular");
        return 0;
    }

    R1 = ((-B)+sqrt(squareRoot))/(2*A);
    R2 = ((-B)-sqrt(squareRoot))/(2*A);
    
    printf("R1 = %.5lf\n",R1);
    printf("R2 = %.5lf\n",R2);
 
    return 0;
}

