#include <stdio.h>
 
int
main(){
	int i, j;
	int A;
	int B;
    int flg;
    int count = 0;
    int d[100000] = {0};
    int max;
    scanf("%d", &A);
    scanf("%d", &B);
    if (A < B){
   	   max = B / 2 + 1;
    } else {
    	max = A / 2 + 1;
    }
    for (i = 1; i <= max; i ++){

      if(A % i == 0 && B % i == 0){
          flg = 1;
          for(j = 0; j < count; j++){
              if(i % d[j] == 0 && d[j] != 1){
                  flg = 0;
                  break;
              }
              
          }

          if(flg == 1){
              d[count] = i;
              count++;
          }
              

              
       }
          
    }
    
    printf("%d", count);
	return 0;
}