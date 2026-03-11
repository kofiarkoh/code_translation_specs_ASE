#include <stdio.h>

int main(void){
     unsigned long int x;
     unsigned long int ans = 0;
     unsigned long int i;
     scanf("%lu",&x);
     for(i=1;i<=(x/2);i++){
         ans++;
     }
     if(x%2 == 0)
         ans--;
     printf("%lu\n",ans);
     return 0;
}