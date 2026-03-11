#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int compare_str(const void *a, const void *b){
    return *(const char *)a - *(const char *)b;
}

int main(void){
    char s[10000][11];
    int i, j, n, count = 0;
    scanf("%d", &n);
    for(i=0; i<n; i++){
        scanf("%s", s[i]);
        qsort(s[i], 10, sizeof(char), compare_str);
    }
    for(i=0; i<n-1; i++){
        for(j=i+1; j<n; j++){
            if(strcmp(s[i], s[j]) == 0) count++;
        }
    }
    printf("%d\n", count);
    return 0;
}