#include <stdio.h>
#include <string.h>
int main(void){


    long int desk1, desk2, limit;
    scanf("%ld%ld%ld", &desk1, &desk2, &limit);
    
    
    //long int now1 = 0, now2 = 0;
    long int sum1 = 0, sum2 = 0, max = 0;
    long int read1[desk1 + 1], read2[desk2 + 1];
    
    for(long int i = 0; i <= desk1; i++){
        if(i != 0){
            scanf("%ld", &read1[i]);
        } else {
            read1[0] = 0;
        }
    }
   
    for(long int i = 0; i <= desk2; i++){
        if(i != 0){
            scanf("%ld", &read2[i]);
        } else {
            read2[i] = 0;
        }
    }
   
    for(long int i = 0; i <= desk1; i++){
        sum1 += read1[i];
        if(sum1 <= limit){
            if(i > max){
                max = i;
            }
        }
        if(sum1 > limit){
            break;
        }
        sum2 = 0;
        for(long int j = 0; j <= desk2; j++){
            sum2 += read2[j];
            if(sum1 + sum2 <= limit){
                if(i + j > max){
                    max = i + j;
                }
            } else {
                break;
            }
        }
        
    }

  
   printf("%ld\n", max);
   
    return 0;
}