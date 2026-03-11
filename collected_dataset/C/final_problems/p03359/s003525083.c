#include <stdio.h>
#include <stdlib.h>
int main (void){
    int a;
    int b;
   
   scanf("%d %d",&a,&b);

     int i=1;
    int j=1;
    int count=0;
    if(a>=b){
    while(i<=a){
        if(i<=b){
        count++;
        }
        i++;
    }
    }else{
        while(j<=b){
        if(j<=a){
            count++;
        }
        j++;
        }

    }    
    printf("%d",count);
    return 0;
}