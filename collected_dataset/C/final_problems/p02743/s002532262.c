#include <stdio.h>
#include <math.h>
#include <stdlib.h>


int main(void){ 
    long a,b,c;
     double x,y,z;
        scanf("%ld%ld%ld",&a,&b,&c);
    x = sqrt((double)a);
    y = sqrt((double)b);
    z = sqrt((double)c);

    if((x + y) <z){
        printf("Yes\n");
         
    }
    else{
        printf("No\n");
         
    }
    return 0;
} 