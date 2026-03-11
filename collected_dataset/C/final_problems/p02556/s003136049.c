#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main() {
    int n;
    scanf("%d", &n);
    int x[n];
    int y[n];
    int i;
    for(i = 0; i < n; i ++){
        scanf("%d %d", &x[i], &y[i]);
    }
    int min;
    min = abs(x[0] - 0) + abs(y[0] - 0);
    int max;
    max = 0;
    for(i = 0; i < n;  i  ++){
        if(min > abs(x[i] - 0) + abs(y[i] - 0)){
            min =abs(x[i] - 0) + abs(y[i] - 0);
        }
    }
    for(i = 0; i < n;  i  ++){
        if(max < abs(x[i] - 0) + abs(y[i] - 0)){
            max =abs(x[i] - 0) + abs(y[i] - 0);
        }
    }
    printf("%d", max - min);

    
    
    return 0;
}
