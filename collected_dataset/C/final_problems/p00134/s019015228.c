#include <stdio.h>

int main(void) {
    int num, i, sum = 0, ave;
    
    scanf("%d", &num);

    
    int va[num];
    
    for(i = 0; i < 100000; i++) {
        if(i >= num)
            break;
        
        do {
            scanf("%d", &va[i]);
        }while(va[i] > 1000000);
        
        if(va[i] > 1000000) {
            ;
        } else {
            sum += va[i];
        }
    }
    
    ave = sum / num;
    
    printf("%d\n", ave);
    
    return(0);
}