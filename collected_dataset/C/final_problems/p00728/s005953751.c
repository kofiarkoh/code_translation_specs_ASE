#include<stdio.h>

int main(){
  int n,s[1000],i,j,num,k;

  while(1){
    num = 0;
    scanf("%d",&n);
    if(n == 0){
      break;
    }
    for(i = 0 ; i < n ; i++){
      scanf("%d",&s[i]);
    }
    for(i = 0 ; i < n ; i++){
      for(j = i ; j < n ; j++){
	if(s[i] > s[j]){
	  k = s[i];
	  s[i] = s[j];
	  s[j] = k;
	}
      }
    }
    for(i = 1 ; i < n - 1 ; i++){
      num += s[i];
    }
    num /= (n-2);
    printf("%d\n",num);
  }
  return 0;
}