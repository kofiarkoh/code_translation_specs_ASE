#include <stdio.h>
#include <stdlib.h>

int main() {
    unsigned long i, n, *a, first = 0, second = 0, tmp;

    scanf("%lu", &n);
    a = (unsigned long *)malloc(n * sizeof(unsigned long));
    
    scanf("%lu", &first);
    if (n >= 2) {
        scanf("%lu", &second);
        if (first < second) {
            tmp = first;
            first = second;
            second = tmp;
        }
    }
    if (n >= 3) {
        for (i = 2; i < n; i++) {
            scanf("%lu", &tmp);
            if (first < tmp) {second = first; first = tmp;}
            else if (second < tmp) {second = tmp;}
        }
    }

    if ((first-second)%2 != 0) {puts("first");}
    else {puts("second");}

    free(a);
    
    return 0;
}
