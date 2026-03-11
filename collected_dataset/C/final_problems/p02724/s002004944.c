
#include <stdio.h>

int main() {

    int x, cont_quinhentos, cont_cinco, hap, resto_quinhentos, resto_cinco;
    scanf("%d", &x);

        cont_quinhentos = x / 500;
        resto_quinhentos = x % 500;

        cont_cinco = resto_quinhentos / 5;
        resto_cinco = resto_quinhentos % 5;

    hap = (cont_quinhentos * 1000) + (cont_cinco * 5);
    printf("%d", hap);
    return 0;
}
