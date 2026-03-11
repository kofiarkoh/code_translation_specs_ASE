#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

int a[1234567891];

int main(){

    int n, menor = 0, maior = 0;

    scanf("%d", &n );

    scanf("%d", &a[0]);
    maior = a[0];
    menor = a[0];

    for (int i = 1; i < n; i++) {
        scanf("%d", &a[i]);
        if (a[i] >= maior)
            maior = a[i];
        if (a[i] <= menor)
            menor = a[i];
    }
    
    int tt = maior - menor;

    printf("%d\n", tt );


return 0;
}
