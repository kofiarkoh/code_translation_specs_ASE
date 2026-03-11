#include <stdio.h>
#include <math.h>
int main()
{
    long long int a,b,n;
    scanf("%lli %lli %lli",&a,&b,&n);
    int i;
    long long int c;
    long long int d;
    long long int result;
    for(i=0; i<sqrt(n)+sqrt(b); i++)
    {
        c=(a*n)/b;
        d=n/b;
        d=d*a;
        c=c-d;
        if(i==0||c>result)
        {
            result=c;
        }
        n--;
    }
    printf("%lli",result);
    return 0;
}