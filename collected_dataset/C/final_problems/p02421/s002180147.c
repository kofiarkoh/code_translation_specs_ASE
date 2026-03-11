#include <stdio.h>
int main(void){
  int i,j,n,taro_p=0,hana_p=0;
  char taro[101], hana[101];
  scanf("%d", &n);
  for(i=0;i<n;i++){
    scanf("%s %s", taro, hana);
    for(j=0;;j++){
      if(taro[j] > hana[j]){
        taro_p += 3;
        break;
      }else if(taro[j] < hana[j]){
        hana_p += 3;
        break;
      }else if(taro[j] + hana[j] == 0){
        hana_p++;
        taro_p++;
        break;
      }
    }
  }
  printf("%d %d\n", taro_p , hana_p);
  return 0;
}
  
