#include <stdio.h>  
  
int main()  
{  
    int n,i;  
    int num[10] = {0};  
    if (scanf("%d", &n)==1){  
    do{  
        num[n%10]++;  
      
    }while((n = n/10) != 0 );  }
  
//  for(i = 0; i < 10; i++){  
//      printf("%d ", num[i]);  
//  }  
    for(i = 0; i < 10; i++){  
        if(num[i] >=3){  
            printf("Yes");  
                break;  
        }  
    }  
    if(i == 10){  
        printf("No");  
    }  
    
  
  
    return 0;  
 }