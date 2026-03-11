#include <stdio.h>
int main(void){
  int i,n,x,y,z,max,ans;
  char str[1000];

  while(1){
    fgets(str,sizeof(str),stdin);
    sscanf(str,"%d",&n);
    if(n==0) break; 
    max=-1;
    for(i=0;i<n;i++){
      fgets(str,sizeof(str),stdin);
      sscanf(str,"%d %d %d",&z,&x,&y);
      if(x+y>max){
      max=x+y;
      ans=z;
      }
    }
    printf("%d %d\n",ans,max);
  }
  return 0;
}