#include <stdio.h>
int main(void){
    long int n, i;
    scanf("%ld", &n);

    for(i=n-1; i>=1; i--){
        n=n*i;
    }
    printf("%ld\n", n);
    return 0;
}

