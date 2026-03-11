#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

int main(void) {
    int K,i,A,B,C;//red gree bule
    scanf("%d %d %d" ,&A,&B,&C);
    scanf("%d" ,&K);
    for (i=0; i<K; i++) {
        if(B>C)C*=2;
        else if(C<A)C*=2;
        else if(B<A)B*=2;
        if (B>A && C>B){
            printf("Yes");
            return 0;
        }
    }
    printf("No");
    return 0;
}
