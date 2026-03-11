#include <stdio.h>
int main(void){

    int D,L,N,sum;
    
    scanf ("%d",&D);
    scanf ("%d",&L);

    printf ("%d\n",(D/L) + (D%L));

    return 0;
}