#include<stdio.h>
#include<math.h>
int main(){
  long long a,b,c,n,i,ans;
  scanf("%lld",&n);
  while(n--){
    scanf("%lld %lld",&a,&b);
    c=(a*b)-1;
    i=sqrt(c);//printf("!%lld\n",i);
    ans=i*2-1;
    if(i*i==c){
      ans--;
    }
    if(a==b)ans++;
    if(a+1==b||a==b+1)ans--;
    printf("%lld\n",ans);
  }
  //printf("%lld\n",c*2);
  return 0;
}
