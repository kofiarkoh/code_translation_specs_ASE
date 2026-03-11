#include <stdio.h>

int main(){
  int i,n,a,b,p[200005],c[2]={1,0},cc[3]={1,2,3};
  p[0]=0;
  scanf("%d",&n);
  for(i=0;i<n-1;i++){
    scanf("%d",&a);
    scanf("%d",&b);
    p[b-1]=p[a-1]^1;
    c[p[b-1]]++;
  }
  if((c[0]>n*2/3)||(c[1]>n*2/3)){
    printf("-1");
  }else{
  	for(i=0;i<n;i++){
      if((c[p[i]]!=0)&&(cc[p[i]]<=n)){
        printf("%d ",cc[p[i]]);
        cc[p[i]]+=3;
        c[p[i]]--;
      }else{
        printf("%d ",cc[2]);
        cc[2]+=3;
      }
    }
  }
  return(0);
}
