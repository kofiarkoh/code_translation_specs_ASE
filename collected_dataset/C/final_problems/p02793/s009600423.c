#include<stdio.h>
#include<math.h>
int ketasu(long long int n){
  int ans=0;
  while(n!=0){
    n=n/10;
    ans++;
  }
  return ans;
}

long long int gcd(long long int a,long long int b)
{
    if (a < b) {
        a ^= b;
        b ^= a;
        a ^= b;
    }

    return b ? gcd(b, a % b) : a;
}

long long int lcm(long long int a,long long  int b)
{
    return a * b / gcd(a, b);
}

int main(void){
  int n;
  long long int tmp=1;
  long long int opi;
  long long ans=0;
  scanf("%d",&n);
  long long int a[n];
  int i;

  for(i=0;i<n;i++){
    scanf("%lld",&a[i]);
  }
  tmp=lcm(a[0],a[1]);
  for(i=2;i<n;i++){
    if(tmp%a[i]==0){
      continue;
    }
    opi=lcm(a[i],tmp);
    tmp=opi;
  }
  for(i=0;i<n;i++){
    ans=ans+tmp/a[i];
    ans=ans%(1000000000+7);
  }

  printf("%lld",ans);


  return 0;
}
