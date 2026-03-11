#include <stdio.h>
#include <math.h>

int main(void) {
    int num;
    int hh, mm;
    double tomm;
    int i, diff;
    
    scanf("%d", &num);
    
    for(i = 0; i < num; i++) {
        scanf("%d:%d", &hh, &mm);
        if((hh >= 0 && hh < 12) && (mm >= 0 && mm < 60)) {
            tomm = (5 * hh) +  5 * ((0.5 * mm) / 30);

            diff = fabs(tomm - mm);
        
            if(diff >= 0 && diff < 5)
                puts("alert");
            else if(diff >= 15 && diff <= 30)
                puts("safe");
            else
                puts("warning");
        }
    }
    
    return(0);
}