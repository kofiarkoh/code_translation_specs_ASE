#include <stdio.h>
int main()
{
    int N,i;
    long long int  c=1000000000000000000,d=1;
    scanf("%d",&N);
    long long int A[N];
    for(i=0;i<N;i++)
    {
      scanf("%lld",&A[i]);
      if(A[i]==0)
        d=0;
    }
    for(i=0;i<N;i++)
    {
        if(A[N]==0) {
          d=0;
          break;
        }
        else if(c/A[i]<d)  {
          d=-1;
          break;
        }
        else d*=A[i];
    }
    printf("%lld",d);

    return 0;
}
