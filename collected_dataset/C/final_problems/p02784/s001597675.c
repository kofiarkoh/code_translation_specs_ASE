#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

int main() {

    int h, n, a;
    scanf("%d %d", &h, &n);

    int mysum = 0;

    for (int i = 0; i < n; i++) {
        scanf("%d", &a);
        mysum += a;
        if(mysum>=h){
            printf("Yes");
            return 0;
        }
    }

    printf("No");

    
    return 0;

}