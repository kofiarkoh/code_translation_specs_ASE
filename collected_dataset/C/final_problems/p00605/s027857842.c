#include<stdio.h>

int main(){
  int n,k,i,j,buff,swi;
  int s[100];
  
  while(1){
    scanf("%d%d",&n,&k);
    if(n==0&&k==0)
      return 0;
    for(i=0;i<k;i++)
      scanf("%d",&s[i]);
    //input the s[i]
    for(i=0;i<n;i++)
      for(j=0;j<k;j++){
	scanf("%d",&buff);
	s[j]-=buff;
      }
    //input the b
    for(i=0;i<k;i++)
      if(s[i]<0){
	printf("No\n");
	buff=-1;
      }
    if(buff!=-1)
      printf("Yes\n");
  }
}