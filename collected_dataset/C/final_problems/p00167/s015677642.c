#include<stdio.h>

int main(){
  int i,j;
  int n;
  int u[500];
  int sum;
  int cnt;

  while(scanf("%d", &n), n!=0){
    for(i=0; i<n; i++){
      scanf("%d", &u[i]);
    }
    cnt=0;

    for(i=n-1; i>0; i--){
      for(j=0; j<i; j++){
	if(u[j]>u[j+1]){
	  sum=u[j];
	  u[j]=u[j+1];
	  u[j+1]=sum;
	  cnt++;
	}
      }
    }
    printf("%d\n", cnt);
  }
  return 0;
}