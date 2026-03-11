#include <stdio.h>
#include <string.h>
 
int main(){
  int n,k,i;
  scanf("%d %d",&n,&k);
  int x[100005],y[100005];
  char c[100005];
  for(i=0;i<n;i++){
    scanf("%d %d %s",&x[i],&y[i],&c[i]);
    x[i] = x[i]%(2*k);
    if(c[i]=='W'){
      y[i] = (y[i]+k)%(2*k);
    }else{
      y[i] = y[i]%(2*k);
    }
  }
  int ans=0,last=0,xx,yy;
  int a,b,eee,aaa;
  for(a=0;a<2*k;a++){
    for(b=0;b<2*k;b++){
      for(i=0;i<n;i++){
        xx = x[i]-a;
        if(xx<0){
          xx += 2*k;
        }
        yy = y[i]-b;
        if(yy<0){
          yy += 2*k;
        }
        if((xx <= k-1 && yy <= k-1) || (xx >= k && yy >= k)){
          ans += 1;
        }
        if(ans>last){
          last = ans;
          eee = a;
          aaa = b;
        }
      }
      ans = 0;
    }
  }
  printf("%d\n",last);
  return 0;
}