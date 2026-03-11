#include<stdio.h>

int main(void) {
    int w_num_vertical, n_num_horizontal;
    scanf("%d", &w_num_vertical);
    scanf("%d", &n_num_horizontal);

    int i, a, b, buf;
    int results[] = {1, 2, 3, 4, 5}; 
    for(i = 0; i < n_num_horizontal; i++) {
        scanf("%d,%d", &a, &b);
        buf = results[a - 1]; 
        results[a - 1] = results[b - 1]; 
        results[b - 1] = buf;
    }   

    int j;                                                                                                         
    for(j = 0; j < w_num_vertical; j++) {
        printf("%d\n", results[j]);
    }   

    return 0;
}