#include<stdio.h>
int main(){
  long int n,f=1,pn=0,mn=0,p=0,m=0,i,t;
  scanf("%ld",&n);
  for(i=0;i<n;i++){
    scanf("%ld",&t);
    pn+=t;mn+=t;
    if(pn*f<=0){p+=(pn*f*-1)+1;pn=f;}
    if(mn*f>=0){m+=(mn*f)+1;mn=f*-1;}
    f*=-1;
  }
  printf("%ld",p<m?p:m);
}
