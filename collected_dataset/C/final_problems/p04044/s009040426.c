#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char s[100][101];

int cmpstr(const void * a, const void * b) {
    //const char *x = a, *y = b;
    return strcmp(a, b);
    //return 0;
}

int cmpindices(const void * a, const void * b) {
    const int *x = a, *y = b;
    return strcmp(s[*x], s[*y]);
}

int main() {
    int n, l;

    scanf("%d %d", &n, &l);

    int indice[n];

    //printf("%p %p %p\n", s, s[0], s[n-1]);

    for(int i = 0; i < n; ++i) {
        scanf("%s", s[i]);
        indice[i] = i;
    }

    // ordenar as strings
    //qsort(s, n, sizeof(s[0]), cmpstr);
    qsort(indice, n, sizeof(indice[0]), cmpindices);

    for(int i = 0; i < n; ++i)
        printf("%s", s[indice[i]]);
    printf("\n");










    return 0;
}
