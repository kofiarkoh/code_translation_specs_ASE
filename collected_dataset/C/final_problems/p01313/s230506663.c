#include<stdio.h>
#include<stdlib.h>
#include<math.h>

int main(void)
{
  int a,b,c;
  long long int i,j,k,p1,p2,tmp1,tmp2;
  int n;

  scanf("%d ",&n);

  for(i=0;i<n;i++){
    scanf("%d %d ",&a,&b);

    p1 = pow(10,a);
    p2 = pow(10,b);
    c = 0;
    for(j=p1/10;j<p1;j++){
      for(k=p2/10;k<p2;k++){
	tmp1 = k*(k+1)/2 - j*(j-1)/2;
	tmp2 = j * p2 + k;
	if(tmp1>tmp2){
	  break;
	}else if(tmp1==tmp2){
	   printf("%lld %lld\n",j,k);
	   c++;
	}
      }
    }
    if(c==0)printf("No cats.\n");
  }
  return 0;
}