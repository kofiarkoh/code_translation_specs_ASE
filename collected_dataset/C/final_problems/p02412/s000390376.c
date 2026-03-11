#include <stdio.h>
int main(void){
    int i, j, k;
    int n, x;
    
    while(1) {
        scanf("%d %d\n", &n, &x);
        if (n == 0 && x == 0) break;
        
        int count=0;
        for(i=1; i<n-1; i++) {
            for(j=i+1; j<n; j++) {
                for(k=j+1; k<n+1; k++) {
                    if (i+j+k == x) count++;
                }
            }
        }
        
        printf("%d\n", count);
    }
    return 0;
}
