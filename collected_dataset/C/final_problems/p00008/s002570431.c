#include <stdio.h>

int main(void) {
    int num;
    int r, c, k, m;
    
    while(scanf("%d", &num) != EOF) {
        
        int count = 0;
        
        for(r = 0; r < 10; r++) {
            for(c = 0; c < 10; c++) {
                for(k = 0; k < 10; k++) {
                    for(m = 0; m < 10; m++) {
                        if(r + c + m + k == num) {
                            count++;
                        }
                    }
                }
            }
        }
        
        printf("%d\n", count);
    }
    
    return(0);
}