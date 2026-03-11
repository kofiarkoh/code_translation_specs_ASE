#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
int main(void){
    int num_attendance;
    int *rate;
    int kinds_color=0;
    int class[9]={0};

    scanf("%d",&num_attendance);
    rate=(int *)malloc(sizeof(int)*num_attendance);
    int i;
    for(i=0;i<num_attendance;i++){
        scanf("%d",&rate[i]);
        if(rate[i]<=399){
            class[0]++;
        }
        else if(rate[i]<=799){
            class[1]++;
        }
        else if(rate[i]<=1199){
            class[2]++;
        }
        else if(rate[i]<=1599){
            class[3]++;
        }
        else if(rate[i]<=1999){
            class[4]++;
        }
        else if(rate[i]<=2399){
            class[5]++;
        }
        else if(rate[i]<=2799){
            class[6]++;
        }
        else if(rate[i]<=3199){
            class[7]++;
        }
        else{
            class[8]++;
        }
    }
    for(i=0;i<8;i++){
        if(class[i]!=0){
            kinds_color++;
        }
    }
    printf("%d ",kinds_color);
    kinds_color+=class[8];
    printf("%d",kinds_color);
    return 0;
    
}